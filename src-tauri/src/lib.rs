pub mod provider;
pub mod state;
pub mod relay;
pub mod store;
pub mod policy;
pub mod approval;
pub mod execution;
pub mod audit;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state::AppState::default())
        .invoke_handler(tauri::generate_handler![
            state::get_emergency_stop_status,
            state::set_emergency_stop_status,
            state::get_provider_status,
            state::get_provider_name,
            state::switch_provider,
            state::open_webview,
            state::set_provider_token,
            state::get_relay_status,
            state::generate_pairing_code,
            state::simulate_relay_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
