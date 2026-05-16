// Web LLM 客户端统一接口
// 支持千问、豆包等网页版 AI 服务

use headless_chrome::{Browser, LaunchOptions};
use std::time::Duration;

/// Web LLM 平台类型
#[derive(Debug, Clone, Copy)]
pub enum WebLlmPlatform {
    Qianwen,  // 千问
    Doubao,   // 豆包
}

impl WebLlmPlatform {
    /// 获取平台的基础 URL
    pub fn base_url(&self) -> &'static str {
        match self {
            WebLlmPlatform::Qianwen => "https://www.qianwen.com/",
            WebLlmPlatform::Doubao => "https://www.doubao.com/",
        }
    }
    
    /// 获取平台名称
    pub fn name(&self) -> &'static str {
        match self {
            WebLlmPlatform::Qianwen => "千问",
            WebLlmPlatform::Doubao => "豆包",
        }
    }
}

/// Web LLM 客户端
pub struct WebLlmClient {
    platform: WebLlmPlatform,
    browser: Browser,
}

impl WebLlmClient {
    /// 创建新的 Web LLM 客户端
    pub fn new(platform: WebLlmPlatform) -> Result<Self, Box<dyn std::error::Error>> {
        let browser = Browser::new(LaunchOptions {
            headless: false,  // 显示浏览器窗口
            sandbox: false,
            window_size: Some((1280, 800)),
            ..Default::default()
        })?;
        
        Ok(Self { platform, browser })
    }
    
    /// 发送消息并获取回复
    pub fn send_message(&self, message: &str, timeout_secs: u64) -> Result<String, Box<dyn std::error::Error>> {
        use std::{thread::sleep, io::Write};
        
        let tab = self.browser.new_tab()?;
        
        // 1. 导航到平台页面
        tab.navigate_to(self.platform.base_url())?;
        tab.wait_until_navigated()?;
        sleep(Duration::from_secs(3));
        
        // 2. 查找输入框
        let input_elem = tab.find_element("[contenteditable='true']")
            .or_else(|_| tab.find_element("textarea"))
            .or_else(|_| tab.find_element("input[type='text']"))
            .map_err(|e| format!("未找到输入框: {}", e))?;
        
        // 3. 输入消息
        input_elem.click()?;
        sleep(Duration::from_millis(500));
        input_elem.type_into(message)?;
        sleep(Duration::from_secs(1));
        
        // 4. 发送消息
        let js_click = r#"(function() {
            const selectors = [
                'button[type="submit"]',
                '.send-button',
                '[aria-label="发送"]',
                'button.send-btn',
                '.chat-input button',
                'button[class*="send"]',
                'button[class*="Submit"]',
                '.submit-btn'
            ];
            
            for (const sel of selectors) {
                const btn = document.querySelector(sel);
                if (btn && btn.offsetParent !== null) {
                    btn.click();
                    return 'clicked:' + sel;
                }
            }
            
            const allButtons = document.querySelectorAll('button');
            for (const btn of allButtons) {
                if (btn.textContent.includes('发送') && btn.offsetParent !== null) {
                    btn.click();
                    return 'clicked:text[发送]';
                }
            }
            
            return 'not_found';
        })()"#;
        
        let _ = tab.evaluate(js_click, false);
        sleep(Duration::from_secs(2));
        
        // 5. 等待并获取回复
        let max_wait = timeout_secs;
        let mut elapsed = 0;
        
        while elapsed < max_wait {
            sleep(Duration::from_secs(2));
            elapsed += 2;
            
            // 每 4 秒尝试获取一次回复
            if elapsed % 4 == 0 {
                if let Ok(result) = tab.evaluate("(function() {
                    const selectors = [
                        '[class*=\"message\"]',
                        '[class*=\"chat\"]',
                        '[class*=\"response\"]',
                        '[class*=\"answer\"]',
                        'div[role=\"article\"]',
                        '.markdown-body',
                        '[class*=\"assistant\"]',
                        '[class*=\"bot\"]'
                    ];
                    
                    let allTexts = [];
                    selectors.forEach(sel => {
                        document.querySelectorAll(sel).forEach(el => {
                            const text = el.innerText.trim();
                            if (text.length > 100) {
                                allTexts.push(text);
                            }
                        });
                    });
                    
                    return allTexts[0] || '';
                })()", false) {
                    if let Some(value) = result.value {
                        let text = value.to_string().trim_matches('"').to_string();
                        if text.len() > 100 && !text.contains("null") {
                            return Ok(text);
                        }
                    }
                }
            }
        }
        
        Err(format!("超时：未在 {} 秒内获取到回复", timeout_secs).into())
    }
}
