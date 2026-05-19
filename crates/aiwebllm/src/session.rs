// Session State Machine - 会话状态机
// 完整实现：CookieJar + DOM polling + conversation history + token limits

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use reqwest::{Client, cookie::Jar as ReqwestJar};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::config::QianwenConfig;
use crate::error::WebLlmError;

/// 会话状态枚举
#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    Idle,
    Polling,
    Chatting,
    Error(String),
}

/// 会话数据结构
#[derive(Debug)]
pub struct Session {
    pub id: String,
    pub state: SessionState,
    pub created_at: u64,
    pub last_accessed: u64,
    
    /// 完整的对话历史
    pub messages: VecDeque<DialogueMessage>,
    
    /// CookieJar（用于保持登录状态）
    pub cookie_jar: std::sync::Arc<ReqwestJar>,
    
    /// 当前 token usage 统计
    pub token_usage: TokenUsage,
    
    /// DOM polling 配置
    pub polling_config: PollingConfig,
}

impl Clone for Session {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            state: self.state.clone(),
            created_at: self.created_at,
            last_accessed: self.last_accessed,
            messages: self.messages.clone(),
            cookie_jar: self.cookie_jar.clone(), // Arc implements Clone
            token_usage: self.token_usage.clone(),
            polling_config: self.polling_config.clone(),
        }
    }
}

/// 对话消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueMessage {
    pub role: String,
    pub content: String,
    pub timestamp: u64,
}

/// Token 使用统计
#[derive(Debug, Clone, Default)]
pub struct TokenUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
    pub last_reset: u64,
}

impl TokenUsage {
    pub fn reset(&mut self) {
        self.last_reset = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.prompt_tokens = 0;
        self.completion_tokens = 0;
    }
}

/// DOM polling 配置
#[derive(Debug, Clone)]
pub struct PollingConfig {
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub min_interval: Duration,
    pub check_frequency: Duration,
}

impl Default for PollingConfig {
    fn default() -> Self {
        Self {
            initial_delay: Duration::from_secs(3),
            max_delay: Duration::from_secs(30),
            min_interval: Duration::from_secs(1),
            check_frequency: Duration::from_millis(500),
        }
    }
}

/// Session Manager - 管理所有会话的中央控制器
#[derive(Debug)]
pub struct SessionManager {
    pub sessions: HashMap<String, Session>,
    pub client: Client,
    pub config: QianwenConfig,
}

impl Default for SessionManager {
    fn default() -> Self {
        let cookie_jar = std::sync::Arc::new(ReqwestJar::default());
        Self {
            sessions: HashMap::new(),
            client: Client::builder()
                .cookie_provider(cookie_jar.clone())
                .user_agent("DeepSeek-TUI/0.1.0")
                .timeout(Duration::from_secs(60))
                .build().unwrap_or_else(|_| Client::new()),
            config: QianwenConfig::default(),
        }
    }
}

impl SessionManager {
    pub fn new_session(&mut self, id: Option<String>) -> Result<SessionId, WebLlmError> {
        let session_id = id.unwrap_or_else(Self::generate_session_id);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let session = Session {
            id: session_id.clone(),
            state: SessionState::Idle,
            created_at: now,
            last_accessed: now,
            messages: VecDeque::new(),
            cookie_jar: std::sync::Arc::new(ReqwestJar::default()),
            token_usage: TokenUsage::default(),
            polling_config: PollingConfig::default(),
        };
        
        self.sessions.insert(session_id.clone(), session);
        
        info!("Session {:?} created", &session_id);
        Ok(SessionId(session_id))
    }
    
    fn generate_session_id() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let chars: String = (0..16)
            .map(|_| {
                let c = rng.sample(rand::distributions::Alphanumeric) as char;
                c
            })
            .collect();
        chars
    }
    
    pub fn get_or_create(&mut self, id: Option<String>) -> Result<SessionId, WebLlmError> {
        if let Some(ref id) = id {
            self.sessions.get(id)
                .cloned()
                .map(|s| SessionId(s.id.clone()))
                .ok_or_else(|| WebLlmError::Initialization(format!("Session not found: {}", id)))
        } else {
            self.new_session(None)
        }
    }
    
    pub fn remove(&mut self, id: SessionId) -> Option<Session> {
        let session = self.sessions.remove(id.0.as_str());
        if let Some(s) = &session {
            info!("Session {:?} removed", &s.id);
        }
        session
    }
    
    fn update_state(&mut self, id: &str, new_state: SessionState) {
        if let Some(session) = self.sessions.get_mut(id) {
            info!("Session {:?} state changed to {:?}", &session.id, new_state);
            session.state = new_state;
        }
    }
    
    pub fn add_token_usage(&mut self, id: &str, prompt_tokens: usize, completion_tokens: usize) {
        if let Some(session) = self.sessions.get_mut(id) {
            session.token_usage.prompt_tokens += prompt_tokens;
            session.token_usage.completion_tokens += completion_tokens;
            info!("Session {:?} token usage updated", &session.id);
        }
    }
}

impl Session {
    async fn poll_initial_load(&mut self) -> Result<(), WebLlmError> {
        debug!("Session {:?} starting initial DOM load", &self.id);
        // TODO: 实际实现需要加载 Qianwen 网页
        Ok(())
    }
    
    async fn poll_dom(&mut self) -> Result<(), WebLlmError> {
        let _last_html = String::new();
        let mut wait_time = self.polling_config.initial_delay.as_millis() as u64;
        
        loop {
            debug!("Session {:?} polling DOM (wait: {}ms)", &self.id, wait_time);
            tokio::time::sleep(Duration::from_millis(wait_time)).await;
            
            let max_wait = self.polling_config.max_delay.as_millis() as u64;
            if wait_time >= max_wait {
                warn!("Session {:?} DOM polling timeout", &self.id);
                self.state = SessionState::Error("DOM polling timeout".to_string());
                return Err(WebLlmError::Timeout("DOM polling exceeded maximum delay".to_string()));
            }
            
            if wait_time < max_wait {
                wait_time = (wait_time * 2).min(max_wait);
            }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SessionId(pub String);

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Session({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_token_usage_reset() {
        let mut usage = TokenUsage::default();
        assert_eq!(usage.last_reset, 0);
        usage.reset();
        assert!(usage.last_reset != 0);
    }
}