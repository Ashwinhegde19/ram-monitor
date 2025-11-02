RAM Monitor (system tray)

A lightweight Windows system-tray RAM monitor written in Rust.

Features
- Shows real-time RAM usage as a tray tooltip
- Updates every second (configurable in code)
- Right-click context menu with Exit (and hooks for additional items)
- Optional auto-start via Windows registry (enable/disable functions present)

Build requirements
- Rust (stable)
- On Windows: Visual Studio Build Tools (Desktop development with C++) so the MSVC linker (link.exe) is available

Build
```powershell
# from repo root
cargo build --release
```

Run (development)
```powershell
cargo run
```

Run (release binary)
```powershell
.\target\release\ram-monitor.exe
```

Smoke (E2E) test (on developer machine)
```powershell
# Start for a short smoke test, confirm it launches then stops
Start-Process .\target\release\ram-monitor.exe -PassThru | Select-Object Id
Start-Sleep -Seconds 2
Get-Process -Name ram-monitor -ErrorAction SilentlyContinue
Stop-Process -Name ram-monitor -Force
```

Notes
- The repository contains helper functions for polling RAM and updating the tray icon. The functions are implemented and can be wired into `main` to change the runtime behavior.
- Auto-start functions write/remove an entry under `HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Run` (requires appropriate permissions).
- This is a small utility intended to minimize resource usage; further optimizations and automated tests can be added.

Troubleshooting
- If cargo build fails with `link.exe` not found, install the Visual Studio Build Tools (Desktop development with C++ workload) or use the MSVC toolchain.

License
- MIT (adjust as needed)
