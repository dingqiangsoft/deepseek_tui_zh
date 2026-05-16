// /web 命令 - Web LLM 集成
// 用法: /web qianwen "问题" 或 /web doubao "问题"

use crate::commands::{CommandResult, App};
use crate::localization::{Locale, MessageId, tr};

/// 执行 /web 命令
pub fn handle_web_command(app: &mut App, args: &str) -> CommandResult {
    // 解析参数: <platform> <message>
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    
    if parts.len() < 2 {
        return CommandResult::error("用法: /web <qianwen|doubao> \"问题\"\n示例: /web qianwen \"推荐一个AI工具\"");
    }
    
    let platform = parts[0].to_lowercase();
    let message = parts[1].trim();
    
    if message.is_empty() {
        return CommandResult::error("问题不能为空");
    }
    
    // 验证平台
    match platform.as_str() {
        "qianwen" | "千问" => {
            execute_web_query(app, "qianwen", message)
        }
        "doubao" | "豆包" => {
            execute_web_query(app, "doubao", message)
        }
        _ => {
            CommandResult::error(format!(
                "未知平台: {}\n支持的平台: qianwen (千问), doubao (豆包)",
                platform
            ))
        }
    }
}

/// 执行 Web LLM 查询
fn execute_web_query(app: &mut App, platform: &str, message: &str) -> CommandResult {
    use std::thread;
    
    let platform_name = match platform {
        "qianwen" => "千问",
        "doubao" => "豆包",
        _ => platform,
    };
    
    // 显示开始消息
    let start_msg = format!("正在向 {} 发送问题...", platform_name);
    
    // 在后台线程中执行 Web LLM 调用
    let msg = message.to_string();
    let plat = platform.to_string();
    
    thread::spawn(move || {
        use deepseek_aiwebllm::web_llm_client::{WebLlmClient, WebLlmPlatform};
        
        let platform = match plat.as_str() {
            "qianwen" => WebLlmPlatform::Qianwen,
            "doubao" => WebLlmPlatform::Doubao,
            _ => return,
        };
        
        match WebLlmClient::new(platform) {
            Ok(client) => {
                match client.send_message(&msg, 60) {
                    Ok(response) => {
                        // TODO: 将回复发送回 TUI
                        println!("{} 回复:\n{}", platform.name(), response);
                    }
                    Err(e) => {
                        eprintln!("获取回复失败: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("创建客户端失败: {}", e);
            }
        }
    });
    
    CommandResult::message(format!(
        "正在向 {} 提问...\n问题: {}\n\n浏览器已启动，请稍候回复。",
        platform_name,
        message
    ))
}

/// 获取 /web 命令的帮助信息
pub fn web_command_help(locale: Locale) -> String {
    format!(
        "用法: /web <平台> \"问题\"\n\n\
         支持的平台:\n\
         - qianwen (千问): https://www.qianwen.com/\n\
         - doubao (豆包): https://www.doubao.com/\n\n\
         示例:\n\
         /web qianwen \"推荐一个AI训练数据工具\"\n\
         /web doubao \"解释量子计算\""
    )
}
