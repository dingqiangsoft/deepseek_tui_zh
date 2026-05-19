// 豆包 Web 版客户端
// 专门针对 doubao.com 优化，支持对话、搜索和内容提取

use headless_chrome::{Browser, LaunchOptions, Tab};
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;

/// 豆包客户端配置
#[derive(Debug, Clone)]
pub struct DoubaoConfig {
    /// 浏览器启动选项
    pub launch_options: LaunchOptions<'static>,
    /// 等待输入框超时（秒）
    pub input_wait_timeout: u64,
    /// 发送消息超时（秒）
    pub send_timeout: u64,
    /// 轮询间隔（毫秒）
    pub poll_interval_ms: u64,
    /// 稳定性检测次数（连续 N 次内容不变则认为完成）
    pub stable_threshold: usize,
    /// 最大重试次数
    pub max_retries: usize,
    /// 是否显示浏览器窗口（调试用）
    pub headless: bool,
}

impl Default for DoubaoConfig {
    fn default() -> Self {
        Self {
            headless: false,
            launch_options: LaunchOptions {
                headless: false,
                sandbox: false,
                window_size: Some((1280, 900)),
                ..Default::default()
            },
            input_wait_timeout: 30,
            send_timeout: 180,
            poll_interval_ms: 2000,
            stable_threshold: 6,  // 6 * 2s = 12s 稳定即认为完成
            max_retries: 2,
        }
    }
}

/// 豆包客户端
pub struct DoubaoClient {
    config: DoubaoConfig,
    browser: Browser,
}

impl DoubaoClient {
    /// 创建新的豆包客户端
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Self::with_config(DoubaoConfig::default())
    }

    /// 使用自定义配置创建客户端
    pub fn with_config(config: DoubaoConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let browser = Browser::new(config.launch_options.clone())?;
        println!("[DoubaoClient] 浏览器已启动（headless: {}）", config.headless);
        Ok(Self { config, browser })
    }

    /// 发送消息并获取回复
    pub fn chat(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        let tab = self.browser.new_tab()?;
        self.chat_with_tab(&tab, message)
    }

    /// 使用现有 tab 发送消息
    pub fn chat_with_tab(&self, tab: &Tab, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut last_error = String::new();

        for attempt in 0..=self.config.max_retries {
            if attempt > 0 {
                println!("[DoubaoClient] 重试 #{}/{}...", attempt, self.config.max_retries);
                sleep(Duration::from_secs(2));
            }

            match self.do_chat(tab, message) {
                Ok(response) => return Ok(response),
                Err(e) => {
                    let err_str = format!("{}", e);
                    last_error = err_str.clone();
                    println!("[DoubaoClient] 第 {} 次尝试失败: {}", attempt + 1, e);
                }
            }
        }

        Err(format!("豆包聊天失败，已重试 {} 次。最后错误: {}", self.config.max_retries, last_error).into())
    }

    /// 执行单次聊天
    fn do_chat(&self, tab: &Tab, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 1. 导航并处理登录
        self.navigate_and_wait_for_login(tab)?;

        // 2. 等待输入框出现
        self.wait_for_input(tab)?;

        // 3. 输入消息（通过 JS 直接操作，无需返回 Element）
        self.fill_input_via_js(tab, message)?;

        // 4. 发送消息
        self.send_message(tab)?;

        // 5. 等待并获取回复
        self.wait_for_response(tab)
    }

    /// 导航到豆包首页并处理登录
    fn navigate_and_wait_for_login(&self, tab: &Tab) -> Result<(), Box<dyn std::error::Error>> {
        const DOUBAO_URL: &str = "https://www.doubao.com/";

        println!("[DoubaoClient] 导航到: {}", DOUBAO_URL);
        tab.navigate_to(DOUBAO_URL)?;
        tab.wait_until_navigated()?;

        // 等待页面基本加载
        sleep(Duration::from_secs(3));

        // 检测 URL 判断是否需要登录
        let url = tab.get_url();
        let needs_login = url.contains("login") || url.contains("signin") || url.contains("auth");

        if needs_login {
            println!("[DoubaoClient] 检测到登录页面，等待用户登录（60秒）...");
            self.wait_for_manual_login(60)?;
        } else {
            // 关闭可能存在的引导弹窗
            self.close_popups(tab)?;
            println!("[DoubaoClient] 已到达豆包页面（可能已登录）");
        }

        Ok(())
    }

    /// 等待手动登录
    fn wait_for_manual_login(&self, seconds: u64) -> Result<(), Box<dyn std::error::Error>> {
        for i in (1..=seconds).rev() {
            print!("\r[DoubaoClient] 等待登录: {} 秒", i);
            std::io::stdout().flush().ok();
            sleep(Duration::from_secs(1));

            // 这里需要通过 Tab 检查是否已登录
            // 但 headless_chrome 的 Tab 是不可变的借用，需要重新设计
        }
        println!();
        Ok(())
    }

    /// 关闭可能存在的弹窗
    fn close_popups(&self, tab: &Tab) -> Result<(), Box<dyn std::error::Error>> {
        // 尝试关闭常见的弹窗元素
        let close_js = r#"(function() {
            const closeSelectors = [
                '.modal-close', '.close-btn', '[class*="close"]',
                '[aria-label="关闭"]', '[aria-label="Close"]',
                '.login-modal button', '[class*="guide"] button',
                '.welcome-modal button', '.tips-dialog button'
            ];

            for (const sel of closeSelectors) {
                const btn = document.querySelector(sel);
                if (btn && btn.offsetParent !== null && btn.textContent.trim().length < 10) {
                    btn.click();
                    console.log('[DoubaoClient] 关闭弹窗:', sel);
                    return 'closed:' + sel;
                }
            }
            return 'no-popup';
        })()"#;

        if let Ok(result) = tab.evaluate(close_js, false) {
            if let Some(val) = result.value {
                let s = val.to_string();
                if s.starts_with("closed:") {
                    println!("[DoubaoClient] 已关闭弹窗");
                    sleep(Duration::from_secs(1));
                }
            }
        }
        Ok(())
    }

    /// 等待并找到输入框
    fn wait_for_input(&self, tab: &Tab) -> Result<(), Box<dyn std::error::Error>> {
        println!("[DoubaoClient] 等待输入框出现...");

        let start = std::time::Instant::now();
        let timeout = Duration::from_secs(self.config.input_wait_timeout);

        // 豆包 2026-05 最新的输入框选择器策略
        const INPUT_SELECTORS: [&str; 6] = [
            // 优先级1: contenteditable div（豆包主要使用）
            "div[contenteditable='true']",
            // 优先级2: 各种可编辑 div
            "[contenteditable='plaintext-only']",
            "[data-slate-editor='true']",
            // 优先级3: textarea
            "textarea",
            // 优先级4: 传统 input
            "input[type='text']",
            "input[placeholder*='说点什么']",
        ];

        while start.elapsed() < timeout {
            for selector in &INPUT_SELECTORS {
                match tab.find_element(selector) {
                    Ok(elem) => {
                        // 验证元素可见
                        if let Ok(is_visible) = elem.is_visible() {
                            if is_visible {
                                println!("[DoubaoClient] ✅ 找到输入框: {}", selector);
                                return Ok(());
                            }
                        }
                    }
                    Err(_) => continue,
                }
            }
            sleep(Duration::from_millis(500));
        }

        // 最后尝试更宽松的选择器
        if tab.find_element("[class*='editor']").is_ok() {
            println!("[DoubaoClient] ✅ 找到宽松匹配输入框");
            return Ok(());
        }

        Err(format!(
            "等待 {} 秒后未找到输入框，超时",
            self.config.input_wait_timeout
        ).into())
    }

    /// 通过 JS 清空并填写输入框
    fn fill_input_via_js(
        &self,
        tab: &Tab,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("[DoubaoClient] 清空并填写输入框...");

        // 通过 JS 找到并清空输入框
        let clear_js = r#"(function() {
            const selectors = [
                'div[contenteditable="true"]',
                'textarea',
                'input[type="text"]',
                '[data-slate-editor="true"]'
            ];

            for (const sel of selectors) {
                const input = document.querySelector(sel);
                if (input) {
                    input.focus();
                    input.innerHTML = '';
                    input.innerText = '';
                    if (input.value !== undefined) input.value = '';

                    // 执行清空命令
                    try { document.execCommand('selectAll', false, null); } catch(e) {}
                    try { document.execCommand('delete', false, null); } catch(e) {}

                    console.log('[DoubaoClient] 输入框已清空');
                    return 'cleared';
                }
            }
            return 'not-found';
        })()"#;

        tab.evaluate(clear_js, false)?;
        sleep(Duration::from_millis(500));

        // 通过 JS 输入消息（更可靠）
        let escaped_message = message
            .replace('\\', "\\\\")
            .replace('\'', "\\'")
            .replace('\n', "\\n");

        let type_js = format!(r#"(function() {{
            const selectors = [
                'div[contenteditable="true"]',
                'textarea',
                'input[type="text"]',
                '[data-slate-editor="true"]'
            ];

            for (const sel of selectors) {{
                const input = document.querySelector(sel);
                if (input) {{
                    input.focus();
                    input.innerText = '{}';
                    input.dispatchEvent(new InputEvent('input', {{
                        bubbles: true,
                        cancelable: true,
                        inputType: 'insertText',
                        data: '{}'
                    }}));
                    console.log('[DoubaoClient] 消息已输入（{} 字符）');
                    return 'typed';
                }}
            }}
            return 'not-found';
        }})()"#, escaped_message, escaped_message, message.len());

        let result = tab.evaluate(&type_js, false)?;
        println!("[DoubaoClient] 输入结果: {:?}", result.value);

        // 备选：使用 headless_chrome 的 type_into
        if result.value.is_none() || result.value.as_ref().is_some_and(|v| v.to_string() == "not-found") {
            println!("[DoubaoClient] JS 输入失败，尝试 headless_chrome type_into...");
            let selectors = [
                "div[contenteditable='true']",
                "textarea",
                "input[type='text']",
            ];
            for sel in &selectors {
                if let Ok(elem) = tab.find_element(sel) {
                    if elem.is_visible().is_ok_and(|v| v) {
                        elem.click()?;
                        sleep(Duration::from_millis(300));
                        elem.type_into(message)?;
                        println!("[DoubaoClient] ✅ type_into 成功");
                        break;
                    }
                }
            }
        }

        sleep(Duration::from_secs(1));
        Ok(())
    }

    /// 发送消息
    fn send_message(&self, tab: &Tab) -> Result<(), Box<dyn std::error::Error>> {
        println!("[DoubaoClient] 发送消息...");

        // 豆包 2026-05 最新的发送按钮选择器策略
        let send_js = r#"(function() {
            console.log('[DoubaoClient] 开始发送流程');

            // === 方法 1: 专用发送按钮 ===
            const sendButtonSelectors = [
                // 优先：明确标注发送的按钮
                'button[type="submit"]',
                'button[class*="send"]',
                'button[class*="Send"]',
                'button[class*="submit"]',
                'button[aria-label*="发送"]',
                'button[aria-label*="send"]',
                'button[aria-label*="Send"]',
                '[class*="send-button"]',
                '[class*="sendBtn"]',
                '.chat-send-btn',
                '.input-send-btn',
                // 豆包特有
                'button[data-__test-id="send"]',
                'button[class*="chat-input"]',
            ];

            for (const sel of sendButtonSelectors) {
                const btn = document.querySelector(sel);
                if (btn && btn.offsetParent !== null) {
                    console.log('[DoubaoClient] 找到发送按钮:', sel);
                    btn.click();
                    return 'send:button:' + sel;
                }
            }

            // === 方法 2: 文本匹配发送按钮 ===
            const allButtons = document.querySelectorAll('button');
            for (const btn of allButtons) {
                const text = btn.textContent.trim();
                const aria = btn.getAttribute('aria-label') || '';
                if (
                    text === '发送' || text === '发送消息' ||
                    text === 'Send' || text === 'Submit' ||
                    text.includes('发送') && btn.querySelectorAll('*').length < 5
                ) {
                    if (btn.offsetParent !== null) {
                        console.log('[DoubaoClient] 找到文本按钮:', text);
                        btn.click();
                        return 'send:text:' + text;
                    }
                }
            }

            // === 方法 3: SVG 发送图标按钮 ===
            try {
                // 查找发送相关的 SVG 按钮
                const svgBtns = document.querySelectorAll('button svg');
                for (const svg of svgBtns) {
                    // 检查 SVG 是否包含发送/箭头图标特征
                    const path = svg.querySelector('path');
                    const d = path?.getAttribute('d') || '';
                    if (d.includes('M') && d.includes('L') && svg.closest('button')) {
                        const btn = svg.closest('button');
                        if (btn.offsetParent !== null) {
                            console.log('[DoubaoClient] 找到 SVG 发送按钮');
                            btn.click();
                            return 'send:svg';
                        }
                    }
                }
            } catch(e) {
                console.log('[DoubaoClient] SVG 查找失败:', e);
            }

            // === 方法 4: Enter 键发送 ===
            console.log('[DoubaoClient] 尝试 Enter 键发送');
            const inputElem = document.querySelector(
                'div[contenteditable="true"]'
            ) || document.querySelector('textarea');

            if (inputElem) {
                inputElem.focus();

                const events = [
                    new InputEvent('input', { bubbles: true, cancelable: true }),
                    new KeyboardEvent('keydown', { key: 'Enter', code: 'Enter', keyCode: 13, which: 13, bubbles: true, cancelable: true }),
                    new KeyboardEvent('keyup', { key: 'Enter', code: 'Enter', keyCode: 13, which: 13, bubbles: true })
                ];

                for (const evt of events) {
                    inputElem.dispatchEvent(evt);
                }

                return 'send:enter';
            }

            return 'send:failed';
        })()"#;

        match tab.evaluate(send_js, false) {
            Ok(result) => {
                if let Some(val) = result.value {
                    let s = val.to_string();
                    if s.starts_with("send:") {
                        println!("[DoubaoClient] ✅ 消息已发送: {}", s);
                    } else {
                        println!("[DoubaoClient] ⚠ 发送结果: {}", s);
                    }
                }
            }
            Err(e) => {
                println!("[DoubaoClient] ⚠ 发送 JS 执行失败: {}", e);
            }
        }

        sleep(Duration::from_secs(2));
        Ok(())
    }

    /// 等待并获取回复
    fn wait_for_response(&self, tab: &Tab) -> Result<String, Box<dyn std::error::Error>> {
        println!("[DoubaoClient] 开始等待回复（最多 {} 秒）...", self.config.send_timeout);

        let max_wait = self.config.send_timeout;
        let poll_interval = Duration::from_millis(self.config.poll_interval_ms);
        let mut elapsed: u64 = 0;
        let mut last_text = String::new();
        let mut stable_count: usize = 0;
        let mut has_seen_response = false;

        // 豆包 2026-05 回复内容选择器（按优先级）
        const RESPONSE_SELECTORS: [&str; 12] = [
            // 优先级1: 豆包特有的回复容器
            "[class*='bubble']",
            "[class*='message-content']",
            "[class*='chat-message']",
            "[class*='reply-content']",
            // 优先级2: 通用 AI 回复容器
            "[class*='markdown-body']",
            "[class*='prose']",
            ".typo",
            "[class*='response-content']",
            "[class*='assistant-message']",
            // 优先级3: 更通用的选择器
            "[role='article']",
            "[class*='answer']",
            "div[class*='content']",
        ];

        while elapsed < max_wait {
            sleep(poll_interval);
            elapsed += (self.config.poll_interval_ms / 1000) as u64;

            print!("\r[DoubaoClient] 已等待: {} 秒 / {} 秒", elapsed, max_wait);
            std::io::stdout().flush().ok();

            // 尝试获取回复内容
            if let Some(text) = self.extract_response_text(tab, &RESPONSE_SELECTORS) {
                if text.len() > 20 && !text.contains("null") {
                    // 检测新回复
                    if text != last_text {
                        println!("\n[DoubaoClient] 📝 获取到回复（{} 字符）: {}",
                            text.len(),
                            text.chars().take(100).collect::<String>()
                        );
                        last_text = text.clone();
                        stable_count = 0;
                        has_seen_response = true;
                    } else if has_seen_response {
                        // 内容稳定
                        stable_count += 1;
                        println!("\n[DoubaoClient] 内容稳定 {}/{} 次", stable_count, self.config.stable_threshold);

                        if stable_count >= self.config.stable_threshold {
                            println!("[DoubaoClient] ✅ 回复已完成，总计 {} 字符", text.len());
                            return Ok(text);
                        }
                    }
                }
            }

            // 每 10 秒打印一次进度
            if elapsed % 10 == 0 && elapsed > 0 {
                println!("\n[DoubaoClient] ⏳ 仍在等待 AI 回复...");
            }
        }

        println!("\n[DoubaoClient] ⚠ 超时，返回已获取内容");
        if last_text.len() > 20 {
            return Ok(last_text);
        }

        Err(format!("等待 {} 秒后未获取到完整回复", max_wait).into())
    }

    /// 从页面提取回复文本
    fn extract_response_text<'a>(
        &self,
        tab: &Tab,
        selectors: &[&'a str],
    ) -> Option<String> {
        let js_code = format!(r#"(function() {{
            const targetSelectors = {};
            let bestText = '';
            let bestLen = 0;

            for (const sel of targetSelectors) {{
                try {{
                    const els = document.querySelectorAll(sel);
                    if (els.length === 0) continue;

                    // 优先查找最新/最后的回复
                    for (let i = els.length - 1; i >= 0; i--) {{
                        const el = els[i];
                        const text = (el.innerText || el.textContent || '').trim();

                        // 过滤无效内容
                        if (text.length > 20 &&
                            !text.includes('null') &&
                            !text.includes('undefined') &&
                            text.length > bestLen &&
                            !text.includes(window.location.host)
                        ) {{
                            // 检查是否在可见区域内
                            const rect = el.getBoundingClientRect();
                            if (rect.height > 0 && rect.width > 0) {{
                                bestLen = text.length;
                                bestText = text;
                                break;
                            }}
                        }}
                    }}

                    if (bestLen > 0) break;
                }} catch(e) {{
                    // 继续尝试下一个选择器
                }}
            }}

            // 备选：查找所有 div 中最长的文本
            if (!bestText) {{
                const allDivs = document.querySelectorAll('div');
                for (let i = Math.max(0, allDivs.length - 100); i < allDivs.length; i++) {{
                    try {{
                        const text = allDivs[i].innerText.trim();
                        if (text.length > bestLen && text.length > 100) {{
                            bestLen = text.length;
                            bestText = text;
                        }}
                    }} catch(e) {{}}
                }}
            }}

            return bestText;
        }})()"#,
            serde_json::to_string(selectors).unwrap_or_default()
        );

        if let Ok(result) = tab.evaluate(&js_code, false) {
            if let Some(value) = result.value {
                let text = value.to_string().trim_matches('"').to_string();
                if !text.is_empty() && text.len() > 20 {
                    return Some(text);
                }
            }
        }
        None
    }

    /// 关闭浏览器
    pub fn close(self) {
        println!("[DoubaoClient] 关闭浏览器");
        drop(self.browser);
    }
}

// ============ CLI 测试入口 ============

/// CLI 测试程序
pub fn run_cli_test(message: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "═".repeat(70));
    println!("  豆包 Web 版客户端 - 自动化测试");
    println!("{}\n", "═".repeat(70));

    let message = message.unwrap_or("推荐一个 AI 写作工具");

    let config = DoubaoConfig {
        headless: false,
        launch_options: LaunchOptions {
            headless: false,
            sandbox: false,
            window_size: Some((1280, 900)),
            ..Default::default()
        },
        input_wait_timeout: 30,
        send_timeout: 180,
        poll_interval_ms: 2000,
        stable_threshold: 6,
        max_retries: 2,
    };

    let client = DoubaoClient::with_config(config)?;
    let response = client.chat(message)?;

    println!("\n{}", "═".repeat(70));
    println!("  豆包回复:");
    println!("{}", "═".repeat(70));
    println!("\n{}\n", response);
    println!("回复长度: {} 字符", response.len());

    client.close();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_cli_test(None)
}
