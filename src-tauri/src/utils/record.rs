use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use serde::{Deserialize, Serialize};
use hound::{WavWriter, WavSpec};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub is_default: bool,
    pub device_type: String, // "Input" or "Output"
}

pub struct AudioRecorder {
    is_recording: Arc<AtomicBool>,
    selected_device_id: Option<String>,
}

impl AudioRecorder {
    pub fn new() -> Self {
        Self {
            is_recording: Arc::new(AtomicBool::new(false)),
            selected_device_id: None,
        }
    }

    pub fn set_device(&mut self, device_id: String) {
        self.selected_device_id = Some(device_id);
    }

    pub fn get_available_devices() -> Result<Vec<AudioDevice>, Box<dyn std::error::Error + Send + Sync>> {
        // For now, return mock devices. This can be replaced with actual WASAPI device enumeration
        let devices = vec![
            AudioDevice {
                id: "default_output".to_string(),
                name: "Default Output Device".to_string(),
                is_default: true,
                device_type: "Output".to_string(),
            },
            AudioDevice {
                id: "speakers".to_string(),
                name: "Speakers (Realtek Audio)".to_string(),
                is_default: false,
                device_type: "Output".to_string(),
            },
            AudioDevice {
                id: "headphones".to_string(),
                name: "Headphones (USB Audio)".to_string(),
                is_default: false,
                device_type: "Output".to_string(),
            },
            AudioDevice {
                id: "default_input".to_string(),
                name: "Default Input Device".to_string(),
                is_default: true,
                device_type: "Input".to_string(),
            },
            AudioDevice {
                id: "microphone".to_string(),
                name: "Microphone (Built-in)".to_string(),
                is_default: false,
                device_type: "Input".to_string(),
            },
        ];
        
        Ok(devices)
    }

    pub fn start_recording(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.is_recording.load(Ordering::Relaxed) {
            return Err("Recording is already in progress".into());
        }

        let is_recording = Arc::clone(&self.is_recording);
        is_recording.store(true, Ordering::Relaxed);

        let is_recording_clone = Arc::clone(&is_recording);
        let device_id = self.selected_device_id.clone();
        
        thread::spawn(move || {
            if let Err(e) = Self::record_audio(is_recording_clone, device_id) {
                eprintln!("Recording error: {}", e);
            }
        });

        Ok(())
    }

    pub fn stop_recording(&self) {
        self.is_recording.store(false, Ordering::Relaxed);
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::Relaxed)
    }

    fn record_audio(is_recording: Arc<AtomicBool>, device_id: Option<String>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all("recordings")?;
        
        // Generate timestamp for filename
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        
        let device_name = device_id.as_ref()
            .map(|id| id.as_str())
            .unwrap_or("default");
            
        let filename = format!("recordings/output_{}_{}.wav", device_name, timestamp);
        
        // Create WAV file with proper specifications
        let spec = WavSpec {
            channels: 2,           // Stereo
            sample_rate: 44100,    // 44.1 kHz
            bits_per_sample: 16,   // 16-bit
            sample_format: hound::SampleFormat::Int,
        };
        
        let mut writer = WavWriter::create(&filename, spec)?;

        println!("Recording started from device: {}. Output file: {}", 
                device_id.unwrap_or_else(|| "default".to_string()), filename);

        // Generate some dummy audio data for testing
        let mut counter = 0u32;
        let sample_rate = 44100.0;
        let frequency = 440.0; // A4 note
        
        while is_recording.load(Ordering::Relaxed) {
            // Generate samples for a small buffer (about 23ms worth of audio)
            let samples_per_buffer = (sample_rate * 0.023) as usize; // ~1024 samples
            
            for _ in 0..samples_per_buffer {
                // Generate a sine wave for left and right channels
                let time = counter as f32 / sample_rate;
                let amplitude = 0.3; // Reduce volume to prevent clipping
                
                // Left channel - pure sine wave
                let left_sample = (amplitude * (2.0 * std::f32::consts::PI * frequency * time).sin() * 32767.0) as i16;
                
                // Right channel - slightly different frequency for stereo effect
                let right_sample = (amplitude * (2.0 * std::f32::consts::PI * (frequency * 1.01) * time).sin() * 32767.0) as i16;
                
                // Write stereo samples
                writer.write_sample(left_sample)?;
                writer.write_sample(right_sample)?;
                
                counter += 1;
            }
            
            // Sleep for a short duration to simulate real-time recording
            std::thread::sleep(std::time::Duration::from_millis(23));
        }

        // Finalize the WAV file
        writer.finalize()?;
        
        println!("Recording stopped. WAV file saved: {}", filename);
        Ok(())
    }
}
