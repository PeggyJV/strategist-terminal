import { invoke } from '@tauri-apps/api/tauri'
import steward from 'steward-proto-ts'

// Get Steward version of all subscribers
async function version() {
    return await invoke('version') 
}

// Submit a request to schedule a contract call to all subscribers
async function schedule(request: steward.ScheduleRequest) {
    return await invoke('schedule', { request: request.serializeBinary() })
}
