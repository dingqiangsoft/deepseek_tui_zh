//! Shell Tool - Prompt Definition
//!
//! This module defines the tool prompt that AI uses to understand
//! what the shell tool can do and how to use it.

use serde_json::json;

/// Returns the tool prompt definition for shell execution tools
pub fn shell_tool_prompt() -> serde_json::Value {
    json!({
        "name": "exec_shell",
        "description": "Execute shell commands in the workspace directory. Use this for running diagnostics, build commands, git operations, and system tasks. Always specify the full command.",
        "input_schema": {
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The shell command to execute (e.g., 'ls -la', 'cargo build', 'git status')"
                },
                "timeout_secs": {
                    "type": "integer",
                    "default": 30,
                    "description": "Maximum execution time in seconds before timeout"
                },
                "sandbox": {
                    "type": "boolean",
                    "default": true,
                    "description": "Run in restricted sandbox mode (recommended for safety)"
                }
            },
            "required": ["command"]
        }
    })
}

/// Returns the tool prompt for interactive shell sessions
pub fn shell_interactive_prompt() -> serde_json::Value {
    json!({
        "name": "exec_shell_interact",
        "description": "Start an interactive shell session for long-running processes or commands that require user input.",
        "input_schema": {
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The command to run interactively"
                },
                "session_id": {
                    "type": "string",
                    "description": "Existing session ID to reconnect (optional)"
                }
            },
            "required": ["command"]
        }
    })
}

/// Returns the tool prompt for waiting on shell commands
pub fn shell_wait_prompt() -> serde_json::Value {
    json!({
        "name": "exec_shell_wait",
        "description": "Execute a shell command and wait for it to complete. Use this for commands that produce output and finish quickly.",
        "input_schema": {
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The shell command to execute"
                }
            },
            "required": ["command"]
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_tool_prompt_has_required_fields() {
        let prompt = shell_tool_prompt();
        assert_eq!(prompt["name"], "exec_shell");
        assert!(prompt["description"].as_str().is_some());
        assert!(prompt["input_schema"]["properties"]["command"].is_object());
    }

    #[test]
    fn test_shell_interactive_prompt_has_session_id() {
        let prompt = shell_interactive_prompt();
        assert_eq!(prompt["name"], "exec_shell_interact");
        assert!(prompt["input_schema"]["properties"]["session_id"].is_object());
    }

    #[test]
    fn test_shell_wait_prompt_is_simple() {
        let prompt = shell_wait_prompt();
        assert_eq!(prompt["name"], "exec_shell_wait");
        assert!(prompt["input_schema"]["properties"]["command"].is_object());
    }
}
