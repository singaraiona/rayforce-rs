use rayforce::*;
use std::io::Write;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::signal;
use tokio::time::{timeout, Duration};

fn print_prompt() {
    print!("\x1b[32m⚡ \x1b[0m");
    std::io::stdout().flush().unwrap();
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

    // Spawn background tasks
    // let mut handles: Vec<JoinHandle<Result<(), io::Error>>> = vec![];

    // Task 1: Some unrelated work
    // handles.push(tokio::spawn(async {
    //     println!("Task 1: Doing some unrelated work...");
    //     sleep(Duration::from_millis(1000)).await;
    //     println!("Task 1: Work completed");
    //     Ok::<(), io::Error>(())
    // }));

    // Task 2: More unrelated work
    // handles.push(tokio::spawn(async {
    //     println!("Task 2: Doing more unrelated work...");
    //     sleep(Duration::from_millis(2000)).await;
    //     println!("Task 2: Work completed");
    //     Ok::<(), io::Error>(())
    // }));

    // Run REPL in the main thread
    let stdin_handle = io::stdin();
    let mut reader = BufReader::new(stdin_handle);
    let mut line = String::new();
    let name = RayObj::from("rayforce-rs");

    print_prompt();

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                break;
            }
            result = timeout(Duration::from_millis(100), reader.read_line(&mut line)) => {
                match result {
                    Ok(Ok(0)) => break, // EOF
                    Ok(Ok(_)) => {
                        let input = line.trim();
                        if input == "exit" {
                            break;
                        }
                        let s = RayObj::from(input);
                        let obj = rayforce.eval_obj_str(&s, &name);
                        if !obj.is_nil() {
                            println!("{}", obj);
                        }
                        line.clear();
                        print_prompt();
                        std::io::stdout().flush().unwrap();
                    }
                    Ok(Err(e)) => {
                        eprintln!("Error reading input: {}", e);
                        break;
                    }
                    Err(_) => {
                        // Timeout, continue polling
                        continue;
                    }
                }
            }
        }
    }

    // Wait for background tasks to complete
    // for handle in handles {
    //     let _ = handle.await?;
    // }

    println!("\nBye!");
    Ok(())
}
