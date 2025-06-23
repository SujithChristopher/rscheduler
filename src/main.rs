use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::io::{self, Read};

struct ProcessMonitor {
    executable_path: String,
    should_restart: Arc<AtomicBool>,
}

impl ProcessMonitor {
    fn new(executable_path: String) -> Self {
        let should_restart = Arc::new(AtomicBool::new(true));
        
        // Set up signal handler for graceful shutdown
        let should_restart_clone = Arc::clone(&should_restart);
        ctrlc::set_handler(move || {
            println!("\nReceived interrupt signal. Shutting down...");
            should_restart_clone.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
        
        ProcessMonitor {
            executable_path,
            should_restart,
        }
    }
    
    fn run(&self) {
        println!("Starting process monitor for: {}", self.executable_path);
        
        while self.should_restart.load(Ordering::SeqCst) {
            match self.start_process() {
                Ok(exit_status) => {
                    println!("Process exited with status: {:?}", exit_status);
                    
                    if self.should_stop_monitoring(&exit_status) {
                        println!("Exit condition met. Stopping monitor.");
                        break;
                    }
                    
                    println!("Restarting in 2 seconds...");
                    thread::sleep(Duration::from_secs(2));
                }
                Err(e) => {
                    eprintln!("Error running process: {}", e);
                    thread::sleep(Duration::from_secs(5));
                }
            }
        }
    }
    
    fn start_process(&self) -> io::Result<std::process::ExitStatus> {
        println!("Starting {}...", self.executable_path);
        
        let mut child = Command::new(&self.executable_path)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        
        // Read stdout and stderr
        let stdout_output = self.read_stdout(child.stdout.take());
        let stderr_output = self.read_stderr(child.stderr.take());
        
        // Wait for the process to complete
        let exit_status = child.wait()?;
        
        // Print outputs
        if let Some(stdout) = stdout_output {
            if !stdout.is_empty() {
                println!("STDOUT:\n{}", stdout);
            }
        }
        
        if let Some(stderr) = stderr_output {
            if !stderr.is_empty() {
                println!("STDERR:\n{}", stderr);
            }
        }
        
        Ok(exit_status)
    }
    
    fn read_stdout(&self, mut output: Option<std::process::ChildStdout>) -> Option<String> {
        if let Some(ref mut out) = output {
            let mut buffer = String::new();
            if out.read_to_string(&mut buffer).is_ok() {
                return Some(buffer);
            }
        }
        None
    }
    
    fn read_stderr(&self, mut output: Option<std::process::ChildStderr>) -> Option<String> {
        if let Some(ref mut out) = output {
            let mut buffer = String::new();
            if out.read_to_string(&mut buffer).is_ok() {
                return Some(buffer);
            }
        }
        None
    }
    
    fn should_stop_monitoring(&self, exit_status: &std::process::ExitStatus) -> bool {
        // Only stop monitoring for very specific conditions
        // By default, keep restarting regardless of exit status
        
        // You can customize these conditions based on your needs:
        
        // Example: Stop only for specific "shutdown" exit codes
        if let Some(code) = exit_status.code() {
            match code {
                // Add specific exit codes that should stop the monitor
                // For example: 99 => return true, // Custom shutdown code
                _ => {}
            }
        }
        
        // For exit message checking, you would need to capture and analyze 
        // the stdout/stderr output and check for specific shutdown messages
        // Example messages that could trigger shutdown:
        // - "PERMANENT_SHUTDOWN"
        // - "EXIT_COMPLETE" 
        // - "MONITOR_STOP"
        
        // By default, always restart (return false)
        false
    }
}

fn main() {
    // Hard-coded application path - modify this to your executable path
    let executable_path = r"E:\homer\rscheduler\my_program\my_program.exe"; // Change this to your application path
    
    // Check if executable exists
    if !std::path::Path::new(executable_path).exists() {
        eprintln!("Error: Executable not found: {}", executable_path);
        eprintln!("Please update the executable_path variable in the code");
        std::process::exit(1);
    }
    
    let monitor = ProcessMonitor::new(executable_path.to_string());
    monitor.run();
}

// Add this to your Cargo.toml:
// [dependencies]
// ctrlc = "3.4"