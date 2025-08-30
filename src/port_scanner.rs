use std::process::Command;

#[derive(Debug, Clone)]
pub struct PortInfo {
    pub port: u16,
    pub pid: u32,
    pub protocol: String,
}

pub fn scan_ports() -> Result<Vec<PortInfo>, Box<dyn std::error::Error>> {
    // Use lsof to get listening ports with process info
    // lsof -i -P -n | grep LISTEN
    let output = Command::new("lsof").args(["-i", "-P", "-n"]).output()?;

    if !output.status.success() {
        return Err("Failed to run lsof command".into());
    }

    let output_str = String::from_utf8(output.stdout)?;
    let mut port_infos = Vec::new();

    for line in output_str.lines() {
        if line.contains("LISTEN") {
            if let Some(port_info) = parse_lsof_line(line) {
                port_infos.push(port_info);
            }
        }
    }

    Ok(port_infos)
}

fn parse_lsof_line(line: &str) -> Option<PortInfo> {
    // lsof output format (simplified):
    // COMMAND     PID   USER   FD   TYPE             DEVICE SIZE/OFF NODE NAME
    // node      12345   user   20u  IPv4 0x1234567890      0t0  TCP *:8080 (LISTEN)

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 9 {
        return None;
    }

    // Extract PID (second column)
    let pid = parts[1].parse::<u32>().ok()?;

    // Extract port from the NAME column (last column)
    let name_part = parts[parts.len() - 2]; // Second to last, before (LISTEN)

    // Parse format like "*:8080" or "127.0.0.1:8080"
    if let Some(port_str) = name_part.split(':').last() {
        if let Ok(port) = port_str.parse::<u16>() {
            // Determine protocol from TYPE column
            let protocol = if line.contains("TCP") {
                "TCP".to_string()
            } else if line.contains("UDP") {
                "UDP".to_string()
            } else {
                "Unknown".to_string()
            };

            return Some(PortInfo {
                port,
                pid,
                protocol,
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lsof_line() {
        let line = "node      12345   user   20u  IPv4 0x1234567890      0t0  TCP *:8080 (LISTEN)";
        let result = parse_lsof_line(line);

        assert!(result.is_some());
        let port_info = result.unwrap();
        assert_eq!(port_info.port, 8080);
        assert_eq!(port_info.pid, 12345);
        assert_eq!(port_info.protocol, "TCP");
    }

    #[test]
    fn test_parse_lsof_line_with_ip() {
        let line =
            "Python    54321   user   4u  IPv4 0x9876543210      0t0  TCP 127.0.0.1:5000 (LISTEN)";
        let result = parse_lsof_line(line);

        assert!(result.is_some());
        let port_info = result.unwrap();
        assert_eq!(port_info.port, 5000);
        assert_eq!(port_info.pid, 54321);
        assert_eq!(port_info.protocol, "TCP");
    }
}
