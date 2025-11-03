# RAM Monitor v1.0.0

A beautiful, lightweight Windows RAM and CPU monitor with an always-on-top glass design overlay.

![RAM Monitor](https://img.shields.io/badge/platform-Windows-blue)
![Version](https://img.shields.io/badge/version-1.0.0-green)
![License](https://img.shields.io/badge/license-MIT-green)

## 📥 Download

**[Download ram-monitor.exe v1.0.0](https://github.com/Ashwinhegde19/ram-monitor/releases/download/v1.0.0/ram-monitor.exe)** 

Or grab it from the [releases](releases/) folder - only **~190KB**!

> **Note:** Currently available for Windows. macOS and Linux support coming soon!

## ✨ Features

### 🎨 **Glass Overlay Design**
- **Always-on-top floating window** with beautiful glass/aero effect
- **Blue-purple gradient** with glowing cyan-gold text
- **Semi-transparent** (78% opacity) - see through to your desktop
- **Real-time updates** every 500ms - never stuck!

### 📊 **Live Stats Display**
- Shows: `7.85/13.9 RAM  25%CPU`
- Format: `Used/Total RAM (GB)  CPU%`
- Updates twice per second for instant feedback

### 🖱️ **Interactive**
- **Left-click and drag** to move anywhere on screen
- Position it wherever you want - top-left, top-right, center, anywhere!
- **Right-click** to close the app
- No taskbar button - stays out of your way

### ⚡ **Lightweight**
- **~190KB** file size
- **<5MB** RAM usage
- **<1%** CPU usage average
- Pure Rust - no dependencies, no bloat

## 🚀 Quick Start

1. **Download** `ram-monitor.exe` from releases
2. **Double-click** to run
3. **Drag** the floating window to your preferred position
4. **Enjoy** live RAM and CPU stats always visible!

**To close:** Right-click the window or press `Ctrl+C` in terminal

## 📸 Screenshot

The window displays as a sleek glass overlay:
```
┌──────────────────────────┐
│  7.85/13.9 RAM  25%CPU  │ ← Glass effect with glow
└──────────────────────────┘
```

## 🛠️ Build from Source

### Requirements
- Rust (latest stable)
- Windows 10/11
- Visual Studio Build Tools (Desktop development with C++)

### Build Steps
```powershell
# Clone the repository
git clone https://github.com/Ashwinhegde19/ram-monitor.git
cd ram-monitor

# Build optimized release binary
cargo build --release

# The exe will be in target/release/ram-monitor.exe
```

### Run
```powershell
# Run the release binary
.\target\release\ram-monitor.exe

# Or run directly with cargo
cargo run --release
```

## ⚙️ Technical Details

- **Language:** Rust (stable)
- **Binary Size:** ~190KB (optimized with LTO and strip)
- **Memory Usage:** <5MB
- **CPU Usage:** <1% average
- **Update Frequency:** 500ms (2x per second)
- **Window Size:** 240x36 pixels
- **Opacity:** 78% (LWA_ALPHA = 200)

**Windows APIs Used:**
- `GlobalMemoryStatusEx` - RAM statistics
- `GetSystemTimes` - CPU usage calculation
- `CreateWindowExW` - Layered window with WS_EX_TOPMOST
- `SetLayeredWindowAttributes` - Glass transparency effect

## ❓ Troubleshooting

**Window doesn't appear?**
- Check top-left corner (10, 10) - it's small!
- Ensure process is running: `Get-Process ram-monitor`
- Try running from PowerShell/CMD to see error messages

**Stats not updating?**
- Give it 1-2 seconds for first CPU calculation
- Window updates every 500ms automatically via WM_TIMER

**Build fails?**
- Install Visual Studio Build Tools with "Desktop development with C++"
- Ensure Rust is latest stable: `rustup update stable`

**Can't move the window?**
- Left-click and **hold**, then drag
- If stuck, restart the app

## 📄 License

MIT License - See [LICENSE](LICENSE) file

Copyright (c) 2025 Ashwinhegde19

## 🤝 Contributing

Contributions welcome! Please:
1. Fork the repo
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

**Guidelines:**
- Keep binary size minimal (<500KB)
- Maintain low memory footprint
- No external dependencies unless absolutely necessary
- Follow Rust best practices

## ⭐ Star This Repo!

If you find this useful, give it a star! ⭐

---

**Made with ❤️ by @Ashwinhegde19**
