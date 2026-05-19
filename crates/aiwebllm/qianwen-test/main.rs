use headless_chrome::{Browser, LaunchOptions};
use std::{thread::sleep, time::Duration, io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 千问网页版测试 ===\n");
    
    let chat_url = "https://www.qianwen.com/";
    
    println!("步骤 1: 启动浏览器...");
    let browser = Browser::new(LaunchOptions {
        headless: false,
        sandbox: false,
        window_size: Some((1280, 800)),
        ..Default::default()
    })?;
    
    let tab = browser.new_tab()?;
    println!("  ✓ 浏览器已启动\n");
    
    println!("步骤 2: 访问千问首页...");
    println!("  URL: {}\n", chat_url);
    tab.navigate_to(chat_url)?;
    tab.wait_until_navigated()?;
    sleep(Duration::from_secs(5));
    
    if let Ok(title) = tab.get_title() {
        println!("  ✓ 页面标题: {}\n", title);
    }
    
    // 检查是否需要登录
    let current_url = tab.get_url();
    if current_url.contains("login") || current_url.contains("signin") || current_url.contains("auth") {
        println!("⚠ 检测到登录页面！");
        println!("请先在浏览器中手动登录千问账号");
        println!("等待 60 秒供您登录...\n");
        
        for i in (1..=60).rev() {
            print!("\r  剩余: {} 秒", i);
            std::io::stdout().flush().ok();
            sleep(Duration::from_secs(1));
        }
        println!("\n");
        
        // 重新检查是否登录成功
        let new_url = tab.get_url();
        if new_url.contains("login") || new_url.contains("signin") {
            println!("❌ 仍未检测到登录状态，程序退出");
            return Ok(());
        }
        println!("✓ 检测到登录成功！\n");
    } else {
        println!("✓ 已访问页面（可能已登录）\n");
    }
    
    println!("步骤 3: 向千问发送消息...");
    let message = "推荐一个AI训练数据处理功能强大的开源工具";
    println!("  问题: {}\n", message);
    
    // 查找输入框
    let input_elem = tab.find_element("[contenteditable='true']")
        .or_else(|_| tab.find_element("textarea"))
        .expect("未找到输入框");
    
    println!("  ✓ 找到输入框");
    input_elem.click()?;
    sleep(Duration::from_millis(500));
    
    input_elem.type_into(message)?;
    println!("  ✓ 已输入消息\n");
    sleep(Duration::from_secs(1));
    
    // 查找并点击发送按钮
    println!("步骤 4: 发送消息...");
    
    // 使用 JavaScript 查找并点击发送按钮
    let js_click = r#"(function() {
        // 尝试多种选择器查找发送按钮
        const selectors = [
            'button[type="submit"]',
            '.send-button',
            '[aria-label="发送"]',
            'button.send-btn',
            '.chat-input button',
            'button[class*="send"]',
            'button[class*="Submit"]'
        ];
        
        for (const sel of selectors) {
            const btn = document.querySelector(sel);
            if (btn && btn.offsetParent !== null) { // 确保按钮可见
                btn.click();
                return 'clicked:' + sel;
            }
        }
        
        // 如果没找到，尝试查找包含"发送"文本的按钮
        const allButtons = document.querySelectorAll('button');
        for (const btn of allButtons) {
            if (btn.textContent.includes('发送') && btn.offsetParent !== null) {
                btn.click();
                return 'clicked:text[发送]';
            }
        }
        
        return 'not_found';
    })()"#;
    
    match tab.evaluate(js_click, false) {
        Ok(result) => {
            if let Some(value) = result.value {
                let result_str = value.to_string().trim_matches('"').to_string();
                if result_str.starts_with("clicked:") {
                    println!("  ✓ 已点击发送按钮 ({})", &result_str[8..]);
                    println!("  ✓ 消息已发送\n");
                } else {
                    println!("  ⚠ 未找到发送按钮，尝试按 Enter 键...");
                    // 使用 JavaScript 模拟 Enter 键
                    let _ = tab.evaluate(r#"(function() {
                        const event = new KeyboardEvent('keydown', {
                            key: 'Enter',
                            code: 'Enter',
                            keyCode: 13,
                            bubbles: true
                        });
                        document.activeElement.dispatchEvent(event);
                    })()"#, false);
                    println!("  ✓ 已按 Enter 发送\n");
                }
            }
        }
        Err(e) => {
            println!("  ⚠ JavaScript 执行失败: {}", e);
            println!("  尝试直接按 Enter 键...");
            let _ = tab.evaluate(r#"(function() {
                const event = new KeyboardEvent('keydown', { key: 'Enter', bubbles: true });
                document.activeElement.dispatchEvent(event);
            })()"#, false);
            println!("  ✓ 已发送\n");
        }
    }
    
    // 等待页面响应
    sleep(Duration::from_secs(2));
    
    // 等待 AI 回复
    println!("步骤 5: 等待千问回复...");
    println!("  预计等待 10-30 秒\n");
    
    let max_wait = 60;
    let mut elapsed = 0;
    
    while elapsed < max_wait {
        sleep(Duration::from_secs(2));
        elapsed += 2;
        print!("\r  已等待: {} 秒 / {} 秒", elapsed, max_wait);
        std::io::stdout().flush().ok();
        
        // 使用 JavaScript 获取页面中的所有消息内容
        if elapsed % 4 == 0 {
            if let Ok(result) = tab.evaluate("(function() {
                // 尝试多种选择器
                const selectors = [
                    '[class*=\"message\"]',
                    '[class*=\"chat\"]',
                    '[class*=\"response\"]',
                    '[class*=\"answer\"]',
                    'div[role=\"article\"]',
                    '.markdown-body'
                ];
                
                let allTexts = [];
                selectors.forEach(sel => {
                    document.querySelectorAll(sel).forEach(el => {
                        const text = el.innerText.trim();
                        if (text.length > 100 && !text.includes('向千问提问')) {
                            allTexts.push(text);
                        }
                    });
                });
                
                return allTexts[0] || '';
            })()", false) {
                if let Some(value) = result.value {
                    let text = value.to_string().trim_matches('"').to_string();
                    if text.len() > 100 && !text.contains("null") {
                        println!("\n\n✓ 获取到千问的回复！\n");
                        println!("{}\n", "═".repeat(70));
                        println!("{}", text);
                        println!("\n{}\n", "═".repeat(70));
                        println!("回复长度: {} 字符", text.len());
                        
                        sleep(Duration::from_secs(3));
                        println!("\n✓ 测试完成！");
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
