use super::AIProvider;
use std::process::Command;

pub struct GeminiAdapter {
    cli_path: String,
}

impl GeminiAdapter {
    pub fn new() -> Self {
        Self {
            cli_path: "gemini-cli".to_string(), // 실제 CLI 명령어
        }
    }
}

impl AIProvider for GeminiAdapter {
    fn check_installed(&self) -> bool {
        Command::new(&self.cli_path)
            .arg("--version")
            .output()
            .is_ok()
    }

    fn check_login_status(&self, _app: &tauri::AppHandle) -> String {
        match Command::new(&self.cli_path).arg("status").output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
                if stdout.contains("logged in") || stdout.contains("ready") {
                    "ready".to_string()
                } else {
                    "logged_out".to_string()
                }
            }
            Err(_) => "missing".to_string(),
        }
    }

    fn get_provider_name(&self) -> String {
        "Gemini CLI (Antigravity)".to_string()
    }

    fn send_prompt(&self, prompt: &str, _app: &tauri::AppHandle) -> Result<String, String> {
        match Command::new(&self.cli_path)
            .arg("ask") // gemini-cli의 프롬프트 전송 명령어 (예: ask)
            .arg(prompt)
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&output.stderr).to_string())
                }
            }
            Err(e) => Err(format!("Failed to execute Gemini CLI: {}", e)),
        }
    }
}
