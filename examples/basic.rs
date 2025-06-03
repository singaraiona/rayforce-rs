use rayforce::*;
use std::io::Write;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::{mpsc, oneshot};

fn print_prompt() {
    print!("\x1b[32m⚡ \x1b[0m");
    std::io::stdout().flush().unwrap();
}

type EvalRequest = (String, mpsc::Sender<String>);

async fn handle_tcp_client(
    mut stream: tokio::net::TcpStream,
    eval_tx: mpsc::Sender<EvalRequest>,
    mut shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    writer.write_all(b"Connected to Rayforce REPL\n").await?;
    writer.write_all(b"\x1b[32m> \x1b[0m").await?;

    loop {
        tokio::select! {
            _ = &mut shutdown_rx => {
                break;
            }
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        let input = line.trim();
                        if input == "exit" {
                            break;
                        }

                        let (result_tx, mut result_rx) = mpsc::channel(1);
                        eval_tx.send((input.to_string(), result_tx)).await?;

                        if let Some(result) = result_rx.recv().await {
                            writer.write_all(format!("{}\n", result).as_bytes()).await?;
                        }

                        line.clear();
                        writer.write_all(b"\x1b[32m> \x1b[0m").await?;
                    }
                    Err(e) => {
                        eprintln!("Error reading from TCP client: {}", e);
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

async fn tcp_server(
    eval_tx: mpsc::Sender<EvalRequest>,
    mut shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("TCP server listening on 127.0.0.1:8080");

    loop {
        tokio::select! {
            _ = &mut shutdown_rx => {
                break;
            }
            accept_result = listener.accept() => {
                match accept_result {
                    Ok((socket, addr)) => {
                        println!("New connection from: {}", addr);
                        let eval_tx = eval_tx.clone();
                        let (_, shutdown_rx) = oneshot::channel();
                        tokio::spawn(async move {
                            if let Err(e) = handle_tcp_client(socket, eval_tx, shutdown_rx).await {
                                eprintln!("Error handling TCP client: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {}", e);
                    }
                }
            }
        }
    }
    Ok(())
}

async fn stdin_handler(
    eval_tx: mpsc::Sender<EvalRequest>,
    mut shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let stdin_handle = io::stdin();
    let mut reader = BufReader::new(stdin_handle);
    let mut line = String::new();

    print_prompt();

    loop {
        tokio::select! {
            _ = &mut shutdown_rx => {
                break;
            }
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        let input = line.trim();
                        if input == "exit" {
                            break;
                        }

                        let (result_tx, mut result_rx) = mpsc::channel(1);
                        eval_tx.send((input.to_string(), result_tx)).await?;

                        if let Some(result) = result_rx.recv().await {
                            println!("{}", result);
                        }

                        line.clear();
                        print_prompt();
                        std::io::stdout().flush().unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error reading input: {}", e);
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create Rayforce instance in the main thread
    let rayforce = RayforceBuilder::new()
        .with_arg("-r")
        .with_arg("0")
        .build()
        .unwrap();

    // Get Rayforce version
    let version = rayforce.get_version();
    println!("Rayforce version: {}", version);

    // Create channel for evaluation requests
    let (eval_tx, mut eval_rx) = mpsc::channel::<EvalRequest>(32);
    let name = RayObj::from("rayforce-rs");

    // Create shutdown channels
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let (shutdown_tx2, shutdown_rx2) = oneshot::channel();

    // Spawn TCP server task
    let eval_tx_clone = eval_tx.clone();
    let tcp_server_handle = tokio::spawn(async move {
        if let Err(e) = tcp_server(eval_tx_clone, shutdown_rx).await {
            eprintln!("TCP server error: {}", e);
        }
    });

    // Spawn stdin handler task
    let eval_tx_clone = eval_tx.clone();
    let stdin_handle = tokio::spawn(async move {
        if let Err(e) = stdin_handler(eval_tx_clone, shutdown_rx2).await {
            eprintln!("Stdin handler error: {}", e);
        }
    });

    // Main evaluation loop
    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                break;
            }
            Some((input, result_tx)) = eval_rx.recv() => {
                let s = RayObj::from(input);
                let obj = rayforce.eval_obj_str(&s, &name);
                if !obj.is_nil() {
                    let _ = result_tx.send(obj.to_string()).await;
                }
            }
        }
    }

    // Signal all tasks to shut down
    let _ = shutdown_tx.send(());
    let _ = shutdown_tx2.send(());

    // Wait for all tasks to finish
    if let Err(e) = tcp_server_handle.await {
        eprintln!("Error waiting for TCP server: {}", e);
    }
    if let Err(e) = stdin_handle.await {
        eprintln!("Error waiting for stdin handler: {}", e);
    }

    println!("Bye!");
    Ok(())
}
