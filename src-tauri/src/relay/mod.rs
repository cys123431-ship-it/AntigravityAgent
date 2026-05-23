use std::sync::Mutex;
use rand::Rng;

pub struct CloudRelayConnector {
    pub status: Mutex<String>,
    pub pairing_code: Mutex<Option<String>>,
}

impl CloudRelayConnector {
    pub fn new() -> Self {
        Self {
            status: Mutex::new("disconnected".to_string()),
            pairing_code: Mutex::new(None),
        }
    }

    pub fn get_status(&self) -> String {
        let status = self.status.lock().unwrap();
        status.clone()
    }

    pub fn generate_pairing_code(&self) -> String {
        let mut code_store = self.pairing_code.lock().unwrap();
        let mut rng = rand::thread_rng();
        let code = format!("{:03}-{:03}", rng.gen_range(100..999), rng.gen_range(100..999));
        *code_store = Some(code.clone());
        
        let mut status = self.status.lock().unwrap();
        *status = "waiting_for_mobile".to_string();
        
        code
    }

    pub fn get_pairing_code(&self) -> Option<String> {
        let code_store = self.pairing_code.lock().unwrap();
        code_store.clone()
    }
}
