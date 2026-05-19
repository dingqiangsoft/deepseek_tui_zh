# DeepSeek TUI 作为 MCP Server 配置方案

## 架构说明

```
WorkBuddy
    ↓ (调用 MCP 工具)
AICodeClaw MCP Server
    ↓ (连接 DeepSeek TUI MCP Server)
DeepSeek TUI (serve --mcp 模式)
```

## 配置步骤

### 步骤 1：启动 DeepSeek TUI MCP Server

在项目根目录执行：

```powershell
cd f:\ai\codes\github\deepseektuizh
.\target\release\deepseek.exe serve --mcp
```

或者使用 dispatcher 命令：

```powershell
deepseek mcp-server
```

**这会启动 DeepSeek TUI 的 MCP 服务模式，提供完整的开发工具。**

---

### 步骤 2：在 WorkBuddy 中配置

打开 WorkBuddy 设置界面：

**路径：** 设置 → AI模型 → 高级设置 → MCP Server 管理

**添加两个 MCP Server：**

#### Server 1: AICodeClaw (已有)
- **名称**: `aicodclaw`
- **命令**: `node`
- **参数**: `f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\index.js`
- **功能**: 文件操作、代码搜索、Shell 命令、AI 查询

#### Server 2: DeepSeek TUI (新增)
- **名称**: `deepseek`
- **命令**: `f:\ai\codes\github\deepseektuizh\target\release\deepseek.exe`
- **参数**: `serve --mcp`
- **工作目录**: `f:\ai\codes\github\deepseektuizh`
- **功能**: 完整的项目开发能力（读写文件、执行命令、代码分析等）

---

### 步骤 3：验证配置

在 WorkBuddy 中应该能看到：

**AICodeClaw 工具（8个）：**
- `mcp_aicodclaw_file_read`
- `mcp_aicodclaw_file_write`
- `mcp_aicodclaw_search`
- `mcp_aicodclaw_apply_patch`
- `mcp_aicodclaw_shell`
- `mcp_aicodclaw_deepseek`
- `mcp_aicodclaw_deepseek-reply`
- `mcp_aicodclaw_yolo`

**DeepSeek TUI 工具（完整开发工具集）：**
- `mcp_deepseek_read`
- `mcp_deepseek_write`
- `mcp_deepseek_edit`
- `mcp_deepseek_bash`
- `mcp_deepseek_grep`
- `mcp_deepseek_glob`
- 等其他开发工具

---

## 使用方式

### 简单任务 - 使用 AICodeClaw
```
调用 aicodclaw_file_read 读取文件
调用 aicodclaw_shell 执行命令
调用 aicodclaw_deepseek 查询 AI
```

### 复杂开发 - 使用 DeepSeek TUI
```
"调用 deepseek 工具创建一个新的 React 组件"
"调用 deepseek 工具重构这个函数"
"调用 deepseek 工具运行测试"
```

---

## 环境变量配置

DeepSeek TUI MCP Server 需要以下环境变量：

```powershell
$env:DEEPSEEK_PROVIDER = "openai"
$env:DEEPSEEK_BASE_URL = "http://192.168.2.5:1234/v1"
$env:DEEPSEEK_MODEL = "qwen3.5-9b-deepseek-v4-flash@q6_k"
$env:DEEPSEEK_API_KEY = "not-needed"
$env:DEEPSEEK_ALLOW_INSECURE_HTTP = "1"
```

**在 WorkBuddy 配置中可以设置环境变量，或者确保这些变量在系统环境中已设置。**

---

## 测试流程

1. **启动 DeepSeek TUI MCP Server**（后台运行）
2. **重启 WorkBuddy**（加载新配置）
3. **测试调用**：
   - 简单任务用 AICodeClaw
   - 复杂开发用 DeepSeek TUI

---

## 注意事项

⚠️ **DeepSeek TUI MCP Server 需要保持运行**
- 可以设置为 Windows 服务
- 或者在后台终端中运行
- WorkBuddy 会在每次调用时启动新的会话

⚠️ **工作目录很重要**
- DeepSeek TUI 会在配置的工作目录中操作文件
- 确保设置为你的项目根目录

⚠️ **权限问题**
- DeepSeek TUI 有细粒度的权限控制
- 首次使用时可能需要确认权限

---

## 优势

✅ **双层工具集**：AICodeClaw (8个工具) + DeepSeek TUI (完整开发工具)
✅ **灵活性**：简单任务用 AICodeClaw，复杂任务用 DeepSeek TUI
✅ **本地化**：全部在本地运行，数据不泄露
✅ **自主 AI**：使用本地 LLM，无需外部 API
