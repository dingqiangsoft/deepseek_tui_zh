# 测试 aiwebllm 模块

## 前置准备

### 1. 千问 (Qianwen) 测试

#### 获取 API Key
1. 访问: https://dashscope.console.aliyun.com/
2. 登录阿里云账号
3. 创建 API Key
4. 设置环境变量:

**PowerShell (Windows):**
```powershell
$env:QIANWEN_API_KEY = "你的API密钥"
```

**CMD (Windows):**
```cmd
set QIANWEN_API_KEY=你的API密钥
```

**Linux/Mac:**
```bash
export QIANWEN_API_KEY="你的API密钥"
```

#### 运行测试

**测试 1: API 连接测试**
```bash
cd crates/aiwebllm
cargo run --example test_qianwen_web
```

**测试 2: 完整功能测试**
```bash
cd crates/aiwebllm
cargo test --lib
```

### 2. 豆包 (Doubao) 测试

豆包目前主要通过网页方式访问，需要：
1. 手动登录 https://www.doubao.com
2. 提取 Cookie/Session Token
3. 配置到 `config/web_llm.toml`

## 快速测试脚本

### Windows PowerShell 一键测试

```powershell
# 设置 API Key
$env:QIANWEN_API_KEY = "你的API密钥"

# 运行测试
cd crates/aiwebllm
cargo run --example test_qianwen_web
```

### 使用提供的测试脚本

```powershell
cd crates/aiwebllm
.\test-qianwen.ps1
```

## 预期输出

成功时应该看到:
```
🧪 测试千问 Web API 连接...

📤 发送请求到千问 API...
   模型: qwen-turbo
   消息: 你好，这是一个测试。请简短回复"测试成功"。

📥 收到响应: HTTP 200

✅ 响应成功！

🎯 解析成功！
回复内容: 测试成功！
完成原因: stop
```

## 常见问题

### 1. 未设置 API Key
```
错误: 请设置环境变量 QIANWEN_API_KEY
```
**解决**: 按照上面的步骤设置环境变量

### 2. API Key 无效
```
HTTP 401: Invalid API Key
```
**解决**: 检查 API Key 是否正确，确保没有多余空格

### 3. 网络问题
```
请求超时或连接失败
```
**解决**: 
- 检查网络连接
- 确认可以访问 `dashscope.aliyuncs.com`
- 如有代理，设置 `HTTP_PROXY` 环境变量

## 高级测试

### 测试 Web LLM 客户端集成

```bash
# 编译整个项目
cargo build --workspace

# 运行 TUI 并测试 /web 命令
cargo run --bin deepseek
```

在 TUI 中使用:
```
/web 你好，这是一个测试
```

### 单元测试

```bash
cargo test -p deepseek-aiwebllm
```

### 集成测试

```bash
cargo test --workspace --all-features
```
