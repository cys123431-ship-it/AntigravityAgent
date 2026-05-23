use super::AIProvider;
use std::sync::Mutex;

pub struct MockProviderAdapter {
    pub status: Mutex<String>,
}

impl MockProviderAdapter {
    pub fn new() -> Self {
        Self {
            status: Mutex::new("ready".to_string()),
        }
    }

    pub fn set_status(&self, new_status: &str) {
        if let Ok(mut status) = self.status.lock() {
            *status = new_status.to_string();
        }
    }
}

impl AIProvider for MockProviderAdapter {
    fn check_installed(&self) -> bool {
        true
    }

    fn check_login_status(&self, _app: &tauri::AppHandle) -> String {
        if let Ok(status) = self.status.lock() {
            status.clone()
        } else {
            "error".to_string()
        }
    }

    fn get_provider_name(&self) -> String {
        "Mock Provider".to_string()
    }

    fn send_prompt(&self, prompt: &str, _app: &tauri::AppHandle) -> Result<String, String> {
        Ok(format!("Mock response to: {}", prompt))
    }
}
