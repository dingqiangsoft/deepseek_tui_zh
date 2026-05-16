// Web LLM Integration - Module Entry
// Phase A MVP: Complete browser automation + TUI commands (Qianwen www.qianwen.com)

mod client;
mod config;
mod error;
mod session;
pub mod web_llm_client;

pub use client::BrowserController;
pub use error::WebLlmError;
pub use session::SessionManager;