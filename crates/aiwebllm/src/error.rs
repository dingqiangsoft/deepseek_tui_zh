// Error Handling - Complete error types
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebLlmError {
    #[error("HTTP {0}")]
    Http(String),
    
    #[error("Request failed: {0}")]
    Request(String),
    
    #[error("API response error: code={0}, message={1}")]
    ApiResponse(String, String),
    
    #[error("Initialization error: {0}")]
    Initialization(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Timeout after {0}ms")]
    Timeout(String),
    
    #[error("Token limit exceeded: current={0}, max={1}")]
    TokenLimit(usize, usize),
}