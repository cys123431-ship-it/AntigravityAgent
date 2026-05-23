use std::sync::Mutex;

pub struct SecretStore {
    session_token: Mutex<Option<String>>,
}

impl SecretStore {
    pub fn new() -> Self {
        Self {
            session_token: Mutex::new(None),
        }
    }

    pub fn set_session_token(&self, token: String) -> Result<(), String> {
        let mut store = self.session_token.lock().map_err(|_| "Mutex error")?;
        *store = Some(token);
        crate::audit::AuditLogger::log_event("secret_store", "Session token updated (masked)");
        Ok(())
    }

    pub fn has_session_token(&self) -> bool {
        if let Ok(store) = self.session_token.lock() {
            store.is_some()
        } else {
            false
        }
    }

    pub fn get_session_token(&self) -> Result<String, String> {
        let store = self.session_token.lock().map_err(|_| "Mutex error")?;
        store.clone().ok_or_else(|| "Token not set".to_string())
    }
}
