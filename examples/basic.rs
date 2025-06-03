use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
enum RayforceError {
    RuntimeCreationFailed,
}

impl std::fmt::Display for RayforceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RayforceError::RuntimeCreationFailed => write!(f, "Failed to create runtime"),
        }
    }
}

impl std::error::Error for RayforceError {}

struct RayforceRuntime {
    runtime: *mut rayforce::runtime_t,
}

// Since runtime_t is a C pointer, we need to manually implement Send and Sync
unsafe impl Send for RayforceRuntime {}
unsafe impl Sync for RayforceRuntime {}

impl RayforceRuntime {
    fn new() -> Result<Self, RayforceError> {
        unsafe {
            // Initialize Rayforce with command line arguments
            let args = vec![
                CString::new("rayforce").unwrap(),
                CString::new("-r").unwrap(),
                CString::new("1").unwrap(),
            ];
            let mut c_args: Vec<*mut c_char> =
                args.iter().map(|arg| arg.as_ptr() as *mut c_char).collect();
            c_args.push(ptr::null_mut());

            println!("Creating runtime...");
            let runtime = rayforce::runtime_create(c_args.len() as i32 - 1, c_args.as_mut_ptr());
            if !runtime.is_null() {
                println!("Runtime created successfully");
                Ok(RayforceRuntime { runtime })
            } else {
                Err(RayforceError::RuntimeCreationFailed)
            }
        }
    }

    fn get_version(&self) -> u8 {
        unsafe { rayforce::version() }
    }

    fn get_arg(&self, key: &str) -> Option<*mut rayforce::obj_t> {
        let key = CString::new(key).unwrap();
        unsafe {
            let value = rayforce::runtime_get_arg(key.as_ptr());
            if value.is_null() {
                None
            } else {
                Some(value)
            }
        }
    }

    fn run(&self) -> i32 {
        unsafe { rayforce::runtime_run() }
    }
}

impl Drop for RayforceRuntime {
    fn drop(&mut self) {
        unsafe {
            println!("\nCleaning up runtime...");
            rayforce::runtime_destroy();
            println!("Runtime cleanup completed");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Rayforce example with Tokio...");

    // Spawn all tasks
    let mut handles = vec![];

    // Rayforce task (runs on local thread)
    handles.push(tokio::spawn(async {
        let runtime = RayforceRuntime::new().unwrap();
        println!("Rayforce runtime initialized in local task");

        // Do Rayforce-specific work
        let version = runtime.get_version();
        println!("Rayforce version: {}", version);

        if let Some(r_value) = runtime.get_arg("repl") {
            unsafe {
                let obj_ref = &*r_value;
                println!(
                    "Value of repl argument: type={}, ptr={:p}",
                    obj_ref.type_, r_value
                );
            }
        }

        // Run the runtime
        println!("\nRunning runtime...");
        let result = runtime.run();
        println!("Runtime run result: {}", result);

        // Runtime will be dropped here
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
