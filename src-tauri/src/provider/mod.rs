pub mod mock;
pub mod codex;
pub mod gemini;
pub mod webview;

pub trait AIProvider: Send + Sync {
    fn check_installed(&self) -> bool;
    fn check_login_status(&self, app: &tauri::AppHandle) -> String; // e.g. "ready", "logged_out", "error"
    fn get_provider_name(&self) -> String;
    fn send_prompt(&self, prompt: &str, app: &tauri::AppHandle) -> Result<String, String>;
    fn get_webview_info(&self) -> Option<(String, String)> {
        None
    }
}
