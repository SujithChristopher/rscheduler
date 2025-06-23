# Rust Process Scheduler/Restarter

A simple yet robust process monitor written in Rust that automatically restarts an application if it crashes or exits. This is useful for ensuring that a critical application remains running.

## Features

- **Automatic Restart**: Monitors a specified executable and restarts it if it terminates.
- **Graceful Shutdown**: Listens for a `Ctrl+C` interrupt signal to shut down the monitor cleanly.
- **Output Capturing**: Captures and prints the `stdout` and `stderr` of the monitored process.
- **Configurable**: Easily configured to point to your application's executable.
- **Customizable Exit Conditions**: Logic can be added to control when the application should or should not be restarted.

## Getting Started

### Prerequisites

You need to have Rust and Cargo installed. If you don't have them, you can install them from [rustup.rs](https://rustup.rs/).

### Configuration

1.  Clone this repository or download the source code.
2.  Open `src/main.rs`.
3.  Modify the `executable_path` variable to point to the absolute path of the application you want to monitor.

    ```rust
    // src/main.rs

    fn main() {
        // Hard-coded application path - modify this to your executable path
        let executable_path = r"C:\path\to\your\application.exe"; // <--- CHANGE THIS
        
        // ...
    }
    ```

### Dependencies

This project uses the `ctrlc` crate to handle interrupt signals. It will be automatically downloaded when you build the project. The dependency is listed in `Cargo.toml`:

```toml
[dependencies]
ctrlc = "3.4"
```

### Building

To build the project, navigate to the project's root directory and run:

```bash
cargo build --release
```

The compiled binary will be located at `target/release/rscheduler.exe`.

### Running

You can run the monitor in two ways:

1.  **Using Cargo**:

    ```bash
    cargo run
    ```

2.  **Running the compiled binary**:

    ```bash
    ./target/release/rscheduler
    ```
    (On Windows, use `.\target\release\rscheduler.exe`)

Once running, the monitor will start your specified application. If the application crashes or closes, the monitor will automatically restart it after a short delay.

## How It Works

The `ProcessMonitor` struct is responsible for managing the application. It runs a loop that:
1.  Starts the specified executable as a child process.
2.  Pipes and captures the `stdout` and `stderr` streams.
3.  Waits for the child process to exit.
4.  Checks if it should stop monitoring based on the exit status. By default, it always restarts.
5.  If it should not stop, it waits for a few seconds and restarts the process.

### Graceful Shutdown

To stop the monitor and the application it's managing, press `Ctrl+C` in the terminal where the monitor is running. The monitor will catch the signal, prevent the application from restarting, and exit cleanly.

### Customizing Exit Behavior

You might not want to restart the application in all cases (e.g., if it exits with a specific success code). You can customize this behavior in the `should_stop_monitoring` function in `src/main.rs`.

By default, it always returns `false`, meaning the application is always restarted.

```rust
// src/main.rs

fn should_stop_monitoring(&self, exit_status: &std::process::ExitStatus) -> bool {
    // Example: Stop if the exit code is 99
    if let Some(code) = exit_status.code() {
        if code == 99 {
            return true; // Stop restarting
        }
    }
    
    // By default, always restart
    false
}
```
You can add your own logic here based on exit codes or even by parsing the application's output to look for specific messages. 