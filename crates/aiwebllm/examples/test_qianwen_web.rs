// test_qianwen_web.rs - 测试千问网页版连接
use headless_chrome::{Browser, LaunchOptions};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 千问网页版测试 ===\n");
    
    let chat_url = "https://www.qianwen.com/";
    
    println!("步骤 1: 启动浏览器...");
    let browser = Browser::new(LaunchOptions {
        headless: false,  // 显示浏览器窗口
        sandbox: false,
        window_size: Some((1280, 800)),
        ..Default::default()
    })?;
    
    let tab = browser.new_tab()?;
    println!("  ✓ 浏览器已启动\n");
    
    println!("步骤 2: 访问千问页面...");
    println!("  URL: {}", chat_url);
    tab.navigate_to(chat_url)?;
    tab.wait_until_navigated()?;
    sleep(Duration::from_secs(3));
    
    if let Ok(title) = tab.get_title() {
        println!("  ✓ 页面标题: {}\n", title);
    }
    
    let current_url = tab.get_url();
    if current_url.contains("login") || current_url.contains("signin") {
        println!("⚠ 检测到登录页面");
        println!("请先在浏览器中登录千问账号\n");
    } else {
        println!("  ✓ 成功访问聊天页面");
        println!("  当前 URL: {}\n", current_url);
    }
    
    println!("步骤 3: 向千问发送消息...");
    println!("  问题: 推荐一个AI训练数据处理功能强大的开源工具\n");
    
    // 查找输入框
    let input_elem = tab.find_element("[contenteditable='true']")
        .or_else(|_| tab.find_element("textarea"))
        .expect("未找到输入框");
    
    println!("  ✓ 找到输入框");
    
    // 点击输入框
    input_elem.click()?;
    sleep(Duration::from_millis(500));
    
    // 输入消息
    let message = "推荐一个AI训练数据处理功能强大的开源工具";
    input_elem.type_into(message)?;
    println!("  ✓ 已输入消息: {}", message);
    sleep(Duration::from_secs(1));
    
    // 查找并点击发送按钮
    println!("\n步骤 4: 发送消息...");
    
    // 尝试多种发送按钮选择器
    let mut sent = false;
    let send_selectors = vec![
        "button[type='submit']",
        ".send-button",
        "[aria-label='发送']",
        "button.send-btn",
        ".chat-input button",
        "button:last-child",
    ];
    
    for selector in send_selectors {
        if let Ok(send_btn) = tab.find_element(selector) {
            println!("  ✓ 找到发送按钮: {}", selector);
            send_btn.click()?;
            println!("  ✓ 已点击发送\n");
            sent = true;
            break;
        }
    }
    
    // 如果没找到发送按钮，尝试按 Enter 键
    if !sent {
        println!("  未找到发送按钮，尝试按 Enter 键...");
        
        // 使用 JavaScript 触发发送
        let _ = tab.evaluate("(function() {
            const event = new KeyboardEvent('keydown', { key: 'Enter', code: 'Enter', bubbles: true });
            document.activeElement.dispatchEvent(event);
        })()", false);
        
        println!("  ✓ 已发送\n");
    }
    
    // 等待 AI 回复
    println!("步骤 5: 等待千问回复...");
    println!("  预计等待 10-30 秒\n");
    
    let max_wait = 60; // 最长等待 60 秒
    let mut elapsed = 0;
    
    while elapsed < max_wait {
        sleep(Duration::from_secs(2));
        elapsed += 2;
        print!("\r  已等待: {} 秒 / {} 秒", elapsed, max_wait);
        use std::io::Write;
        std::io::stdout().flush().ok();
        
        // 尝试获取回复内容 - 使用 JavaScript 直接获取
        let response_selectors = vec![
            ".message-content",
            ".assistant-message",
            "[data-role='assistant']",
            ".answer-content",
            ".response-text",
            "div.markdown-body",
            ".chat-message",
        ];
        
        for selector in response_selectors {
            if let Ok(elements) = tab.find_elements(selector) {
                if !elements.is_empty() {
                    if let Ok(text) = elements.last().unwrap().get_inner_text() {
                        if text.len() > 50 && !text.contains("向千问提问") {
                            // 找到有效回复
                            println!("\n\n✓ 获取到回复！\n");
                            println!("{} {}\n", "═".repeat(60), "千问的回复");
                            println!("{}", text);
                            println!("{}\n", "═".repeat(60));
                            println!("回复长度: {} 字符", text.len());
                            
                            // 再等待几秒确保内容完整
                            sleep(Duration::from_secs(3));
                            println!("\n测试完成！");
                            return Ok(());
                        }
                    }
                }
            }
        }
        
        // 备选方案：使用 JavaScript 获取所有消息
        if elapsed % 10 == 0 {
            if let Ok(result) = tab.evaluate("(function() {
                const messages = document.querySelectorAll('[class*=\"message\"], [class*=\"chat\"], [class*=\"response\"]');
                let texts = [];
                messages.forEach(m => {
                    if (m.innerText.length > 50) {
                        texts.push(m.innerText.trim());
                    }
                });
                return texts.join('\\n---\\n');
            })()", false) {
                if let Some(value) = result.value {
                    let text = value.to_string();
                    if text.len() > 100 && !text.contains("null") {
                        println!("\n\n✓ 通过 JavaScript 获取到回复！\n");
                        println!("{} {}\n", "═".repeat(60), "千问的回复");
                        println!("{}", text.trim_matches('"'));
                        println!("{}\n", "═".repeat(60));
                        println!("测试完成！");
                        return Ok(());
                    }
                }
            }
        }
    }
    
    println!("\n\n⚠ 超时未获取到完整回复");
    println!("请在浏览器中查看千问的回复");
    Ok(())
}
