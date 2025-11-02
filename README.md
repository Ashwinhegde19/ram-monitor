# RAM Monitor

A lightweight Windows system-tray RAM and CPU monitor written in Rust.

![RAM Monitor](https://img.shields.io/badge/platform-Windows-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## 📥 Download

**[Download ram-monitor.exe](releases/ram-monitor.exe)** (Latest Release)

Simply download and run - no installation required!

## Features

- ⚡ Shows real-time RAM and CPU usage in system tray tooltip
- 📊 Format: `3.44/16.0 RAM 67%CPU` (Used/Total RAM in GB, CPU percentage)
- 🔄 Updates every second
- 🪶 Minimal resource usage (optimized Rust binary)
- 🚀 Optional auto-start with Windows (functions included)

## Usage

1. Download `ram-monitor.exe` from the releases folder
2. Double-click to run
3. Look for the white square icon in your system tray (near the clock)
4. Hover over the icon to see: **`3.44/16.0 RAM  67%CPU`**
5. To stop: Press `Ctrl+C` in the terminal, or use Task Manager

## Build from Source

### Requirements
- Rust (stable)
- Windows 10/11
- Visual Studio Build Tools (Desktop development with C++)

### Build
```powershell
# Clone the repository
git clone <your-repo-url>
cd ram-monitor

# Build release binary
cargo build --release
```

### Run
```powershell
# Run the compiled binary
.\target\release\ram-monitor.exe

# Or run in development mode
cargo run --release
```

## Technical Details

- **Language:** Rust
- **Size:** ~500KB (optimized with LTO and strip)
- **Memory:** <5MB RAM usage
- **CPU:** Minimal (<1% average)
- **APIs Used:** 
  - `GlobalMemoryStatusEx` for RAM monitoring
  - `GetSystemTimes` for CPU usage calculation
  - Windows Registry API for auto-start feature

## Configuration

To modify update interval, edit `src/main.rs`:
```rust
thread::sleep(Duration::from_secs(1)); // Change from 1 to desired seconds
```

## Auto-Start with Windows

Functions are included in the code to enable/disable auto-start. To use them, uncomment the calls in `main()`:

```rust
// Enable auto-start
enable_auto_start().expect("Failed to enable auto-start");

// Disable auto-start
disable_auto_start().expect("Failed to disable auto-start");
```
## Testing

```powershell
# Smoke test - verify it starts and runs
Start-Process .\target\release\ram-monitor.exe -PassThru | Select-Object Id
Start-Sleep -Seconds 2
Get-Process -Name ram-monitor -ErrorAction SilentlyContinue
Stop-Process -Name ram-monitor -Force
```

## Troubleshooting

**Icon not visible?**
- Check the system tray overflow (click ^ arrow near the clock)
- Ensure the process is running: `Get-Process ram-monitor`

**Build fails with `link.exe` not found?**
- Install Visual Studio Build Tools with "Desktop development with C++" workload

**CPU shows 0%?**
- CPU usage is calculated after the first update cycle (takes ~1-2 seconds)

## License

MIT License - Feel free to use and modify!

## Contributing

Pull requests welcome! Please ensure:
- Code compiles without errors
- Binary size stays under 1MB
- Memory usage remains minimal
