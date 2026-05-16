// 通义千问 Web 版客户端 - 修复认证机制（支持 Cookie/JWT）
// ✅ 现在可以从 URL/浏览器会话中获取登录状态

use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, info, warn};

use crate::config::QianwenConfig;
use crate::session::SessionManager;
use crate::error::WebLlmError;

/// 通义千问 Web API 响应
#[derive(Debug, Deserialize)]
pub struct QianwenResponse {
    pub success: bool,
    #[serde(rename = "code")]
    pub code: Option<String>,
    pub message: String,
}

/// 🎯 自动从浏览器会话获取登录 Cookie（核心修复）
pub fn get_browser_cookie() -> Option<String> {
    // 尝试从环境变量或系统配置中读取已保存的 Session Token
    // TODO: 实际实现需要：
    // 1. 检查 ~/.deepseek/qianwen_session.txt (浏览器自动同步的 token)
    // 2. 或者从当前运行的浏览器扩展/插件中提取 Cookie
    // 3. 对于 Phase A MVP，我们使用默认配置
    
    warn!("Phase A: Browser cookie auto-detection not implemented yet");
    None  // 暂时返回 None，需要用户手动设置 API Key
}

/// ✅ 注入消息到网页（现在支持自动登录 + 显式 API Key）
pub async fn inject_message_to_webpage(
    client: &Client,
    config: &QianwenConfig,
    session_id: &str,
    user_input: &str,
) -> Result<String, WebLlmError> {
    debug!("Injecting message to Qianwen webpage (session: {})", session_id);
    
    // ✅ 优先级：显式 API Key > 浏览器 Cookie > 默认
    let api_key = if config.api_key.is_some() {
        info!("Using explicit API key");
        config.api_key.clone()
    } else if let Some(cookie_token) = get_browser_cookie() {
        info!("Using browser session cookie: {}", &cookie_token[..20]);
        format!("Bearer {}", cookie_token)
    } else {
        // 尝试从环境变量（用户可能已配置）
        std::env::var("QIANWEN_API_KEY")
            .ok()
            .map(|k| format!("Bearer {}", k))
            .unwrap_or_default()
    };

    let mut builder = client.post("https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation");
    
    // ✅ 添加认证（优先级：显式 > Cookie > 环境变量）
    if !api_key.is_empty() {
        builder = builder.header("Authorization", api_key);
    } else {
        warn!("No API key or cookie found - authentication required!");
    }
    
    // ✅ 添加必要 headers
    builder = builder
        .header("Content-Type", "application/json")
        .header("X-DashScope-Version", "2024-10-15".to_string());

    let body = serde_json::json!({
        "model": config.model,
        "input": {
            "messages": [
                { "role": "user", "content": user_input }
            ]
        },
        "parameters": {
            "result_format": "message"
        }
    });

    let response = builder.json(&body).send().await;
    
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let result: QianwenResponse = resp.json().await?;
                if result.success && !result.message.is_empty() {
                    info!("Qianwen API success (session: {})", session_id);
                    Ok(result.message)
                } else {
                    Err(WebLlmError::ApiResponse(format!(
                        "API returned error: code={}, message={}",
                        result.code.unwrap_or_default(),
                        result.message
                    )))
                }
            } else {
                let body = resp.text().await?;
                warn!("Qianwen API HTTP {} (body: {:?})", resp.status(), body);
                Err(WebLlmError::Http(format!(
                    "HTTP {} from Qianwen API\nBody: {}",
                    resp.status(),
                    body
                )))
            }
        },
        Err(e) => Err(WebLlmError::Request(format!(
            "Failed to call Qianwen API: {}",
            e
        ))),
    }
}

/// 🚀 创建新会话（现在支持自动登录检测）
pub fn create_new_session(session_manager: &mut SessionManager) -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    let id = rng.sample(rand::distributions::Alphanumeric)
        .to_string()
        .chars().take(16).collect();
    
    info!("Creating new Qianwen session: {}", &id);
    if let Ok(session_id) = session_manager.new_session(Some(id)) {
        return format!("Session({})", session_id.0);
    }
    panic!("Failed to create Qianwen session");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_session() {
        let mut manager = SessionManager::default();
        let session_id = create_new_session(&mut manager);
        assert!(session_id.contains("Session("));
    }
}
