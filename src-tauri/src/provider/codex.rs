use super::AIProvider;
use std::process::Command;

pub struct CodexAdapter {
    cli_path: String,
}

impl CodexAdapter {
    pub fn new() -> Self {
        Self {
            cli_path: "codex-cli".to_string(), // 실제 CLI 명령어 (예: codex-cli, openai-codex 등)
        }
    }
}

impl AIProvider for CodexAdapter {
    fn check_installed(&self) -> bool {
        // 'codex-cli --version' 등의 명령어로 설치 여부 확인
        Command::new(&self.cli_path)
            .arg("--version")
            .output()
            .is_ok()
    }

    fn check_login_status(&self, _app: &tauri::AppHandle) -> String {
        // 'codex-cli status' 등을 실행하여 로그인 여부 판단
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
        "OpenAI Codex CLI".to_string()
    }

    fn send_prompt(&self, prompt: &str, _app: &tauri::AppHandle) -> Result<String, String> {
        match Command::new(&self.cli_path)
            .arg("prompt")
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
            Err(e) => Err(format!("Failed to execute Codex CLI: {}", e)),
        }
    }
}
