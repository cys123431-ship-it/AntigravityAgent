use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Denied,
    Expired,
}

#[derive(Debug, Clone)]
pub struct ApprovalRequest {
    pub id: String,
    pub original_command: String,
    pub proposed_action: String,
    pub target: String,
    pub risk_level: String,
    pub status: ApprovalStatus,
}

pub struct ApprovalGate {
    requests: Mutex<HashMap<String, ApprovalRequest>>,
}

impl ApprovalGate {
    pub fn new() -> Self {
        Self {
            requests: Mutex::new(HashMap::new()),
        }
    }

    pub fn request_approval(&self, req: ApprovalRequest) -> Result<(), String> {
        let mut reqs = self.requests.lock().map_err(|_| "Mutex error".to_string())?;
        reqs.insert(req.id.clone(), req);
        Ok(())
    }

    pub fn resolve_approval(&self, id: &str, new_status: ApprovalStatus) -> Result<(), String> {
        let mut reqs = self.requests.lock().map_err(|_| "Mutex error".to_string())?;
        if let Some(req) = reqs.get_mut(id) {
            req.status = new_status;
            Ok(())
        } else {
            Err("Request not found".to_string())
        }
    }
}
