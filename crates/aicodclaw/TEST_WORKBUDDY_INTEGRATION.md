# WorkBuddy 调用 AICodeClaw 测试指南

## 📋 测试前准备

### 1. 确保 AICodeClaw 已编译

```powershell
# 检查可执行文件是否存在
Test-Path .\target\release\deepseek.exe

# 如果不存在，先编译
cargo build --release
```

### 2. 确保本地模型已启动（如使用 Ollama）

```powershell
# 检查 Ollama 是否运行
ollama list

# 如果没有模型，先拉取
ollama pull qwen2.5-coder:7b

# 启动 Ollama（通常自动启动）
ollama serve
```

### 3. 确认 WorkBuddy 已安装

```powershell
# 检查 WorkBuddy 配置目录
Test-Path C:\Users\Administrator\.workbuddy\mcp.json
```

---

## 🧪 测试方法

### 方法 1：在 WorkBuddy 界面中测试（推荐）

#### 步骤 1：配置 MCP

1. 打开 WorkBuddy
2. 进入 **连接器** → **自定义连接器** → **配置 MCP**
3. 添加 AICodeClaw 配置（参考 `WorkBuddy对接指南.md`）
4. 保存并启用

#### 步骤 2：验证服务状态

在 WorkBuddy 的 MCP 服务列表中：
- ✅ 看到 `aicodclaw` 服务
- ✅ 状态为 **在线**（绿色）
- ✅ 开关已打开

#### 步骤 3：发送测试消息

在 WorkBuddy 对话框中输入：

**测试 1：代码审查**
```
请使用 AICodeClaw 审查以下代码的安全性：

def login(username, password):
    query = f"SELECT * FROM users WHERE username='{username}' AND password='{password}'"
    return execute_query(query)
```

**期望结果：**
- AICodeClaw 识别 SQL 注入漏洞
- 提供修复建议
- 响应来自本地模型

---

**测试 2：代码生成**
```
请使用 AICodeClaw 生成一个 Python 函数，用于读取 CSV 文件并统计每列的空值数量。
```

**期望结果：**
- 生成完整的 Python 代码
- 包含错误处理
- 代码可执行

---

**测试 3：运维诊断**
```
服务器 CPU 使用率突然达到 100%，请分析可能的原因并提供排查步骤。
```

**期望结果：**
- 列出可能原因
- 提供诊断命令
- 给出解决方案

---

### 方法 2：在 TUI 中测试 MCP 连接

#### 步骤 1：启动 TUI

```powershell
.\target\release\deepseek
```

#### 步骤 2：查看 MCP 状态

在 TUI 输入框中输入：

```
/mcp status
```

**期望输出：**
```
MCP Servers:
  - aicodclaw: connected
```

#### 步骤 3：验证 MCP 连接

```
/mcp validate
```

**期望输出：**
```
Validating MCP servers...
  aicodclaw: ✓ connected
  Tools available: 15
```

#### 步骤 4：测试工具调用

```
/tools list
```

**期望输出：**
```
Available tools:
  - code_review
  - security_scan
  - log_analysis
  ...
```

---

### 方法 3：命令行直接测试 MCP Server

#### 步骤 1：启动 MCP Server

```powershell
.\target\release\deepseek.exe serve --mcp
```

**期望行为：**
- 程序启动并等待输入
- 不报错
- 控制台无输出（等待 JSON-RPC 请求）

#### 步骤 2：发送测试请求（新终端）

打开新的 PowerShell 窗口：

```powershell
# 发送 tools/list 请求
@'
{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}
'@ | .\target\release\deepseek.exe serve --mcp
```

**期望输出：**
```json
{"jsonrpc":"2.0","id":1,"result":{"tools":[...]}}
```

---

### 方法 4：使用 MCP Inspector 测试（高级）

#### 安装 MCP Inspector

```powershell
# 安装 Node.js（如果未安装）
# 下载：https://nodejs.org/

# 安装 MCP Inspector
npm install -g @modelcontextprotocol/inspector
```

#### 启动 Inspector

```powershell
mcp-inspector --command ".\target\release\deepseek.exe" --args "serve","--mcp"
```

#### 在浏览器中测试

1. 打开浏览器访问：`http://localhost:3000`
2. 查看可用工具列表
3. 测试工具调用
4. 查看请求/响应详情

---

## ✅ 测试检查清单

### 基础检查

- [ ] AICodeClaw 可执行文件存在
- [ ] 本地模型（Ollama）已启动
- [ ] WorkBuddy 已安装并运行
- [ ] MCP 配置正确（路径、参数）
- [ ] MCP 服务状态为在线

### 功能测试

- [ ] WorkBuddy 能看到 `aicodclaw` 服务
- [ ] 服务开关可以打开
- [ ] 发送消息后能收到响应
- [ ] 响应来自本地模型（非云端）
- [ ] 代码审查功能正常
- [ ] 代码生成功能正常

### 性能测试

- [ ] 响应时间 < 10 秒（简单任务）
- [ ] 无崩溃或卡死
- [ ] 内存占用正常（< 2GB）
- [ ] 多次调用稳定

### 安全测试

- [ ] 代码在本地执行
- [ ] 无数据上传到云端
- [ ] 网络监控无异常外传
- [ ] 日志中无敏感信息泄露

---

## 🔍 故障排查

### 问题 1：WorkBuddy 中看不到 aicodclaw

**检查步骤：**
```powershell
# 1. 检查配置文件
Get-Content C:\Users\Administrator\.workbuddy\mcp.json

# 2. 确认 JSON 格式正确
# 使用 JSON 验证工具检查

# 3. 重启 WorkBuddy
```

**解决方案：**
- 确保 `mcp.json` 格式正确
- 重启 WorkBuddy
- 检查配置路径是否正确

---

### 问题 2：服务状态显示"离线"

**检查步骤：**
```powershell
# 1. 测试可执行文件
& "F:\ai\codes\github\deepseektuizh\target\release\deepseek.exe" --version

# 2. 测试 MCP Server
.\target\release\deepseek.exe serve --mcp

# 3. 检查 Ollama
ollama list
```

**解决方案：**
- 修正 `command` 路径
- 启动 Ollama
- 检查防火墙设置

---

### 问题 3：调用无响应

**检查步骤：**
```powershell
# 1. 查看 WorkBuddy 日志
# 通常在：%LOCALAPPDATA%\WorkBuddy\logs\

# 2. 检查模型是否加载
ollama ps

# 3. 测试模型响应
curl http://localhost:11434/v1/chat/completions `
  -H "Content-Type: application/json" `
  -d '{
    "model": "qwen2.5-coder:7b",
    "messages": [{"role": "user", "content": "hello"}]
  }'
```

**解决方案：**
- 等待模型加载完成
- 更换更强的模型
- 增加超时时间

---

### 问题 4：响应来自云端而非本地

**检查步骤：**
```powershell
# 1. 断开网络测试
# 拔掉网线或禁用网卡

# 2. 再次调用
# 如果能正常响应，说明是本地模型

# 3. 检查配置
Get-Content C:\Users\Administrator\.workbuddy\mcp.json | Select-String "DEEPSEEK_BASE_URL"
```

**解决方案：**
- 确认 `DEEPSEEK_BASE_URL` 指向本地
- 确认 Ollama 正在运行
- 检查模型名称是否正确

---

## 📊 测试报告模板

### 测试记录

| 测试项 | 测试时间 | 测试结果 | 备注 |
|--------|---------|---------|------|
| MCP 配置 | 2026-05-17 | ✅ 通过 | 配置正确 |
| 服务状态 | 2026-05-17 | ✅ 通过 | 显示在线 |
| 代码审查 | 2026-05-17 | ✅ 通过 | 识别 SQL 注入 |
| 代码生成 | 2026-05-17 | ✅ 通过 | 生成正确代码 |
| 响应时间 | 2026-05-17 | ✅ 通过 | 平均 5 秒 |
| 本地验证 | 2026-05-17 | ✅ 通过 | 断网可工作 |

### 性能指标

| 指标 | 数值 | 标准 |
|------|------|------|
| 平均响应时间 | 5 秒 | < 10 秒 |
| 内存占用 | 1.5 GB | < 2 GB |
| 成功率 | 100% | > 95% |
| 并发数 | 1 | 1（MCP stdio） |

---

## 🎯 比赛演示测试脚本

### 完整演示流程（5 分钟）

```powershell
# 1. 展示 AICodeClaw 版本
.\target\release\deepseek.exe --version

# 2. 展示 MCP 配置
Get-Content C:\Users\Administrator\.workbuddy\mcp.json

# 3. 启动 WorkBuddy
Start-Process workbuddy

# 4. 在 WorkBuddy 中演示
# - 代码审查（展示安全漏洞识别）
# - 代码生成（展示本地 AI 能力）
# - 强调："所有处理在本地完成，零泄露"

# 5. 断网测试（可选）
# - 拔掉网线
# - 再次调用
# - 证明无需云端
```

---

## 📞 获取帮助

- **问题反馈**：提交 GitHub Issue
- **文档**：查看 `docs/zh/WorkBuddy对接指南.md`
- **配置参考**：查看 `crates/aicodclaw/mcp-config-corrected.json`

---

**祝测试顺利！** 🦞✨
