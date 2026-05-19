# 🧪 aiwebllm 测试指南

## 快速开始

### 1️⃣ 运行单元测试（无需 API Key）

```powershell
cd crates/aiwebllm
cargo test --lib
```

**预期输出:**
```
running 1 test
test session::tests::test_token_usage_reset ... ok

test result: ok. 1 passed; 0 failed; 0 ignored
```

### 2️⃣ 测试千问 API（需要 API Key）

#### 步骤 1: 获取 API Key

1. 访问: https://dashscope.console.aliyun.com/
2. 登录阿里云账号
3. 创建 API Key

#### 步骤 2: 设置环境变量

**PowerShell:**
```powershell
$env:QIANWEN_API_KEY = "你的API密钥"
```

**CMD:**
```cmd
set QIANWEN_API_KEY=你的API密钥
```

#### 步骤 3: 运行测试

```powershell
cd crates/aiwebllm
cargo run --example test_qianwen_web
```

**预期输出:**
```
🧪 测试千问 Web API 连接...

📤 发送请求到千问 API...
   模型: qwen-turbo
   消息: 你好，这是一个测试

📥 收到响应: HTTP 200

✅ 响应成功！
回复内容: 测试成功！
```

### 3️⃣ 测试千问网页版（需要浏览器）

```powershell
cd crates/aiwebllm
cargo run --example test_qianwen_web
```

这个测试会：
- ✅ 启动 Chrome 浏览器
- ✅ 访问 https://www.qianwen.com/
- ✅ 自动输入消息并发送
- ✅ 等待并获取 AI 回复

**注意:** 需要先在浏览器中登录千问账号

### 4️⃣ 测试豆包（Doubao）

豆包目前通过网页方式访问：

1. **手动登录**: 访问 https://www.doubao.com 并登录
2. **提取 Cookie**: 从浏览器开发者工具中获取
3. **配置**: 编辑 `config/web_llm.toml`

```toml
[doubao]
enabled = true
session_token = "你的Cookie中的token"
```

### 5️⃣ 在 TUI 中测试 Web LLM

```powershell
# 编译整个项目
cargo build --release

# 运行 TUI
./target/release/deepseek
```

在 TUI 中使用 `/web` 命令：
```
/web 你好，请介绍一下自己
```

## 使用快速测试脚本

```powershell
cd crates/aiwebllm
.\quick-test.ps1
```

这个脚本会自动：
- ✅ 检查编译
- ✅ 运行单元测试
- ✅ 如果有 API Key，运行 API 测试

## 常见问题

### Q1: 未设置 API Key
```
错误: 请设置环境变量 QIANWEN_API_KEY
```

**解决:**
```powershell
$env:QIANWEN_API_KEY = "sk-xxxxxxxxxxxx"
```

### Q2: API Key 无效
```
HTTP 401: Invalid API Key
```

**解决:**
- 检查 API Key 是否正确
- 确保没有多余空格
- 确认账号状态正常

### Q3: 网络超时
```
请求超时或连接失败
```

**解决:**
```powershell
# 检查网络
ping dashscope.aliyuncs.com

# 如有代理，设置
$env:HTTP_PROXY = "http://proxy:port"
$env:HTTPS_PROXY = "http://proxy:port"
```

### Q4: 浏览器测试失败
```
未找到输入框 / 未找到发送按钮
```

**解决:**
- 确保 Chrome 浏览器已安装
- 手动访问千问网页确认可以正常访问
- 检查页面结构是否变化

## 测试清单

- [ ] 编译通过 (`cargo check`)
- [ ] 单元测试通过 (`cargo test --lib`)
- [ ] API 连接测试（需要 API Key）
- [ ] 网页版测试（需要登录）
- [ ] TUI 集成测试（`/web` 命令）

## 高级测试

### 运行所有测试

```powershell
cargo test --workspace --all-features
```

### 查看测试覆盖率

```powershell
cargo install cargo-tarpaulin
cargo tarpaulin --workspace --out Html
```

### 性能测试

```powershell
cargo bench -p deepseek-aiwebllm
```

## 贡献测试用例

欢迎添加更多测试用例！请在 `tests/` 目录下添加集成测试。
