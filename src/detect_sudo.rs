use anyhow::Result;
use procfs::process::Process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

pub static SUDO_READING_INPUT: AtomicBool = AtomicBool::new(false);

pub fn run() -> thread::JoinHandle<Result<()>> {
    // Spawn a background thread to monitor for sudo processes
    thread::spawn(move || -> Result<()> {
        let mut last_sudo_reading_input_time = None;
        loop {
            let reading_input_raw = match is_sudo_reading_password() {
                Ok(is_reading) => is_reading,
                Err(e) => {
                    println!("Failed to determine sudo status: {e:?}");
                    SUDO_READING_INPUT.store(true, Ordering::Release);
                    println!("All future keystrokes will be hidden out of an abundance of caution");
                    return Err(e);
                }
            };

            if reading_input_raw {
                last_sudo_reading_input_time = Some(Instant::now());
            }

            // Continue hiding keystrokes for 1s after we saw the last instance of sudo
            let reading_debounce = last_sudo_reading_input_time
                .map(|t| t.elapsed() < Duration::from_secs(1))
                .unwrap_or_default();

            let reading_input = reading_debounce | reading_input_raw;

            SUDO_READING_INPUT.store(reading_input, Ordering::Release);

            thread::sleep(Duration::from_millis(50));
        }
    })
}

fn is_sudo_reading_password() -> Result<bool> {
    for process in find_sudo_processes()? {
        if !has_child_processes(&process)? {
            // An instance of sudo without child processes means it could be reading the password
            return Ok(true);
        }
    }

    Ok(false)
}

fn has_child_processes(process: &Process) -> Result<bool> {
    // Get all tasks (threads) for the process, which includes the main thread
    let tasks = process.tasks()?;

    for task in tasks {
        let task = task?;
        if !task.children().unwrap_or_default().is_empty() {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Finds all running sudo processes
fn find_sudo_processes() -> Result<Vec<Process>> {
    Ok(procfs::process::all_processes()?
        .filter_map(|p| p.ok())
        .filter_map(|p| {
            if let Ok(args) = p.cmdline() {
                if let Some(arg) = args.first() {
                    if arg.contains("sudo") {
                        return Some(p);
                    }
                }
            }

            None
        })
        .collect())
}
