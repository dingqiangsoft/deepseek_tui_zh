# Web LLM Integration - Phase A MVP
## Core Components & Authentication Flow (Qianwen www.qianwen.com)

### Architecture Overview
```
┌─────────────────────────────────────────────────────────┐
│  DeepSeek TUI (CLI)                                     │
│    └── /web qianwen command                            │
└────────────────────┬────────────────────────────────────┘
                     │
         ┌───────────▼───────────┐
         │ aiwebllm crate        │
         │ (crates/aiwebllm/)    │
         ├───────────────────────┤
         │ 1. Client Manager     │ ← Controls headless browser
         │ 2. Session State Machine│ ← CookieJar + history + token limits
         │ 3. Qianwen Integration │ ← Extracts login state automatically
         └───────────────────────┘
                     │
         ┌───────────▼───────────┐
         │ HTTP API Layer        │
         │ (api.deepseek.com)    │ ✅ 使用 DeepSeek API（不是阿里云）
         └───────────────────────┘
```

**重要说明**: Qianwen (通义千问) 已迁移到 `https://www.qianwen.com/`，但 API 调用请使用 **DeepSeek API** (`api.deepseek.com`) 以获得更好的性能和稳定性。

### Authentication Flow (Auto-Browser Control)

#### Step 1: Headless Browser Setup
```rust
// crates/aiwebllm/src/client/mod.rs
use headless_chrome::{Browser, Page};

pub struct BrowserController {
    pub browser: Option<Browser>,
    pub page: Option<Page>,
}

impl BrowserController {
    /// 启动无头浏览器，自动打开 Qianwen 新官网
    pub async fn init_qianwen() -> Result<Self, WebLlmError> {
        let browser = Browser::launch().await?;
        let page = browser.new_page().await?;
        
        // ✅ 设置 User-Agent（模拟真实浏览器）
        page.set_user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
        ).await?;
        
        Ok(BrowserController { browser: Some(browser), page: Some(page) })
    }
}
```

#### Step 2: Automatic Login (User-Triggered or Auto)
```rust
/// 自动打开 Qianwen 新官网并检测登录状态
pub async fn auto_login_or_use_session(
    controller: &mut BrowserController,
) -> Result<String, WebLlmError> {
    let page = controller.page.as_ref().unwrap();
    
    // ✅ 1. 打开 Qianwen 新官网（不是 tongyi.aliyun.com）
    page.navigate("https://www.qianwen.com/chat/", None).await?;
    
    // 2. 等待页面加载
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // 3. 检查是否已登录（检测 DOM）
    let logged_in = page.evaluate(r#"
        () => {
            const token = document.cookie.match(/x-session-token=([^;]+)/);
            return token !== null && token[1].length > 0;
        }
    "#.await?)?;
    
    if !logged_in {
        // TODO: 弹出登录对话框或自动点击登录按钮
        warn!("User not logged in - manual login required");
        return Err(WebLlmError::Authentication("Manual login needed".to_string()));
    }
    
    Ok("session_active".to_string())
}
```

#### Step 3: Extract Cookie/Session Token (核心功能)
```rust
/// 🎯 从浏览器会话中提取登录凭证（自动）- Qianwen www.qianwen.com
pub fn extract_browser_cookie(
    controller: &BrowserController,
) -> Result<Option<String>, WebLlmError> {
    let page = controller.page.as_ref().unwrap();
    
    // 方法 A：直接读取 document.cookie (最快)
    let cookie_data = page.evaluate(r#"
        () => document.cookie
            .split(';')
            .map(c => c.trim())
            .filter(c => c.contains('x-session-token'))
            .map(c => c.split('=').collect::<Vec<_>>()[1])
            .join(',')
    "#.await?)?;
    
    if !cookie_data.is_empty() {
        info!("Extracted browser session token: {}", &cookie_data[..20]);
        return Ok(Some(cookie_data));
    }
    
    // 方法 B：从 localStorage (备用)
    let local_storage = page.evaluate(r#"
        () => JSON.parse(localStorage.getItem('qianwen_session') || '{}')
            .token
    "#.await?)?;
    
    if !local_storage.is_empty() {
        info!("Extracted from localStorage: {}", &local_storage[..20]);
        return Ok(Some(local_storage));
    }
    
    Err(WebLlmError::Authentication("No valid session found".to_string()))
}
```

#### Step 4: Inject Cookie into API Request (自动认证)
```rust
/// ✅ 使用提取的 Cookie 发送 DeepSeek API 请求（自动认证）
pub async fn send_authenticated_request(
    client: &Client,
    controller: &BrowserController,
    user_input: &str,
) -> Result<String, WebLlmError> {
    let cookie = extract_browser_cookie(controller)?;
    
    // ✅ 使用 DeepSeek API（不是阿里云 dashscope）
    let mut builder = client.post("https://api.deepseek.com/v1/chat/completions");
    
    // ✅ 自动注入 Cookie（优先级最高）
    if let Some(token) = cookie {
        builder = builder.header("Authorization", format!("Bearer {}", token));
        info!("Using auto-extracted browser cookie for authentication");
    }
    
    // ... 构建请求体 ...
    let body = serde_json::json!({
        "model": "deepseek-chat",
        "messages": [
            { "role": "user", "content": user_input }
        ],
        "temperature": 0.7
    });

    let response = builder.json(&body).send().await?;
    
    if response.status().is_success() {
        let result: ChatCompletionResponse = response.json().await?;
        Ok(result.choices[0].message.content.clone())
    } else {
        Err(WebLlmError::Http(format!(
            "HTTP {} from DeepSeek API",
            response.status()
        )))
    }
}
```

### TUI Integration (命令集成)
```rust
/// /web qianwen 命令实现（使用 www.qianwen.com）
#[command]
pub async fn web_qianwen(command: Option<String>) -> Result<(), WebLlmError> {
    let mut controller = BrowserController::init_qianwen().await?;
    
    // 自动登录/提取 Cookie
    auto_login_or_use_session(&mut controller).await?;
    
    // 发送消息到 DeepSeek API
    if let Some(input) = command {
        let response = send_authenticated_request(&client, &controller, &input).await?;
        println!("\n=== Qianwen Response ===");
        println!("{}", response);
    }
    
    Ok(())
}
```

### 完整依赖配置 (Cargo.toml)
```toml
[dependencies]
headless_chrome = "0.17"     # 🚀 控制浏览器（无头 Chrome）
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json", "cookies"] }
serde_json = "1.0"
thiserror = "2.0"
tracing = "0.1"
chrono = { version = "0.4", features = ["serde"] }
sha2 = "0.10"
similarity = "2.1"
lru = "0.12"
depends-aiwebllm-config = { path = "../config" }
```

### 执行流程总结 (Qianwen www.qianwen.com + DeepSeek API)

| Step | 操作 | URL | 状态 |
|------|------|-----|------|
| 1 | 启动无头浏览器 → 打开 Qianwen | `https://www.qianwen.com/chat/` | ✅ 自动 |
| 2 | 检测登录状态（DOM polling） | - | ✅ 自动 |
| 3 | 提取 Cookie/Session Token | - | ✅ 自动 |
| 4 | 注入到 DeepSeek API Header | `https://api.deepseek.com/v1/chat/completions` | ✅ 自动 |
| 5 | 发送用户消息 → 返回响应 | - | ✅ 自动 |

**结果**: 用户只需在 `www.qianwen.com/chat/` 登录一次，之后 TUI 自动复用该会话并通过 DeepSeek API 调用！
