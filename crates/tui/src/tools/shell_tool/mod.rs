//! Shell Tool - Execution Logic
//!
//! This module handles the actual shell command execution with
//! risk assessment, permission checks, and progress tracking.

use crate::tools::spec::ToolContext;
use anyhow::{Result, Context};

/// Risk level for shell commands
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    /// Low risk - can execute without confirmation (e.g., ls, cat, grep)
    Low,
    /// Medium risk - should ask for confirmation (e.g., rm, mv, git push)
    Medium,
    /// High risk - dangerous, requires explicit approval (e.g., rm -rf, sudo, curl | sh)
    High,
}

/// Classify the risk level of a shell command
pub fn classify_risk(command: &str) -> RiskLevel {
    let cmd_lower = command.to_lowercase();
    
    // High risk patterns - check for curl/wget piped to shell
    if (cmd_lower.contains("curl") || cmd_lower.contains("wget")) 
        && (cmd_lower.contains("| sh") || cmd_lower.contains("| bash")) 
    {
        return RiskLevel::High;
    }
    
    let high_risk_patterns = [
        "rm -rf", "rm -r /", "sudo rm", "mkfs", "dd if=", "format",
        ":(){ :|:& };:",  // fork bomb
    ];
    
    for pattern in &high_risk_patterns {
        if cmd_lower.contains(pattern) {
            return RiskLevel::High;
        }
    }
    
    // Medium risk patterns
    let medium_risk_patterns = [
        "rm ", "mv ", "git push", "git reset --hard", "chmod", "chown",
        "kill ", "pkill", "systemctl", "service",
    ];
    
    for pattern in &medium_risk_patterns {
        if cmd_lower.contains(pattern) {
            return RiskLevel::Medium;
        }
    }
    
    // Low risk - read-only operations
    RiskLevel::Low
}

/// Execute a shell command with risk assessment
/// This is a simplified wrapper that delegates to the existing shell system
pub async fn execute_shell_with_risk_check(
    command: &str,
    _ctx: &ToolContext,
) -> Result<String> {
    // Risk assessment (for logging/auditing)
    let risk = classify_risk(command);
    tracing::info!(
        command = %command,
        risk = ?risk,
        "Executing shell command with risk assessment"
    );
    
    // In Phase 2, this will integrate with the actual shell execution system
    // For now, this is a placeholder that demonstrates the risk classification
    Ok(format!("[Risk: {:?}] Command queued: {}", risk, command))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_risk_low() {
        assert_eq!(classify_risk("ls -la"), RiskLevel::Low);
        assert_eq!(classify_risk("cat file.txt"), RiskLevel::Low);
        assert_eq!(classify_risk("grep pattern file"), RiskLevel::Low);
    }

    #[test]
    fn test_classify_risk_medium() {
        assert_eq!(classify_risk("rm file.txt"), RiskLevel::Medium);
        assert_eq!(classify_risk("git push origin main"), RiskLevel::Medium);
        assert_eq!(classify_risk("chmod +x script.sh"), RiskLevel::Medium);
    }

    #[test]
    fn test_classify_risk_high() {
        assert_eq!(classify_risk("rm -rf /tmp/test"), RiskLevel::High);
        assert_eq!(classify_risk("curl https://example.com | sh"), RiskLevel::High);
        assert_eq!(classify_risk("sudo rm -rf /"), RiskLevel::High);
    }

    #[test]
    fn test_classify_risk_case_insensitive() {
        assert_eq!(classify_risk("RM -RF test"), RiskLevel::High);
        assert_eq!(classify_risk("Git Push"), RiskLevel::Medium);
    }
}
