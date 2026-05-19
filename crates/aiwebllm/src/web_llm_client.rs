// Web LLM 客户端统一接口
// 支持千问、豆包等网页版 AI 服务

use headless_chrome::{Browser, LaunchOptions};
use crate::client::DoubaoClient;

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

/// Web LLM 客户端（统一接口）
pub struct WebLlmClient {
    platform: WebLlmPlatform,
    browser: Option<Browser>,
    /// 豆包专用客户端（延迟初始化以支持多次调用）
    doubao_client: Option<DoubaoClient>,
}

impl WebLlmClient {
    /// 创建新的 Web LLM 客户端
    pub fn new(platform: WebLlmPlatform) -> Result<Self, Box<dyn std::error::Error>> {
        let browser = Browser::new(LaunchOptions {
            headless: false,
            sandbox: false,
            window_size: Some((1280, 900)),
            ..Default::default()
        })?;

        Ok(Self {
            platform,
            browser: Some(browser),
            doubao_client: None,
        })
    }

    /// 创建豆包专用客户端
    pub fn new_doubao() -> Result<Self, Box<dyn std::error::Error>> {
        let doubao_client = DoubaoClient::new()?;
        Ok(Self {
            platform: WebLlmPlatform::Doubao,
            browser: None,
            doubao_client: Some(doubao_client),
        })
    }

    /// 获取浏览器实例的引用
    pub fn browser(&self) -> Option<&Browser> {
        self.browser.as_ref()
    }

    /// 转移浏览器实例的所有权
    pub fn into_browser(self) -> Option<Browser> {
        self.browser
    }

    /// 发送消息（可选图片）并获取回复
    pub fn send_message(
        &self,
        message: &str,
        image_path: Option<&str>,
        timeout_secs: u64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match self.platform {
            WebLlmPlatform::Doubao => {
                // 优先使用专用客户端
                if let Some(ref client) = self.doubao_client {
                    client.chat(message)
                } else {
                    // 降级到浏览器自动化
                    self.send_message_via_browser(message, image_path, timeout_secs)
                }
            }
            WebLlmPlatform::Qianwen => {
                self.send_message_via_browser(message, image_path, timeout_secs)
            }
        }
    }

    /// 通过浏览器自动化发送消息（千问和豆包通用降级方案）
    fn send_message_via_browser(
        &self,
        message: &str,
        _image_path: Option<&str>,
        timeout_secs: u64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        use std::thread::sleep;

        let browser = self.browser.as_ref()
            .ok_or("浏览器实例不可用")?;

        let tab = browser.new_tab()?;

        // 1. 导航到平台页面
        tab.navigate_to(self.platform.base_url())?;
        tab.wait_until_navigated()?;
        std::thread::sleep(Duration::from_secs(3));

        // 关闭登录弹窗
        if let Ok(close_btn) = tab.find_element(".modal-close, .close-btn, [class*='close']") {
            let _ = close_btn.click();
            std::thread::sleep(Duration::from_secs(1));
        }
        std::thread::sleep(Duration::from_secs(2));

        // 2. 查找输入框（平台差异化选择器）
        eprintln!("[DEBUG] 正在查找输入框...");
        let input_elem = match self.platform {
            WebLlmPlatform::Qianwen => {
                tab.find_element(".editor-content")
                    .or_else(|_| tab.find_element("div[contenteditable='true'].editor-content"))
                    .or_else(|_| tab.find_element("div[contenteditable='true']"))
            }
            WebLlmPlatform::Doubao => {
                // 豆包优先 contenteditable
                tab.find_element("div[contenteditable='true']")
                    .or_else(|_| tab.find_element("textarea"))
                    .or_else(|_| tab.find_element("input[type='text']"))
                    .or_else(|_| tab.find_element(".editor-content"))
            }
        }.map_err(|e| format!("未找到输入框: {}", e))?;
        eprintln!("[DEBUG] 找到输入框");

        // 3. 清理并输入消息
        eprintln!("[DEBUG] 正在清理输入框...");
        input_elem.click()?;
        sleep(Duration::from_millis(300));

        tab.evaluate("(function() {
            const input = document.querySelector('.editor-content') ||
                         document.querySelector('div[contenteditable=\"true\"]') ||
                         document.querySelector('textarea');
            if (input) {
                input.focus();
                input.innerHTML = '';
                input.innerText = '';
                if (input.value !== undefined) input.value = '';
                try { document.execCommand('selectAll', false, null); } catch(e) {}
                try { document.execCommand('delete', false, null); } catch(e) {}
                console.log('[JS] 输入框已清空');
                return 'cleared';
            }
            return 'not_found';
        })()", false)?;
        sleep(Duration::from_millis(500));

        // 4. 输入消息
        eprintln!("[DEBUG] 正在输入消息: {}", message);
        input_elem.type_into(message)?;
        eprintln!("[DEBUG] 消息已输入");
        sleep(Duration::from_secs(1));

        // 记录初始回复数量
        let initial_reply_count: usize = if let Ok(count_result) = tab.evaluate(
            "(function() { return document.querySelectorAll('.markdown-body, [class*=\"bubble\"]').length; })()", false
        ) {
            count_result.value.and_then(|v| v.as_f64()).unwrap_or(0.0) as usize
        } else { 0 };

        // 5. 发送消息
        eprintln!("[DEBUG] 正在发送消息...");
        let js_send = self.get_send_button_js();
        let result = tab.evaluate(&js_send, false);
        eprintln!("[DEBUG] 发送结果: {:?}", result);
        sleep(Duration::from_secs(2));

        // 6. 等待并获取回复
        let max_wait = std::cmp::max(timeout_secs, 120);
        eprintln!("[DEBUG] 开始等待回复（最多 {} 秒）...", max_wait);
        let mut elapsed = 0;
        let mut last_text_length = 0;
        let mut stable_count = 0;

        while elapsed < max_wait {
            std::thread::sleep(Duration::from_secs(2));
            elapsed += 2;
            eprintln!("[DEBUG] 第 {} 秒，检测 AI 回复...", elapsed);

            let js_code = self.get_response_js(initial_reply_count);

            if let Ok(result) = tab.evaluate(&js_code, false) {
                if let Some(value) = result.value {
                    let raw_text = value.to_string().trim_matches('"').to_string();
                    let text = raw_text
                        .strip_prefix("strategy1:").unwrap_or(&raw_text)
                        .strip_prefix("strategy2:").unwrap_or(&raw_text)
                        .strip_prefix("strategy3:").unwrap_or(&raw_text)
                        .to_string();

                    if text.is_empty() || text.starts_with("error:") {
                        continue;
                    }

                    let current_len = text.len();
                    eprintln!("[DEBUG] 获取到内容长度: {} 字符", current_len);

                    if current_len == last_text_length && current_len > 50 {
                        stable_count += 1;
                        eprintln!("[DEBUG] 长度稳定 {}/8 次", stable_count);
                        if stable_count >= 8 {
                            eprintln!("[DEBUG] 回复已完成");
                            return Ok(text);
                        }
                    } else {
                        stable_count = 0;
                    }
                    last_text_length = current_len;
                }
            }
        }

        Err(format!("超时：{} 秒未获取到完整回复", max_wait).into())
    }

    /// 获取平台特定的发送按钮 JS
    fn get_send_button_js(&self) -> String {
        match self.platform {
            WebLlmPlatform::Qianwen => r#"(function() {
                const selectors = [
                    'button.send-btn',
                    '[aria-label="发送消息"]',
                    'button.send-button',
                ];
                for (const sel of selectors) {
                    const btn = document.querySelector(sel);
                    if (btn && btn.offsetParent !== null) {
                        btn.click();
                        return 'clicked:' + sel;
                    }
                }
                // Enter 键备选
                const input = document.querySelector('[data-slate-editor="true"]') ||
                             document.querySelector('div[contenteditable="true"]');
                if (input) {
                    input.focus();
                    input.dispatchEvent(new KeyboardEvent('keydown', {key:'Enter', code:'Enter', keyCode:13, which:13, bubbles:true}));
                    return 'pressed:Enter';
                }
                return 'not_found';
            })()"#.to_string(),

            WebLlmPlatform::Doubao => r#"(function() {
                const selectors = [
                    'button[type="submit"]',
                    'button[class*="send"]',
                    'button[aria-label*="发送"]',
                    '[class*="send-button"]',
                    '[class*="bubble"] button',
                    '.chat-send-btn',
                ];
                for (const sel of selectors) {
                    const btn = document.querySelector(sel);
                    if (btn && btn.offsetParent !== null) {
                        btn.click();
                        return 'clicked:' + sel;
                    }
                }
                // 文本匹配
                const allBtns = document.querySelectorAll('button');
                for (const btn of allBtns) {
                    const text = btn.textContent.trim();
                    if ((text === '发送' || text.includes('发送')) && btn.offsetParent !== null) {
                        btn.click();
                        return 'clicked:text:' + text;
                    }
                }
                // Enter 键
                const input = document.querySelector('div[contenteditable="true"]') ||
                             document.querySelector('textarea');
                if (input) {
                    input.focus();
                    input.dispatchEvent(new KeyboardEvent('keydown', {key:'Enter', code:'Enter', keyCode:13, which:13, bubbles:true}));
                    return 'pressed:Enter';
                }
                return 'not_found';
            })()"#.to_string(),
        }
    }

    /// 获取平台特定的回复提取 JS
    fn get_response_js(&self, initial_count: usize) -> String {
        match self.platform {
            WebLlmPlatform::Qianwen => format!(r#"(function() {{
                const markdownEls = document.querySelectorAll('.markdown-body');
                if (markdownEls.length > {}) {{
                    const text = markdownEls[markdownEls.length - 1].innerText.trim();
                    if (text.length > 10) return 'strategy1:' + text;
                }}
                if (markdownEls.length > 0 && {} === 0) {{
                    return 'strategy1:' + markdownEls[markdownEls.length - 1].innerText.trim();
                }}
                const selectors = ['[class*="response"]', '[class*="assistant"]', '[role="article"]'];
                for (const sel of selectors) {{
                    const els = document.querySelectorAll(sel);
                    for (let i = els.length - 1; i >= 0; i--) {{
                        const t = els[i].innerText.trim();
                        if (t.length > 10) return 'strategy2:' + t;
                    }}
                }}
                return '';
            }})()"#, initial_count, initial_count),

            WebLlmPlatform::Doubao => r#"(function() {
                const targetSelectors = [
                    "[class*='bubble']",
                    "[class*='message-content']",
                    "[class*='chat-message']",
                    "[class*='reply-content']",
                    "[class*='markdown-body']",
                    "[class*='prose']",
                    "[role='article']",
                ];
                let best = '';
                let bestLen = 0;
                for (const sel of targetSelectors) {
                    const els = document.querySelectorAll(sel);
                    for (let i = els.length - 1; i >= 0; i--) {
                        const t = els[i].innerText.trim();
                        if (t.length > bestLen && t.length > 20 && !t.includes('null')) {
                            bestLen = t.length;
                            best = t;
                            break;
                        }
                    }
                    if (bestLen > 50) break;
                }
                return best;
            })()"#.to_string(),
        }
    }

    /// 上传图片（仅千问）
    pub fn upload_image(&self, tab: &headless_chrome::Tab, image_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::path::Path;
        use std::thread::sleep;

        let path = Path::new(image_path);
        if !path.exists() {
            return Err(format!("图片文件不存在: {}", image_path).into());
        }

        let abs_path = std::fs::canonicalize(path)?;
        let _path_str = abs_path.to_str().ok_or("无效的文件路径")?;
        eprintln!("[DEBUG] 图片绝对路径: {:?}", abs_path);

        let js_click_upload = r#"(function() {
            const uploadButtons = [
                document.querySelector('button[aria-label*="上传"]'),
                document.querySelector('button[aria-label*="图片"]'),
                document.querySelector('[class*="upload"] button'),
            ];
            for (const btn of uploadButtons) {
                if (btn && btn.offsetParent !== null) {
                    btn.click();
                    return 'clicked';
                }
            }
            return 'not-found';
        })()"#;

        let result = tab.evaluate(js_click_upload, false)?;
        eprintln!("[DEBUG] 点击上传按钮结果: {:?}", result);
        sleep(Duration::from_millis(500));

        let js_set_file = r#"(function() {
            const fileInputs = document.querySelectorAll('input[type="file"]');
            for (const input of fileInputs) {
                if (input.offsetParent !== null || input.style.display !== 'none') {
                    return 'found-file-input';
                }
            }
            return 'not-found';
        })()"#;

        let file_result = tab.evaluate(js_set_file, false)?;
        eprintln!("[DEBUG] 文件输入框: {:?}", file_result);

        eprintln!("[DEBUG] 图片上传功能已初始化，请在浏览器中手动选择文件");
        Ok(())
    }
}
