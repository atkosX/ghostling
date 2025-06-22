import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  useEffect(() => {
    invoke("set_display_affinity");
  }, []);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }
  function hide() {
    invoke("set_display_affinity");
  }

  return (
    <main data-tauri-drag-region className="container">
      <div className="text-amber-400">vghjhgjhj</div>
      <button onClick={hide}>hide</button>
    </main>
  );
}

export default App;
