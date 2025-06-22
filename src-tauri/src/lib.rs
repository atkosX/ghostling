// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Gong!", name)
}

#[cfg(target_os = "windows")]
#[tauri::command]
fn set_display_affinity(window: tauri::Window) -> Result<(), String> {
    use windows::Win32::UI::WindowsAndMessaging::{
        SetWindowDisplayAffinity, SetWindowLongPtrW, WDA_EXCLUDEFROMCAPTURE,
        WS_EX_TOOLWINDOW, GWL_EXSTYLE,
    };

    // Get the HWND of the Tauri window
    let hwnd = window.hwnd().map_err(|e| {
        let msg = format!("Failed to get HWND: {e}");
        println!("[set_display_affinity] {msg}");
        msg
    })?;

    // Set display affinity (prevents screen capture)
    let result = unsafe { SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE) };

    // Set window styles to hide from Alt+Tab, taskbar, etc.
    let ex_style = (WS_EX_TOOLWINDOW.0) as isize;
    let result2 = unsafe { SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style) };

    let mut errors = Vec::new();

    if result.is_ok() {
        println!("[set_display_affinity] Successfully set WDA_EXCLUDEFROMCAPTURE on window {hwnd:?}");
    } else {
        println!("[set_display_affinity] Failed to set display affinity on window {hwnd:?}");
        errors.push("SetWindowDisplayAffinity failed");
    }

    if result2 != 0 {
        println!("[set_display_affinity] Successfully set WS_EX_TOOLWINDOW on window {hwnd:?}");
    } else {
        println!("[set_display_affinity] Failed to set WS_EX_TOOLWINDOW on window {hwnd:?}");
        errors.push("SetWindowLongPtrW failed");
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![set_display_affinity,greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
