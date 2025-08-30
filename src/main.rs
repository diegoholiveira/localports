mod known_process;
mod port_scanner;
mod process_info;

use sysinfo::{Pid, System};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct PortEntry {
    #[tabled(rename = "Port")]
    port: String,
    #[tabled(rename = "PID")]
    pid: String,
    #[tabled(rename = "Bin")]
    directory: String,
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let port_infos = port_scanner::scan_ports()?;

    if port_infos.is_empty() {
        println!("No listening ports found.");
        return Ok(());
    }

    let mut system = System::new_all();

    system.refresh_all();

    let mut entries = Vec::new();

    for port_info in port_infos {
        if let Some(process) = system.process(Pid::from_u32(port_info.pid)) {
            let details = process_info::get_process_details(process);

            // Skip system processes
            if known_process::is_system_process(&details.bin_path) {
                continue;
            }

            entries.push(PortEntry {
                port: format!("{} ({})", port_info.port, port_info.protocol),
                pid: format!("{}", port_info.pid),
                directory: details.directory,
            });
        } else {
            // Process not found in system info, use PID as fallback
            entries.push(PortEntry {
                port: format!("{} ({})", port_info.port, port_info.protocol),
                pid: format!("{}", port_info.pid),
                directory: "unknown".to_string(),
            });
        }
    }

    entries.sort_by(|a, b| {
        let port_a: u16 = a
            .port
            .split_whitespace()
            .next()
            .unwrap()
            .parse()
            .unwrap_or(0);
        let port_b: u16 = b
            .port
            .split_whitespace()
            .next()
            .unwrap()
            .parse()
            .unwrap_or(0);
        port_a.cmp(&port_b)
    });

    let table = Table::new(entries);
    println!("{}", table);

    Ok(())
}
