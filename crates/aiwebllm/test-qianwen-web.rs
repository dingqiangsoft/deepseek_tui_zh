// 千问网页版自动化测试
// 使用 headless_chrome 控制浏览器访问千问网页

use headless_chrome::{Browser, LaunchOptions};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 测试千问网页版自动化...\n");

    // 千问聊天页面地址
    let chat_url = "https://www.qianwen.com/chat/a7daf5f3403c4f41a7f5bd3bcf391cf5";
    
    println!("📍 目标地址: {}", chat_url);
    println!("⚠️  请确保您已经在浏览器中登录了千问账号！\n");

    // 启动浏览器（有头模式，方便调试）
    println!("🚀 启动浏览器...");
    let browser = Browser::new(LaunchOptions {
        headless: false, // 设为 false 可以看到浏览器界面
        sandbox: false,
        ..Default::default()
    })?;

    // 创建新标签页
    let tab = browser.new_tab()?;
    println!("✅ 浏览器启动成功！\n");

    // 导航到千问页面
    println!("📡 导航到千问聊天页面...");
    tab.navigate_to(chat_url)?;
    
    // 等待页面加载
    println!("⏳ 等待页面加载...");
    tab.wait_until_navigated()?;
    sleep(Duration::from_secs(3));

    // 检查页面标题
    if let Ok(title) = tab.get_title() {
        println!("📄 页面标题: {}", title);
    }

    // 获取当前 URL（可能被重定向到登录页）
    if let Ok(current_url) = tab.get_url() {
        println!("🔗 当前 URL: {}", current_url);
        
        if current_url.contains("login") || current_url.contains("signin") {
            println!("\n⚠️  检测到登录页面！");
            println!("请先在浏览器中手动登录千问账号，然后重新运行测试。");
        }
    }

    // 尝试查找输入框
    println!("\n🔍 查找消息输入框...");
    
    // 常见的输入框选择器
    let input_selectors = vec![
        "textarea",
        "input[type='text']",
        "[contenteditable='true']",
        ".chat-input",
        "#chat-input",
    ];

    let mut input_found = false;
    for selector in input_selectors {
        match tab.find_element(selector) {
            Ok(element) => {
                println!("✅ 找到输入框: {}", selector);
                input_found = true;
                
                // 尝试输入测试消息
                println!("📝 输入测试消息...");
                element.click()?;
                element.type_into("你好，这是一个自动化测试")?;
                sleep(Duration::from_secs(1));
                
                println!("💡 消息已输入，请在浏览器中手动点击发送按钮");
                println!("⏳ 等待 10 秒供您操作...");
                sleep(Duration::from_secs(10));
                
                // 尝试获取回复
                println!("\n🔍 查找 AI 回复...");
                let response_selectors = vec![
                    ".message-content",
                    ".ai-response",
                    "[data-role='assistant']",
                    ".assistant-message",
                ];
                
                for resp_selector in response_selectors {
                    if let Ok(resp_element) = tab.find_element(resp_selector) {
                        if let Ok(text) = resp_element.get_inner_text() {
                            if !text.is_empty() {
                                println!("✅ 找到回复 (选择器: {})", resp_selector);
                                println!("\n📥 AI 回复内容:");
                                println!("{}", text);
                                break;
                            }
                        }
                    }
                }
                
                break;
            }
            Err(_) => continue,
        }
    }

    if !input_found {
        println!("❌ 未找到输入框！");
        println!("💡 可能需要手动在浏览器中输入消息");
        println!("⏳ 保持浏览器打开 30 秒供您测试...");
        sleep(Duration::from_secs(30));
    }

    println!("\n✅ 测试完成！");
    println!("💡 浏览器将保持打开，关闭窗口结束程序");
    
    // 保持程序运行，让用户可以继续操作
    sleep(Duration::from_secs(60));

    Ok(())
}
