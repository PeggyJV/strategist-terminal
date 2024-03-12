import { invoke } from "@tauri-apps/api/tauri";

// Get Steward version of all subscribers
async function version() {
  return await invoke("version");
}
