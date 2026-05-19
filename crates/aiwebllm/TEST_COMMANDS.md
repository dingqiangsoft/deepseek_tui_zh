# 🧪 aiwebllm 测试命令参考

> 本文档包含所有测试 aiwebllm 模块的命令，从简单到高级，按需选择。

---

## 📋 目录

- [快速测试（推荐新手）](#快速测试推荐新手)
- [单元测试](#单元测试)
- [千问 API 测试](#千问-api-测试)
- [千问网页版测试](#千问网页版测试)
- [豆包测试](#豆包测试)
- [TUI 集成测试](#tui-集成测试)
- [高级测试](#高级测试)
- [常见问题](#常见问题)

---

## 快速测试（推荐新手）

### 使用自动化测试脚本

```powershell
cd crates/aiwebllm
.\quick-test.ps1
```

**这个脚本会自动：**
- ✅ 检查是否设置 API Key
- ✅ 编译检查
- ✅ 运行单元测试
- ✅ 运行 API 连接测试（如果有 API Key）

---

## 单元测试

### 基础单元测试

```powershell
cd crates/aiwebllm
cargo test --lib
```

**预期输出：**
```
running 1 test
test session::tests::test_token_usage_reset ... ok

test result: ok. 1 passed; 0 failed; 0 ignored
```

### 查看详细输出

```powershell
cargo test --lib -- --nocapture
```

### 运行特定测试

```powershell
# 运行包含 "token" 的测试
cargo test --lib token

# 运行指定测试函数
cargo test --lib test_token_usage_reset
```

---

## 千问 API 测试

### 前置准备：设置 API Key

#### 方式 1: PowerShell（临时，当前会话有效）

```powershell
$env:QIANWEN_API_KEY = "sk-xxxxxxxxxxxxxxxx"
```

#### 方式 2: CMD（临时，当前会话有效）

```cmd
set QIANWEN_API_KEY=sk-xxxxxxxxxxxxxxxx
```

#### 方式 3: 永久设置（Windows 系统环境变量）

```powershell
[System.Environment]::SetEnvironmentVariable("QIANWEN_API_KEY", "sk-xxxxxxxxxxxxxxxx", "User")
```

设置后需要**重启终端**才能生效。

#### 方式 4: Linux/Mac

```bash
# 临时
export QIANWEN_API_KEY="sk-xxxxxxxxxxxxxxxx"

# 永久（添加到 ~/.bashrc 或 ~/.zshrc）
echo 'export QIANWEN_API_KEY="sk-xxxxxxxxxxxxxxxx"' >> ~/.bashrc
source ~/.bashrc
```

### 验证 API Key 是否设置成功

```powershell
# PowerShell
$env:QIANWEN_API_KEY

# Linux/Mac
echo $QIANWEN_API_KEY

# CMD
echo %QIANWEN_API_KEY%
```

### 运行 API 测试

```powershell
cd crates/aiwebllm
cargo run --example test_qianwen_web
```

**预期输出：**
```
🧪 测试千问 Web API 连接...

📤 发送请求到千问 API...
   模型: qwen-turbo
   消息: 你好，这是一个测试

📥 收到响应: HTTP 200

✅ 响应成功！

🎯 解析成功！
回复内容: 测试成功！
完成原因: stop
```

### 使用独立测试脚本

```powershell
cd crates/aiwebllm
.\test-qianwen.ps1
```

---

## 千问网页版测试

### 基础网页测试

```powershell
cd crates/aiwebllm
cargo run --example test_qianwen_web
```

**测试流程：**
1. 🌐 启动 Chrome 浏览器（可见窗口）
2. 🔗 访问 https://www.qianwen.com/
3. ⌨️ 自动输入测试消息
4. 📤 自动点击发送
5. ⏳ 等待 AI 回复（最长 60 秒）
6. 📝 显示回复内容

### 前置要求

- ✅ 已安装 Chrome 浏览器
- ✅ 已登录千问账号（在浏览器中手动登录一次）
- ✅ 网络连接正常

### 手动登录千问

```powershell
# 在默认浏览器中打开千问
start https://www.qianwen.com/
```

登录后关闭浏览器，再运行测试脚本。

---

## 豆包测试

### 方式 1: 网页版测试

```powershell
cd crates/aiwebllm
cargo run --example test_doubao_web
```

> 注：如果此示例不存在，需要手动创建

### 方式 2: 手动配置 Cookie

#### 步骤 1: 登录豆包

```powershell
start https://www.doubao.com/
```

#### 步骤 2: 提取 Cookie

1. 按 `F12` 打开开发者工具
2. 切换到 **Application**（应用程序）标签
3. 左侧选择 **Cookies** → `https://www.doubao.com`
4. 找到 `session_token` 或类似的 token
5. 复制值

#### 步骤 3: 配置到 aiwebllm

创建或编辑配置文件：

```powershell
notepad crates/aiwebllm/config/web_llm.toml
```

添加内容：

```toml
[doubao]
enabled = true
session_token = "从浏览器复制的token值"
model = "doubao-pro"
```

#### 步骤 4: 测试配置

```powershell
cargo run --example test_doubao_config
```

---

## TUI 集成测试

### 编译整个项目

```powershell
# 开发模式（快速编译）
cargo build --workspace

# 发布模式（优化性能）
cargo build --release
```

### 运行 TUI

```powershell
# 从源码运行
cargo run --bin deepseek

# 使用已编译的二进制文件
./target/release/deepseek
```

### 在 TUI 中测试 Web LLM

进入 TUI 后，使用 `/web` 命令：

```
/web 你好，请介绍一下自己
```

### 测试不同平台

```
# 测试千问
/web --platform qianwen 什么是机器学习？

# 测试豆包
/web --platform doubao 解释一下深度学习
```

### 查看帮助

```
/web --help
```

---

## 高级测试

### 运行所有测试

```powershell
# 整个 workspace
cargo test --workspace

# 包含集成测试
cargo test --workspace --all-features

# 显示详细输出
cargo test --workspace --all-features -- --nocapture
```

### 仅测试 aiwebllm 模块

```powershell
cargo test -p deepseek-aiwebllm
cargo test -p deepseek-aiwebllm --all-features
```

### 查看测试覆盖率

#### 安装 tarpaulin

```powershell
cargo install cargo-tarpaulin
```

#### 生成覆盖率报告

```powershell
# HTML 格式
cargo tarpaulin --workspace --out Html

# 终端输出
cargo tarpaulin --workspace --out Stdout
```

查看报告：

```powershell
start tarpaulin-report.html
```

### 性能基准测试

```powershell
cargo bench -p deepseek-aiwebllm
```

### 检查代码质量

```powershell
# 编译检查
cargo check --workspace

# 代码风格检查
cargo clippy --workspace --all-targets --all-features

# 格式化检查
cargo fmt --all -- --check
```

### 内存泄漏检测

```powershell
# Linux/Mac
cargo test --workspace --features "valgrind"

# Windows (使用 Dr. Memory)
# 需要手动安装 Dr. Memory
```

---

## 常见问题

### ❌ 未设置 API Key

**错误信息：**
```
错误: 请设置环境变量 QIANWEN_API_KEY
```

**解决方案：**
```powershell
$env:QIANWEN_API_KEY = "sk-xxxxxxxxxxxxxxxx"
```

---

### ❌ API Key 无效

**错误信息：**
```
HTTP 401: Invalid API Key
```

**解决方案：**
1. 检查 API Key 是否正确（复制时不要包含空格）
2. 访问 https://dashscope.console.aliyun.com/ 确认账号状态
3. 重新生成 API Key

---

### ❌ 网络超时

**错误信息：**
```
请求超时或连接失败
Error: timed out
```

**解决方案：**

```powershell
# 测试网络连通性
ping dashscope.aliyuncs.com

# 如有代理，设置代理
$env:HTTP_PROXY = "http://127.0.0.1:7890"
$env:HTTPS_PROXY = "http://127.0.0.1:7890"

# 或在 Cargo 配置中设置代理
# 编辑 ~/.cargo/config.toml
```

---

### ❌ 浏览器测试失败

**错误信息：**
```
未找到输入框
未找到发送按钮
```

**解决方案：**

1. 确认 Chrome 浏览器已安装
2. 手动访问千问网页确认页面正常
3. 检查页面结构是否变化（千问可能更新 UI）
4. 尝试使用 API 方式测试代替网页测试

---

### ❌ 编译失败

**错误信息：**
```
error: could not compile `deepseek-aiwebllm`
```

**解决方案：**

```powershell
# 清理构建缓存
cargo clean

# 重新编译
cargo build -p deepseek-aiwebllm

# 更新依赖
cargo update
```

---

### ❌ TUI 中 /web 命令不可用

**解决方案：**

1. 确认已编译整个项目：
   ```powershell
   cargo build --workspace
   ```

2. 检查 aiwebllm 模块是否启用：
   ```powershell
   grep -r "aiwebllm" crates/tui/Cargo.toml
   ```

3. 查看 TUI 日志：
   ```powershell
   RUST_LOG=debug cargo run --bin deepseek
   ```

---

## 📊 测试检查清单

运行以下命令确认所有测试通过：

- [ ] `cargo check --workspace` - 编译检查
- [ ] `cargo test --lib` - 单元测试
- [ ] `cargo clippy --workspace` - 代码质量
- [ ] `cargo fmt --all -- --check` - 代码格式
- [ ] API 连接测试（需要 API Key）
- [ ] 网页版测试（需要登录）
- [ ] TUI `/web` 命令测试

---

## 📚 相关文档

- [TESTING.md](./TESTING.md) - 完整测试文档
- [TEST_GUIDE.md](./TEST_GUIDE.md) - 测试指南（中文版）
- [README.md](./README.md) - 项目说明
- [WEB_LLM_USAGE.md](./WEB_LLM_USAGE.md) - Web LLM 使用指南

---

## 💡 提示

1. **新手建议**：先运行 `.\quick-test.ps1` 脚本
2. **API 测试**：比网页测试更稳定，推荐优先使用
3. **调试技巧**：使用 `RUST_LOG=debug` 查看详细日志
4. **性能优化**：使用 `--release` 模式编译获得最佳性能

---

**最后更新**: 2026-05-16
