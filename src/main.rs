use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
use windows::Win32::System::Threading::GetSystemTimes;
use windows::Win32::Foundation::{FILETIME, HWND, LPARAM, WPARAM, LRESULT, RECT, COLORREF};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, RegisterClassW, ShowWindow,
    GetMessageW, TranslateMessage, DispatchMessageW, PostQuitMessage,
    GetClientRect, SetLayeredWindowAttributes,
    WS_EX_TOPMOST, WS_EX_LAYERED, WS_EX_TOOLWINDOW, LWA_ALPHA,
    WS_POPUP, SW_SHOW, MSG, WNDCLASSW,
    WM_PAINT, WM_DESTROY, WM_LBUTTONDOWN, WM_RBUTTONDOWN, WM_TIMER,
    SetTimer, KillTimer
};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, EndPaint, PAINTSTRUCT, CreateSolidBrush,
    SetTextColor, SetBkMode, TextOutW, TRANSPARENT,
    CreateFontW, SelectObject, DeleteObject, FW_BOLD,
    DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
    DEFAULT_QUALITY, FF_DONTCARE, HBRUSH, HFONT, RedrawWindow, RDW_INVALIDATE
};
use std::sync::Mutex;
use std::time::Duration;
use std::thread;

static LAST_CPU_TIMES: Mutex<Option<(u64, u64)>> = Mutex::new(None);
static STATS_TEXT: Mutex<String> = Mutex::new(String::new());
const TIMER_ID: usize = 1;

unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            
            // Glass/blur effect background with gradient
            let mut rect = RECT::default();
            let _ = GetClientRect(hwnd, &mut rect);
            
            // Create gradient background (blue-purple glass effect)
            let brush: HBRUSH = CreateSolidBrush(COLORREF(0x00463020)); // Dark blue-purple
            windows::Win32::Graphics::Gdi::FillRect(hdc, &rect, brush);
            let _ = DeleteObject(brush.into());
            
            // Add border for glass effect
            let border_brush: HBRUSH = CreateSolidBrush(COLORREF(0x00886644)); // Lighter border
            let mut border_rect = rect;
            border_rect.right = border_rect.left + 2;
            windows::Win32::Graphics::Gdi::FillRect(hdc, &border_rect, border_brush);
            border_rect = rect;
            border_rect.left = border_rect.right - 2;
            windows::Win32::Graphics::Gdi::FillRect(hdc, &border_rect, border_brush);
            border_rect = rect;
            border_rect.bottom = border_rect.top + 2;
            windows::Win32::Graphics::Gdi::FillRect(hdc, &border_rect, border_brush);
            border_rect = rect;
            border_rect.top = border_rect.bottom - 2;
            windows::Win32::Graphics::Gdi::FillRect(hdc, &border_rect, border_brush);
            let _ = DeleteObject(border_brush.into());
            
            // Create bold font for better visibility
            let font: HFONT = CreateFontW(
                18, 0, 0, 0, FW_BOLD.0 as i32, 0, 0, 0,
                DEFAULT_CHARSET, OUT_DEFAULT_PRECIS,
                CLIP_DEFAULT_PRECIS, DEFAULT_QUALITY,
                FF_DONTCARE.0.into(), windows::core::w!("Segoe UI")
            );
            let _ = SelectObject(hdc, font.into());
            
            // Set text properties - bright cyan for glass effect
            SetTextColor(hdc, COLORREF(0x00FFDD88)); // Bright cyan-gold
            SetBkMode(hdc, TRANSPARENT);
            
            // Get stats text
            let stats = STATS_TEXT.lock().unwrap().clone();
            let wide_text: Vec<u16> = stats.encode_utf16().collect();
            
            // Draw text with shadow effect
            SetTextColor(hdc, COLORREF(0x00000000)); // Shadow
            let _ = TextOutW(hdc, 13, 10, &wide_text);
            SetTextColor(hdc, COLORREF(0x00FFDD88)); // Main text
            let _ = TextOutW(hdc, 12, 9, &wide_text);
            
            // Cleanup
            let _ = DeleteObject(font.into());
            let _ = EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        WM_TIMER => {
            // Trigger repaint on timer
            let _ = RedrawWindow(Some(hwnd), None, None, RDW_INVALIDATE);
            LRESULT(0)
        }
        WM_LBUTTONDOWN => {
            // Allow dragging the window
            use windows::Win32::UI::WindowsAndMessaging::{SendMessageW, WM_NCLBUTTONDOWN, HTCAPTION};
            SendMessageW(hwnd, WM_NCLBUTTONDOWN, Some(WPARAM(HTCAPTION as usize)), Some(lparam));
            LRESULT(0)
        }
        WM_RBUTTONDOWN => {
            // Right-click to close
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_DESTROY => {
            let _ = KillTimer(Some(hwnd), TIMER_ID);
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

fn main() {
    println!("Starting RAM Monitor...");
    
    unsafe {
        let class_name = windows::core::w!("RamMonitorClass");
        
        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: windows::Win32::System::LibraryLoader::GetModuleHandleW(None).unwrap().into(),
            lpszClassName: class_name,
            ..Default::default()
        };
        
        RegisterClassW(&wc);
        
        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_LAYERED | WS_EX_TOOLWINDOW,
            class_name,
            windows::core::w!("RAM Monitor"),
            WS_POPUP,
            10, 10, 240, 36,
            None, None,
            Some(wc.hInstance),
            None
        ).unwrap();
        
        // Make window more transparent for glass effect (200 = 78% opaque)
        let _ = SetLayeredWindowAttributes(
            hwnd,
            COLORREF(0),
            200,
            LWA_ALPHA
        );
        
        let _ = ShowWindow(hwnd, SW_SHOW);
        
        // Set up timer for regular repaints (500ms)
        SetTimer(Some(hwnd), TIMER_ID, 500, None);
        
        // Start stats update thread
        thread::spawn(move || {
            loop {
                if let Ok((used, total)) = get_ram_usage() {
                    let cpu = get_cpu_usage();
                    let text = format!("{:.2}/{:.1} RAM  {:.0}%CPU", used, total, cpu);
                    *STATS_TEXT.lock().unwrap() = text;
                }
                thread::sleep(Duration::from_millis(500));
            }
        });
        
        // Message loop
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
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
