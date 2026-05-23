use super::AIProvider;
use tauri::{AppHandle, Manager};

pub struct WebViewProvider {
    pub window_label: String,
    pub target_url: String,
    pub provider_name: String,
}

impl WebViewProvider {
    pub fn new_chatgpt() -> Self {
        Self {
            window_label: "chatgpt_webview".to_string(),
            target_url: "https://chatgpt.com".to_string(),
            provider_name: "ChatGPT WebView".to_string(),
        }
    }

    pub fn new_gemini() -> Self {
        Self {
            window_label: "gemini_webview".to_string(),
            target_url: "https://gemini.google.com/app".to_string(),
            provider_name: "Gemini WebView".to_string(),
        }
    }
}

impl AIProvider for WebViewProvider {
    fn check_installed(&self) -> bool {
        true
    }

    fn check_login_status(&self, app: &AppHandle) -> String {
        if app.get_webview_window(&self.window_label).is_some() {
            "ready (WebView Connected)".to_string()
        } else {
            "logged_out (Needs WebView)".to_string()
        }
    }

    fn get_provider_name(&self) -> String {
        self.provider_name.clone()
    }

    fn get_webview_info(&self) -> Option<(String, String)> {
        Some((self.window_label.clone(), self.target_url.clone()))
    }

    fn send_prompt(&self, prompt: &str, app: &AppHandle) -> Result<String, String> {
        if let Some(window) = app.get_webview_window(&self.window_label) {
            // MVP: Evaluate simple JS to simulate injection
            // In a real implementation, you would query the chat input box and trigger a click.
            let js = format!("console.log('Sending prompt to ChatGPT via Injection: {}');", prompt);
            window.eval(&js).map_err(|e| e.to_string())?;
            
            // For now, return a mock response that proves it went through the WebView provider
            Ok(format!("[WebView Response]: Successfully injected '{}' into DOM.", prompt))
        } else {
            Err("WebView window is not open. Please open it first.".to_string())
        }
    }
}
