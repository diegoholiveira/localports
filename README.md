# localports

A simple command-line tool to list network ports and their associated binaries. Perfect for developers who need to quickly identify what's using a specific port.

## Features

- Lists all listening network ports
- Shows the binary path for each port
- Filters out system processes (focuses on developer-relevant processes)
- Clean table output with port, PID, and binary information

## Installation

### Via Homebrew

```bash
brew install diegoholiveira/localports/localports
```

### From Source

```bash
git clone https://github.com/diegoholiveira/localports.git
cd localports
cargo build --release
./target/release/localports
```

## Usage

Simply run the command:

```bash
localports
```

Example output:

```
+-------------+-------+--------------------------------------------------+
| Port        | PID   | Bin                                              |
+-------------+-------+--------------------------------------------------+
| 3000 (TCP)  | 12345 | ~/dev/my-app/target/debug/my-server              |
| 5432 (TCP)  | 67890 | /usr/local/bin/postgres                          |
| 8080 (TCP)  | 11111 | ~/Projects/web-service/node_modules/.bin/next    |
+-------------+-------+--------------------------------------------------+
```

## Why localports?

Ever been frustrated trying to find what's using port 8080? Instead of running multiple commands like `lsof`, `ps`, and `netstat`, just run `localports` and immediately see what's listening where.

## License

MIT
