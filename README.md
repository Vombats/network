# Network Info Tool

A simple Rust application that displays your network information in GUI popup dialogs using Zenity. The tool shows your public IP address and internet download speed in an easy-to-read format.

## Features

- 🌐 **Public IP Detection**: Retrieves your current public IP address using `ident.me`
- 🚀 **Speed Test**: Measures your internet download speed using `speedtest-cli`
- 💻 **GUI Interface**: Clean popup dialogs using Zenity for user-friendly display
- ⚡ **Loading Indicator**: Shows a loading popup while gathering network information
- 🔧 **Easy Deployment**: Simple build script for desktop deployment

## Prerequisites

Before running this application, make sure you have the following dependencies installed:

### System Dependencies

- **Zenity**: For GUI popup dialogs

  ```bash
  # Ubuntu/Debian
  sudo apt install zenity
  
  # Fedora/RHEL
  sudo dnf install zenity
  
  # Arch Linux
  sudo pacman -S zenity
  ```

- **curl**: For IP address detection

  ```bash
  # Usually pre-installed on most Linux distributions
  # Ubuntu/Debian
  sudo apt install curl
  ```

- **speedtest-cli**: For internet speed testing

  ```bash
  # Ubuntu/Debian
  sudo apt install speedtest-cli
  
  # Fedora/RHEL
  sudo dnf install speedtest-cli
  
  # Arch Linux
  sudo pacman -S speedtest-cli
  
  # Alternative: Install via pip
  pip install speedtest-cli
  ```

### Rust Environment

- **Rust**: Make sure you have Rust installed (edition 2024)

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source ~/.cargo/env
  ```

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/Vombats/network.git
   cd network
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. (Optional) Use the provided deployment script:

   ```bash
   chmod +x build_and_deploy.sh
   ./build_and_deploy.sh
   ```

   This will build the project and copy the executable to `~/Desktop/Speed`.

## Usage

### Running from Source

```bash
cargo run
```

### Running the Compiled Binary

```bash
./target/release/network
```

### Running the Deployed Version

If you used the deployment script:

```bash
~/Desktop/Speed
```

## How It Works

1. **Loading Phase**: The application immediately shows a "Wait..." popup to indicate it's gathering information
2. **IP Detection**: Uses `curl` to query `ident.me` for your public IP address
3. **Speed Test**: Runs `speedtest-cli --simple` to measure download speed
4. **Results Display**: Shows both pieces of information in a formatted popup dialog

## Output Example

The application displays information in the following format:

```text
Your IP address: 203.0.113.42

Speed: 85.4 Mbps
```

## Error Handling

The application gracefully handles various error conditions:

- Network connectivity issues
- Missing system dependencies
- Invalid speed test results
- Command execution failures

Error messages are displayed in the same popup format for consistency.

## Project Structure

```text
network/
├── src/
│   └── main.rs          # Main application logic
├── Cargo.toml           # Rust project configuration
├── Cargo.lock           # Dependency lock file
├── build_and_deploy.sh  # Build and deployment script
├── README.md            # Project documentation
└── LICENSE              # MIT license
```

## Building for Distribution

To create a portable executable:

```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

The resulting binary will be located at `target/x86_64-unknown-linux-gnu/release/network`.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Uses [ident.me](https://ident.me) for IP address detection
- Relies on [speedtest-cli](https://github.com/sivel/speedtest-cli) for speed testing
- Built with [Zenity](https://help.gnome.org/users/zenity/) for GUI dialogs

## Troubleshooting

### Common Issues

**"Command not found" errors**: Make sure all system dependencies are installed:

```bash
which zenity curl speedtest-cli
```

**Permission denied**: Make sure the executable has the correct permissions:

```bash
chmod +x target/release/network
```

**Slow speed test**: The speed test may take 10-30 seconds depending on your connection.

**No GUI display**: This application requires a graphical desktop environment with X11 support.
