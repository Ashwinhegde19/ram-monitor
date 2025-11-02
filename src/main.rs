use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
use windows::Win32::System::Threading::{GetSystemTimes};
use windows::Win32::System::Registry::{RegCreateKeyExW, RegSetValueExW, RegDeleteValueW, HKEY_CURRENT_USER, KEY_WRITE, REG_SZ};
use windows::Win32::Foundation::{NO_ERROR, FILETIME};
use windows::core::PCWSTR;
use tray_icon::{TrayIconBuilder, Icon};
use std::time::Duration;
use std::thread;
use std::sync::Mutex;

static LAST_CPU_TIMES: Mutex<Option<(u64, u64)>> = Mutex::new(None);

fn main() {
    println!("Starting RAM Monitor...");
    
    // Create tray icon and update tooltip with RAM stats every second
    update_tooltip_with_ram_stats();
}

fn get_ram_usage() -> Result<(f64, f64), String> {
    unsafe {
        let mut memory_status = MEMORYSTATUSEX::default();
        memory_status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

        if GlobalMemoryStatusEx(&mut memory_status).is_ok() {
            let total = memory_status.ullTotalPhys as f64 / 1_073_741_824.0; // Convert to GB
            let free = memory_status.ullAvailPhys as f64 / 1_073_741_824.0;
            let used = total - free;
            Ok((used, total))
        } else {
            Err("Failed to retrieve memory status".to_string())
        }
    }
}

fn filetime_to_u64(ft: &FILETIME) -> u64 {
    ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64)
}

fn get_cpu_usage() -> f64 {
    unsafe {
        let mut idle_time = FILETIME::default();
        let mut kernel_time = FILETIME::default();
        let mut user_time = FILETIME::default();

        if GetSystemTimes(Some(&mut idle_time), Some(&mut kernel_time), Some(&mut user_time)).is_ok() {
            let idle = filetime_to_u64(&idle_time);
            let kernel = filetime_to_u64(&kernel_time);
            let user = filetime_to_u64(&user_time);
            let system = kernel + user;

            let mut last_times = LAST_CPU_TIMES.lock().unwrap();
            
            if let Some((last_idle, last_system)) = *last_times {
                let idle_delta = idle.saturating_sub(last_idle) as f64;
                let system_delta = system.saturating_sub(last_system) as f64;
                
                *last_times = Some((idle, system));
                
                if system_delta > 0.0 {
                    return ((system_delta - idle_delta) / system_delta * 100.0).max(0.0).min(100.0);
                }
            } else {
                *last_times = Some((idle, system));
            }
        }
    }
    0.0
}



fn update_tooltip_with_ram_stats() {
    // Create a simple 16x16 icon (RGBA = 4 bytes per pixel, so 16*16*4 = 1024 bytes)
    let icon = Icon::from_rgba(vec![255; 1024], 16, 16).expect("Failed to create icon");
    let tray_icon = TrayIconBuilder::new()
        .with_icon(icon.clone())
        .with_tooltip("RAM Monitor")
        .build()
        .expect("Failed to create tray icon");

    loop {
        match get_ram_usage() {
            Ok((used, total)) => {
                let cpu_usage = get_cpu_usage();
                let tooltip = format!("{:.2}/{:.1} RAM  {:.0}%CPU", used, total, cpu_usage);
                tray_icon.set_tooltip(Some(&tooltip)).expect("Failed to update tooltip");
            }
            Err(_) => {
                tray_icon.set_tooltip(Some("RAM data unavailable")).expect("Failed to update tooltip");
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}



fn enable_auto_start() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let subkey = "Software\\Microsoft\\Windows\\CurrentVersion\\Run\0".encode_utf16().collect::<Vec<u16>>();
        let app_name = "RAMMonitor\0".encode_utf16().collect::<Vec<u16>>();
        let exe_path = std::env::current_exe()?;
        let exe_path_str = format!("{}\0", exe_path.display());
        let exe_path_wide = exe_path_str.encode_utf16().collect::<Vec<u16>>();

        let mut hkey = Default::default();
        let result = RegCreateKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            Some(0),
            None,
            Default::default(),
            KEY_WRITE,
            None,
            &mut hkey,
            None,
        );
        if result != NO_ERROR {
            return Err("Failed to create registry key".into());
        }

        let data = exe_path_wide.iter().flat_map(|&c| c.to_le_bytes()).collect::<Vec<u8>>();
        let result = RegSetValueExW(
            hkey,
            PCWSTR(app_name.as_ptr()),
            Some(0),
            REG_SZ,
            Some(&data),
        );
        if result != NO_ERROR {
            return Err("Failed to set registry value".into());
        }

        Ok(())
    }
}

fn disable_auto_start() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let subkey = "Software\\Microsoft\\Windows\\CurrentVersion\\Run\0".encode_utf16().collect::<Vec<u16>>();
        let app_name = "RAMMonitor\0".encode_utf16().collect::<Vec<u16>>();

        let mut hkey = Default::default();
        let result = RegCreateKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            Some(0),
            None,
            Default::default(),
            KEY_WRITE,
            None,
            &mut hkey,
            None,
        );
        if result != NO_ERROR {
            return Err("Failed to open registry key".into());
        }

        let result = RegDeleteValueW(hkey, PCWSTR(app_name.as_ptr()));
        if result != NO_ERROR {
            return Err("Failed to delete registry value".into());
        }

        Ok(())
    }
}
