use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, TranslateMessage, MSG};
use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
use windows::Win32::System::Registry::{RegCreateKeyExW, RegSetValueExW, RegDeleteValueW, HKEY_CURRENT_USER, KEY_WRITE, REG_SZ};
use windows::Win32::Foundation::NO_ERROR;
use windows::core::PCWSTR;
use tray_icon::{TrayIconBuilder, Icon, menu::Menu, menu::MenuItem};
use std::time::Duration;

fn main() {
    println!("Starting Windows message loop...");

    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

fn get_ram_usage() -> Result<(u64, u64, u64), String> {
    unsafe {
        let mut memory_status = MEMORYSTATUSEX::default();
        memory_status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

        if GlobalMemoryStatusEx(&mut memory_status).is_ok() {
            let total = memory_status.ullTotalPhys;
            let free = memory_status.ullAvailPhys;
            let used = total - free;
            Ok((used, free, total))
        } else {
            Err("Failed to retrieve memory status".to_string())
        }
    }
}

fn create_tray_icon() {
    let tray_menu = Menu::new();
    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("RAM Monitor")
        .build()
        .expect("Failed to create tray icon");
}

fn display_ram_usage() {
    let icon = Icon::from_rgba(vec![0; 256], 16, 16)
        .expect("Failed to create icon");
    let tray_icon = TrayIconBuilder::new()
        .with_icon(icon.clone())
        .with_tooltip("RAM Monitor")
        .build()
        .expect("Failed to create tray icon");

    loop {
        match get_ram_usage() {
            Ok((used, _, total)) => {
                let usage_percentage = (used as f64 / total as f64 * 100.0) as u8;
                let tooltip = format!("RAM Usage: {}%", usage_percentage);
                tray_icon.set_tooltip(Some(&tooltip)).expect("Failed to update tooltip");
            }
            Err(_) => {
                tray_icon.set_tooltip(Some("RAM data unavailable")).expect("Failed to update tooltip");
            }
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn update_tooltip_with_ram_stats() {
    let icon = Icon::from_rgba(vec![0; 256], 16, 16).expect("Failed to create icon");
    let tray_icon = TrayIconBuilder::new()
        .with_icon(icon.clone())
        .with_tooltip("RAM Monitor")
        .build()
        .expect("Failed to create tray icon");

    loop {
        match get_ram_usage() {
            Ok((used, free, total)) => {
                let tooltip = format!(
                    "Used: {:.2} GB\nFree: {:.2} GB\nTotal: {:.2} GB",
                    used as f64 / 1_073_741_824.0,
                    free as f64 / 1_073_741_824.0,
                    total as f64 / 1_073_741_824.0
                );
                tray_icon.set_tooltip(Some(&tooltip)).expect("Failed to update tooltip");
            }
            Err(_) => {
                tray_icon.set_tooltip(Some("RAM data unavailable")).expect("Failed to update tooltip");
            }
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn add_context_menu() {
    let icon = Icon::from_rgba(vec![0; 256], 16, 16).expect("Failed to create icon");
    let menu = Menu::new();
    let exit_item = MenuItem::new("Exit", true, None); // Enabled and no accelerator
    let _ = menu.append(&exit_item);

    let _tray_icon = TrayIconBuilder::new()
        .with_icon(icon.clone())
        .with_menu(Box::new(menu))
        .with_tooltip("RAM Monitor")
        .build()
        .expect("Failed to create tray icon with context menu");
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
