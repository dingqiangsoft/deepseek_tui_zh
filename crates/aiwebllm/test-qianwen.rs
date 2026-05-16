// 测试千问 Web API 连接
// 用法: cargo run --bin test-qianwen

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize)]
struct QianwenResponse {
    output: Output,
    request_id: String,
}

#[derive(Debug, Deserialize)]
struct Output {
    text: String,
    finish_reason: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 测试千问 Web API 连接...\n");

    // 从环境变量获取 API Key
    let api_key = env::var("QIANWEN_API_KEY")
        .expect("请设置环境变量 QIANWEN_API_KEY\n获取方式: https://dashscope.console.aliyun.com/");

    let client = Client::new();

    // 构建请求
    let body = serde_json::json!({
        "model": "qwen-turbo",
        "input": {
            "messages": [
                {
                    "role": "user",
                    "content": "你好，这是一个测试。请简短回复"测试成功"。"
                }
            ]
        },
        "parameters": {
            "result_format": "message"
        }
    });

    println!("📤 发送请求到千问 API...");
    println!("   模型: qwen-turbo");
    println!("   消息: 你好，这是一个测试。请简短回复\"测试成功\"。\n");

    let response = client
        .post("https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("X-DashScope-Version", "2024-10-15")
        .json(&body)
        .send()
        .await?;

    let status = response.status();
    println!("📥 收到响应: HTTP {}", status);

    if status.is_success() {
        let text = response.text().await?;
        println!("\n✅ 响应成功！\n");
        println!("原始响应:\n{}", text);

        // 尝试解析
        match serde_json::from_str::<QianwenResponse>(&text) {
            Ok(parsed) => {
                println!("\n🎯 解析成功！");
                println!("回复内容: {}", parsed.output.text);
                println!("完成原因: {}", parsed.output.finish_reason);
            }
            Err(e) => {
                println!("\n⚠️  JSON 解析失败: {}", e);
            }
        }
    } else {
        let error_text = response.text().await?;
        println!("\n❌ 请求失败！\n");
        println!("错误响应:\n{}", error_text);
    }

    Ok(())
}
