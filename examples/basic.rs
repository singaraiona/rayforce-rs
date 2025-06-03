use tokio::time::{sleep, Duration};

use rayforce::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Rayforce example with Tokio...");

    // Spawn all tasks
    let mut handles = vec![];

    // Rayforce task (runs on local thread)
    handles.push(tokio::spawn(async {
        let rayforce = RayforceBuilder::new()
            .with_arg("-r")
            .with_arg("1")
            .build()
            .unwrap();

        // Do Rayforce-specific work
        let version = rayforce.get_version();
        println!("Rayforce version: {}", version);

        // Run the Rayforce
        println!("\nRunning Rayforce...");
        let result = rayforce.run();
        println!("Rayforce run result: {}", result);

        // Rayforce will be dropped here
    }));

    // Task 1: Some unrelated work
    handles.push(tokio::spawn(async {
        println!("Task 1: Doing some unrelated work...");
        sleep(Duration::from_millis(100)).await;
        println!("Task 1: Work completed");
    }));

    // Task 2: More unrelated work
    handles.push(tokio::spawn(async {
        println!("Task 2: Doing more unrelated work...");
        sleep(Duration::from_millis(200)).await;
        println!("Task 2: Work completed");
    }));

    // Wait for all tasks to complete
    for handle in handles {
        handle.await?;
    }

    println!("\nExample completed successfully!");
    Ok(())
}
