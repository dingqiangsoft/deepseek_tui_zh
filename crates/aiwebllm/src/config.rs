// Configuration Management - Complete config loading
use serde::{Deserialize, Serialize};
use std::env;

/// Qianwen (通义千问) API 配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QianwenConfig {
    /// Model name (e.g., "qwen-turbo", "qwen-max")
    pub model: String,
    /// API Key - can be set via environment variable QIANWEN_API_KEY
    #[serde(default)]
    pub api_key: Option<String>,
}

impl Default for QianwenConfig {
    fn default() -> Self {
        Self {
            model: "qwen-turbo".to_string(),
            api_key: env::var("QIANWEN_API_KEY")
                .ok()
                .map(|k| k.trim().to_string()),
        }
    }
}

/// Config module entry point (placeholder for now)
pub fn load_config() -> Result<QianwenConfig, String> {
    // TODO: Load from ~/.deepseek/config.toml or environment variables
    Ok(QianwenConfig::default())
}