use std::sync::Mutex;
use tauri::State;
use crate::provider::{mock::MockProviderAdapter, codex::CodexAdapter, gemini::GeminiAdapter, AIProvider};
use crate::relay::CloudRelayConnector;
use crate::store::{FileScopeManager, secret::SecretStore};

pub struct AppState {
    pub emergency_stop: Mutex<bool>,
    pub provider: Mutex<Box<dyn AIProvider>>,
    pub relay: CloudRelayConnector,
    pub file_scope: FileScopeManager,
    pub secret_store: SecretStore,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            emergency_stop: Mutex::new(false),
            provider: Mutex::new(Box::new(MockProviderAdapter::new())),
            relay: CloudRelayConnector::new(),
            file_scope: FileScopeManager::new(),
            secret_store: SecretStore::new(),
        }
    }
}

#[tauri::command]
pub fn get_emergency_stop_status(state: State<'_, AppState>) -> Result<String, String> {
    let status = state.emergency_stop.lock().map_err(|_| "Mutex lock failed")?;
    if *status {
        Ok("enabled".to_string())
    } else {
        Ok("disabled".to_string())
    }
}

#[tauri::command]
pub fn set_emergency_stop_status(
    status: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut current_status = state.emergency_stop.lock().map_err(|_| "Mutex lock failed")?;
    if status == "enabled" {
        *current_status = true;
    } else if status == "disabled" {
        *current_status = false;
    } else {
        return Err("Invalid status".to_string());
    }
    Ok(())
}

#[tauri::command]
pub fn get_provider_status(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    let provider = state.provider.lock().map_err(|_| "Mutex error")?;
    Ok(provider.check_login_status(&app))
}

#[tauri::command]
pub fn open_webview(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let provider = state.provider.lock().map_err(|_| "Mutex error")?;
    if let Some((label, url)) = provider.get_webview_info() {
        tauri::WebviewWindowBuilder::new(&app, &label, tauri::WebviewUrl::External(url.parse().unwrap()))
            .title(&provider.get_provider_name())
            .build()
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Current provider does not support WebView".to_string())
    }
}

#[tauri::command]
pub fn set_provider_token(token: String, state: State<'_, AppState>) -> Result<(), String> {
    state.secret_store.set_session_token(token)?;
    Ok(())
}

#[tauri::command]
pub fn get_provider_name(state: State<'_, AppState>) -> Result<String, String> {
    let provider = state.provider.lock().map_err(|_| "Mutex error")?;
    Ok(provider.get_provider_name())
}

#[tauri::command]
pub fn switch_provider(provider_type: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut provider = state.provider.lock().map_err(|_| "Mutex error")?;
    match provider_type.as_str() {
        "webview_chatgpt" => *provider = Box::new(crate::provider::webview::WebViewProvider::new_chatgpt()),
        "webview_gemini" => *provider = Box::new(crate::provider::webview::WebViewProvider::new_gemini()),
        "codex" => *provider = Box::new(crate::provider::codex::CodexAdapter::new()),
        "gemini" => *provider = Box::new(crate::provider::gemini::GeminiAdapter::new()),
        _ => *provider = Box::new(crate::provider::mock::MockProviderAdapter::new()),
    }
    Ok(())
}

#[tauri::command]
pub fn get_relay_status(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.relay.get_status())
}

#[tauri::command]
pub fn generate_pairing_code(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.relay.generate_pairing_code())
}

#[tauri::command]
pub fn simulate_relay_message(message: String, _state: State<'_, AppState>) -> Result<String, String> {
    use crate::policy::{PolicyEngine, PolicyAction};
    use crate::audit::AuditLogger;

    AuditLogger::log_event("relay_command_received", &format!("Received from Mobile: {}", message));
    
    // Simulate policy check
    let action = PolicyEngine::evaluate_command(&message);
    
    match action {
        PolicyAction::Safe => Ok("Action is SAFE and would be executed.".to_string()),
        PolicyAction::NeedsApproval => Ok("Action NEEDS APPROVAL. Adding to queue...".to_string()),
        PolicyAction::Blocked => Err("Action is BLOCKED by PolicyEngine.".to_string()),
    }
}
