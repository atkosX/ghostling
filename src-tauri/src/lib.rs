// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod utils;

use std::sync::Mutex;
use utils::record::{AudioRecorder, AudioDevice};

// Global audio recorder instance
static AUDIO_RECORDER: Mutex<Option<AudioRecorder>> = Mutex::new(None);

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Gong!", name)
}

#[tauri::command]
fn get_audio_devices() -> Result<Vec<AudioDevice>, String> {
    AudioRecorder::get_available_devices()
        .map_err(|e| format!("Failed to get audio devices: {}", e))
}

#[tauri::command]
fn set_recording_device(device_id: String) -> Result<String, String> {
    let mut recorder_guard = AUDIO_RECORDER.lock().map_err(|e| format!("Lock error: {}", e))?;
    
    if recorder_guard.is_none() {
        *recorder_guard = Some(AudioRecorder::new());
    }
    
    if let Some(recorder) = recorder_guard.as_mut() {
        if recorder.is_recording() {
            return Err("Cannot change device while recording is in progress".to_string());
        }
        
        recorder.set_device(device_id.clone());
        Ok(format!("Recording device set to: {}", device_id))
    } else {
        Err("Failed to initialize audio recorder".to_string())
    }
}

#[tauri::command]
fn start_recording() -> Result<String, String> {
    let mut recorder_guard = AUDIO_RECORDER.lock().map_err(|e| format!("Lock error: {}", e))?;
    
    if recorder_guard.is_none() {
        *recorder_guard = Some(AudioRecorder::new());
    }
    
    if let Some(recorder) = recorder_guard.as_ref() {
        if recorder.is_recording() {
            return Err("Recording is already in progress".to_string());
        }
        
        recorder.start_recording().map_err(|e| format!("Failed to start recording: {}", e))?;
        Ok("Recording started successfully".to_string())
    } else {
        Err("Failed to initialize audio recorder".to_string())
    }
}

#[tauri::command]
fn stop_recording() -> Result<String, String> {
    let recorder_guard = AUDIO_RECORDER.lock().map_err(|e| format!("Lock error: {}", e))?;
    
    if let Some(recorder) = recorder_guard.as_ref() {
        if !recorder.is_recording() {
            return Err("No recording in progress".to_string());
        }
        
        recorder.stop_recording();
        Ok("Recording stopped successfully".to_string())
    } else {
        Err("Audio recorder not initialized".to_string())
    }
}

#[tauri::command]
fn get_recording_status() -> Result<bool, String> {
    let recorder_guard = AUDIO_RECORDER.lock().map_err(|e| format!("Lock error: {}", e))?;
    
    if let Some(recorder) = recorder_guard.as_ref() {
        Ok(recorder.is_recording())
    } else {
        Ok(false)
    }
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
        .invoke_handler(tauri::generate_handler![
            set_display_affinity,
            greet,
            get_audio_devices,
            set_recording_device,
            start_recording,
            stop_recording,
            get_recording_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
