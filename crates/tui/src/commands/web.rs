// /web 命令 - Web LLM 集成
// 用法: /web qianwen "问题" 或 /web doubao "问题"
// 更新时间: 2026-05-17 00:15

use crate::commands::{CommandResult, App};
use crate::localization::Locale;
use crate::tui::history::HistoryCell;

/// 执行 /web 命令
pub fn handle_web_command(app: &mut App, args: &str) -> CommandResult {
    // 解析参数: <platform> [--image <path>] <message>
    let mut args_iter = args.split_whitespace().peekable();
    let platform = match args_iter.next() {
        Some(p) => p.to_lowercase(),
        None => {
            return CommandResult::error("用法: /web <qianwen|doubao> [--image <图片路径>] \"问题\"\n示例: /web qianwen \"推荐一个AI工具\"\n带图片: /web qianwen --image C:\\test.jpg \"这张图片里有什么\"");
        }
    };
    
    let mut image_path: Option<String> = None;
    let mut message_parts = Vec::new();
    
    while let Some(part) = args_iter.next() {
        if part == "--image" || part == "-i" {
            if let Some(path) = args_iter.next() {
                image_path = Some(path.to_string());
            }
        } else {
            message_parts.push(part);
        }
    }
    
    let message = message_parts.join(" ");
    
    if message.is_empty() {
        return CommandResult::error("问题不能为空\n用法: /web <qianwen|doubao> [--image <图片路径>] \"问题\"");
    }
    
    // 验证平台
    match platform.as_str() {
        "qianwen" | "千问" => {
            execute_web_query(app, "qianwen", &message, image_path.as_deref())
        }
        "doubao" | "豆包" => {
            execute_web_query(app, "doubao", &message, image_path.as_deref())
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
fn execute_web_query(app: &mut App, platform: &str, message: &str, image_path: Option<&str>) -> CommandResult {
    use std::time::Instant;
    
    let platform_name = match platform {
        "qianwen" => "千问",
        "doubao" => "豆包",
        _ => platform,
    };
    
    let start_time = Instant::now();
    
    // 显示加载状态
    app.is_loading = true;
    
    // 获取或创建 Web LLM 客户端
    let client_key = platform.to_string();
    
    // 如果客户端不存在，创建新的
    if !app.web_llm_clients.contains_key(&client_key) {
        use deepseek_aiwebllm::web_llm_client::{WebLlmClient, WebLlmPlatform};
        
        let web_platform = match platform {
            "qianwen" => WebLlmPlatform::Qianwen,
            "doubao" => WebLlmPlatform::Doubao,
            _ => {
                app.is_loading = false;
                return CommandResult::error(format!("未知平台: {}", platform));
            }
        };
        
        match WebLlmClient::new(web_platform) {
            Ok(client) => {
                app.web_llm_clients.insert(client_key.clone(), client);
            }
            Err(e) => {
                app.is_loading = false;
                return CommandResult::error(format!("创建客户端失败: {}", e));
            }
        }
    }
    
    // 构建用户消息显示内容
    let user_content = if let Some(img) = image_path {
        format!("[{}] [图片: {}] {}", platform_name, img, message)
    } else {
        format!("[{}] {}", platform_name, message)
    };
    
    // 使用存储的客户端发送消息
    let result = if let Some(client) = app.web_llm_clients.get(&client_key) {
        match client.send_message(message, image_path, 90) {
            Ok(response) => Some(Ok(response)),
            Err(e) => Some(Err(e.to_string())),
        }
    } else {
        None
    };
    
    app.is_loading = false;
    let elapsed = start_time.elapsed().as_secs();
    
    // 根据结果添加到对话流
    match result {
        Some(Ok(response)) => {
            // 成功：添加用户问题和 AI 回复
            app.add_message(HistoryCell::User {
                content: user_content,
            });
            
            app.add_message(HistoryCell::Assistant {
                content: format!(
                    "🌐 **{} 回复** (耗时: {}秒)\n\n{}",
                    platform_name,
                    elapsed,
                    response
                ),
                streaming: false,
            });
            
            CommandResult::message(format!("已从 {} 获取回复", platform_name))
        }
        Some(Err(error)) => {
            // 失败：添加错误消息
            app.add_message(HistoryCell::System {
                content: format!(
                    "❌ 向 {} 提问失败\n问题: {}\n\n错误: {}\n\n提示: 请确保已登录 {} 网站",
                    platform_name,
                    user_content,
                    error,
                    platform_name
                ),
            });
            
            CommandResult::error(format!("查询失败: {}", error))
        }
        None => {
            CommandResult::error("未知错误")
        }
    }
}

/// 获取 /web 命令的帮助信息
pub fn web_command_help(locale: Locale) -> String {
    format!(
        "用法: /web <平台> [--image <图片路径>] \"问题\"\n\n\
         支持的平台:\n\
         - qianwen (千问): https://www.qianwen.com/\n\
         - doubao (豆包): https://www.doubao.com/\n\n\
         示例:\n\
         /web qianwen \"推荐一个AI训练数据工具\"\n\
         /web qianwen --image C:\\test.jpg \"这张图片里有什么\"\n\
         /web doubao \"解释量子计算\""
    )
}
