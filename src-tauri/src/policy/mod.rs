#[derive(Debug, Clone, PartialEq)]
pub enum PolicyAction {
    Safe,
    NeedsApproval,
    Blocked,
}

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn evaluate_command(command: &str) -> PolicyAction {
        let cmd_lower = command.to_lowercase();
        
        // 1. Blocked: 키로깅, 쿠키, 타인 제어, 비밀번호 탈취, 결제 등
        let blocked_keywords = ["keylog", "steal", "cookie", "password", "payment", "transfer"];
        for keyword in blocked_keywords {
            if cmd_lower.contains(keyword) {
                return PolicyAction::Blocked;
            }
        }

        // 2. NeedsApproval: 파일 쓰기, 삭제, 터미널 실행, 앱 종료, 설정 변경 등 파괴적 행동
        let approval_keywords = ["write", "delete", "rm ", "kill", "chmod", "chown", "mv ", "cp "];
        for keyword in approval_keywords {
            if cmd_lower.contains(keyword) {
                return PolicyAction::NeedsApproval;
            }
        }

        // 3. Safe: 읽기, 검색, 요약
        PolicyAction::Safe
    }
}
