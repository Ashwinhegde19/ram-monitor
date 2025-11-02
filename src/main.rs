use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, TranslateMessage, MSG};
use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
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

fn get_ram_usage() -> Option<(u64, u64, u64)> {
    unsafe {
        let mut memory_status = MEMORYSTATUSEX::default();
        memory_status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

        if GlobalMemoryStatusEx(&mut memory_status).is_ok() {
            let total = memory_status.ullTotalPhys;
            let free = memory_status.ullAvailPhys;
            let used = total - free;
            Some((used, free, total))
        } else {
            None
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
        if let Some((used, _, total)) = get_ram_usage() {
            let usage_percentage = (used as f64 / total as f64 * 100.0) as u8;
            let tooltip = format!("RAM Usage: {}%", usage_percentage);
            tray_icon.set_tooltip(Some(&tooltip)).expect("Failed to update tooltip");
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
        if let Some((used, free, total)) = get_ram_usage() {
            let tooltip = format!(
                "Used: {:.2} GB\nFree: {:.2} GB\nTotal: {:.2} GB",
                used as f64 / 1_073_741_824.0,
                free as f64 / 1_073_741_824.0,
                total as f64 / 1_073_741_824.0
            );
            tray_icon.set_tooltip(Some(&tooltip)).expect("Failed to update tooltip");
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
