//! Project Context - DEEPSEEK.md Support
//!
//! This module loads project-level AI instructions from `.deepseek/DEEPSEEK.md`
//! to help the AI understand project-specific conventions, coding standards,
//! and workflow preferences.
//!
//! Similar to Claude Code's CLAUDE.md, but adapted for DeepSeek TUI.

use std::path::{Path, PathBuf};
use std::fs;

/// Default filename for project-specific AI instructions
const DEEPSEEK_MD_FILENAME: &str = "DEEPSEEK.md";

/// Load project context from `.deepseek/DEEPSEEK.md`
///
/// Returns the content if the file exists, otherwise returns an empty string.
pub fn load_deepseek_md(workspace: &Path) -> String {
    let deepseek_dir = workspace.join(".deepseek");
    let md_path = deepseek_dir.join(DEEPSEEK_MD_FILENAME);
    
    if !md_path.exists() {
        return String::new();
    }
    
    match fs::read_to_string(&md_path) {
        Ok(content) => {
            tracing::info!(path = ?md_path, "Loaded DEEPSEEK.md project context");
            content
        }
        Err(e) => {
            tracing::warn!(path = ?md_path, error = %e, "Failed to read DEEPSEEK.md");
            String::new()
        }
    }
}

/// Build the complete system context including project-specific instructions
pub fn build_system_context(
    workspace: &Path,
    base_prompt: &str,
) -> String {
    let mut context = String::new();
    
    // Base system prompt
    context.push_str(base_prompt);
    context.push_str("\n\n");
    
    // Project context from DEEPSEEK.md
    let project_context = load_deepseek_md(workspace);
    if !project_context.is_empty() {
        context.push_str("---\n\n");
        context.push_str("# Project-Specific Instructions\n\n");
        context.push_str(&project_context);
        context.push_str("\n\n---\n");
    }
    
    // Working directory info
    context.push_str(&format!("\nWorking directory: {}\n", workspace.display()));
    
    context
}

/// Check if DEEPSEEK.md exists in the workspace
pub fn has_deepseek_md(workspace: &Path) -> bool {
    workspace
        .join(".deepseek")
        .join(DEEPSEEK_MD_FILENAME)
        .exists()
}

/// Get the path to DEEPSEEK.md (for display purposes)
pub fn deepseek_md_path(workspace: &Path) -> PathBuf {
    workspace
        .join(".deepseek")
        .join(DEEPSEEK_MD_FILENAME)
}

// ===== Backward compatibility: re-export stubs for old API =====

/// Project context structure
#[derive(Debug, Clone)]
pub struct ProjectContext {
    pub content: String,
    pub path: PathBuf,
}

impl ProjectContext {
    /// Convert to system prompt block
    pub fn as_system_block(&self) -> Option<String> {
        if self.content.is_empty() {
            None
        } else {
            Some(format!(
                "# Project Context (from {})\n\n{}",
                self.path.display(),
                self.content
            ))
        }
    }
    
    /// Check if project context has instructions
    pub fn has_instructions(&self) -> bool {
        !self.content.trim().is_empty()
    }
}

/// Stub for backward compatibility
pub fn load_project_context_with_parents(workspace: &Path) -> Option<ProjectContext> {
    // Try to load DEEPSEEK.md
    let content = load_deepseek_md(workspace);
    if content.is_empty() {
        None
    } else {
        Some(ProjectContext {
            content,
            path: deepseek_md_path(workspace),
        })
    }
}

/// Stub for backward compatibility  
pub fn create_default_agents_md(workspace: &Path) -> std::io::Result<PathBuf> {
    let path = workspace.join(".deepseek").join("AGENTS.md");
    // Stub implementation - does nothing
    Ok(path)
}

/// Stub for backward compatibility
pub fn generate_project_context_pack(_workspace: &Path) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_load_deepseek_md_returns_empty_when_missing() {
        let temp_dir = TempDir::new().unwrap();
        let content = load_deepseek_md(temp_dir.path());
        assert!(content.is_empty());
    }

    #[test]
    fn test_load_deepseek_md_returns_content_when_exists() {
        let temp_dir = TempDir::new().unwrap();
        let deepseek_dir = temp_dir.path().join(".deepseek");
        fs::create_dir_all(&deepseek_dir).unwrap();
        
        let md_path = deepseek_dir.join(DEEPSEEK_MD_FILENAME);
        let mut file = fs::File::create(&md_path).unwrap();
        writeln!(file, "# Project Rules").unwrap();
        writeln!(file, "- Use Rust").unwrap();
        writeln!(file, "- No PowerShell").unwrap();
        
        let content = load_deepseek_md(temp_dir.path());
        assert!(content.contains("# Project Rules"));
        assert!(content.contains("Use Rust"));
        assert!(content.contains("No PowerShell"));
    }

    #[test]
    fn test_has_deepseek_md_detects_file() {
        let temp_dir = TempDir::new().unwrap();
        assert!(!has_deepseek_md(temp_dir.path()));
        
        let deepseek_dir = temp_dir.path().join(".deepseek");
        fs::create_dir_all(&deepseek_dir).unwrap();
        let md_path = deepseek_dir.join(DEEPSEEK_MD_FILENAME);
        fs::write(&md_path, "test").unwrap();
        
        assert!(has_deepseek_md(temp_dir.path()));
    }

    #[test]
    fn test_build_system_context_includes_project_info() {
        let temp_dir = TempDir::new().unwrap();
        let deepseek_dir = temp_dir.path().join(".deepseek");
        fs::create_dir_all(&deepseek_dir).unwrap();
        
        let md_path = deepseek_dir.join(DEEPSEEK_MD_FILENAME);
        fs::write(&md_path, "Use TypeScript").unwrap();
        
        let context = build_system_context(temp_dir.path(), "You are an AI assistant");
        assert!(context.contains("You are an AI assistant"));
        assert!(context.contains("Use TypeScript"));
        assert!(context.contains("Project-Specific Instructions"));
    }
}
