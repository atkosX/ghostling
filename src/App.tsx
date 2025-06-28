import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface AudioDevice {
  id: string;
  name: string;
  is_default: boolean;
  device_type: string;
}

function App() {
  const [isRecording, setIsRecording] = useState(false);
  const [recordingStatus, setRecordingStatus] = useState("");
  const [audioDevices, setAudioDevices] = useState<AudioDevice[]>([]);
  const [selectedDevice, setSelectedDevice] = useState<string>("");
  const [deviceStatus, setDeviceStatus] = useState("");

  useEffect(() => {
    invoke("set_display_affinity");
    checkRecordingStatus();
    loadAudioDevices();
  }, []);

  async function loadAudioDevices() {
    try {
      const devices = (await invoke("get_audio_devices")) as AudioDevice[];
      setAudioDevices(devices);
      
      // Set default device as selected
      const defaultDevice = devices.find(device => device.is_default && device.device_type === "Output");
      if (defaultDevice) {
        setSelectedDevice(defaultDevice.id);
      }
    } catch (error) {
      console.error("Failed to load audio devices:", error);
      setDeviceStatus(`Error loading devices: ${error}`);
    }
  }

  async function selectDevice(deviceId: string) {
    if (isRecording) {
      setDeviceStatus("Cannot change device while recording");
      return;
    }

    try {
      const result = (await invoke("set_recording_device", { deviceId })) as string;
      setSelectedDevice(deviceId);
      setDeviceStatus(result);
      console.log(result);
    } catch (error) {
      setDeviceStatus(`Error: ${error}`);
      console.error("Failed to set recording device:", error);
    }
  }

  function hide() {
    invoke("set_display_affinity");
  }

  async function checkRecordingStatus() {
    try {
      const status = (await invoke("get_recording_status")) as boolean;
      setIsRecording(status);
    } catch (error) {
      console.error("Failed to check recording status:", error);
    }
  }

  async function startRecording() {
    try {
      const result = (await invoke("start_recording")) as string;
      setRecordingStatus(result);
      setIsRecording(true);
      console.log(result);
    } catch (error) {
      setRecordingStatus(`Error: ${error}`);
      console.error("Failed to start recording:", error);
    }
  }

  async function stopRecording() {
    try {
      const result = (await invoke("stop_recording")) as string;
      setRecordingStatus(result);
      setIsRecording(false);
      console.log(result);
    } catch (error) {
      setRecordingStatus(`Error: ${error}`);
      console.error("Failed to stop recording:", error);
    }
  }

  return (
    <main data-tauri-drag-region className="container">
      <div className="text-amber-400">Audio Recording App</div>

      <div style={{ margin: "20px 0" }}>
        <h2>Audio Device Selection</h2>
        
        <div style={{ margin: "10px 0" }}>
          <label htmlFor="device-select" style={{ marginRight: "10px" }}>
            <strong>Select Recording Device:</strong>
          </label>
          <select
            id="device-select"
            value={selectedDevice}
            onChange={(e) => selectDevice(e.target.value)}
            disabled={isRecording}
            style={{
              padding: "5px 10px",
              borderRadius: "5px",
              border: "1px solid #ccc",
              backgroundColor: isRecording ? "#f0f0f0" : "white",
              cursor: isRecording ? "not-allowed" : "pointer",
              minWidth: "200px"
            }}
          >
            <option value="">Select a device...</option>
            {audioDevices.map((device) => (
              <option key={device.id} value={device.id}>
                {device.name} {device.is_default ? "(Default)" : ""} - {device.device_type}
              </option>
            ))}
          </select>
          
          <button
            onClick={loadAudioDevices}
            style={{
              backgroundColor: "#6c757d",
              color: "white",
              padding: "5px 10px",
              border: "none",
              borderRadius: "5px",
              cursor: "pointer",
              marginLeft: "10px"
            }}
          >
            Refresh Devices
          </button>
        </div>

        {deviceStatus && (
          <div
            style={{
              margin: "10px 0",
              padding: "8px",
              backgroundColor: "#e7f3ff",
              borderRadius: "5px",
              fontSize: "14px"
            }}
          >
            <strong>Device Status: </strong>
            {deviceStatus}
          </div>
        )}

        <h2>Audio Recording Controls</h2>

        <div style={{ margin: "10px 0" }}>
          <button
            onClick={startRecording}
            disabled={isRecording}
            style={{
              backgroundColor: isRecording ? "#ccc" : "#4CAF50",
              color: "white",
              padding: "10px 20px",
              border: "none",
              borderRadius: "5px",
              cursor: isRecording ? "not-allowed" : "pointer",
              marginRight: "10px",
            }}
          >
            {isRecording ? "Recording..." : "Start Recording"}
          </button>

          <button
            onClick={stopRecording}
            disabled={!isRecording}
            style={{
              backgroundColor: !isRecording ? "#ccc" : "#f44336",
              color: "white",
              padding: "10px 20px",
              border: "none",
              borderRadius: "5px",
              cursor: !isRecording ? "not-allowed" : "pointer",
              marginRight: "10px",
            }}
          >
            Stop Recording
          </button>

          <button
            onClick={checkRecordingStatus}
            style={{
              backgroundColor: "#008CBA",
              color: "white",
              padding: "10px 20px",
              border: "none",
              borderRadius: "5px",
              cursor: "pointer",
            }}
          >
            Check Status
          </button>
        </div>

        <div style={{ margin: "10px 0" }}>
          <strong>Status: </strong>
          <span style={{ color: isRecording ? "green" : "red" }}>
            {isRecording ? "Recording" : "Not Recording"}
          </span>
        </div>

        {recordingStatus && (
          <div
            style={{
              margin: "10px 0",
              padding: "10px",
              backgroundColor: "#f0f0f0",
              borderRadius: "5px",
            }}
          >
            <strong>Last Action: </strong>
            {recordingStatus}
          </div>
        )}

        <div style={{ fontSize: "12px", color: "#666", marginTop: "10px" }}>
          Recordings will be saved to the "recordings" folder as .wav files (16-bit, 44.1kHz, Stereo)
        </div>
      </div>

      <button onClick={hide}>Hide Window</button>
    </main>
  );
}

export default App;
