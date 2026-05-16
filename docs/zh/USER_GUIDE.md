# DeepSeek TUI 用户使用指南

本文档详细说明 DeepSeek TUI 的界面布局、启动方式、所有可用命令及其使用方法。

---

## 🖥️ 界面布局

### 整体布局

```
┌─────────────────────────────────────────────────────────────────┐
│  Header (顶部栏)                                                 │
│  🤖 agent · qwen3.5-9b · deepseek-v4-flash · auto  0% · v0.8.37 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Main Content (主内容区)                                         │
│  ┌─────────────────────┬───────────────────────────────────────┐ │
│  │ SidePanel (可选)    │ Transcript (转录区)                    │ │
│  │                     │                                       │ │
│  │ • Work              │ ● User: 你好                           │ │
│  │ • Tasks             │                                       │ │
│  │                     │ ○ Assistant: 你好！有什么可以帮您？    │ │
│  │                     │                                       │ │
│  │                     │                                       │ │
│  └─────────────────────┴───────────────────────────────────────┘ │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│  Composer (底部输入区)                                           │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │ 编写任务或使用 /。                           [📎] [⚙️]      │ │
│  └─────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 1. Header (顶部栏)

**显示内容**:
```
🤖 agent · qwen3.5-9b-deepseek-v4-flash@q6_k · OpenAI · auto · 0% · v0.8.37
```

**各部分说明**:

| 部分 | 说明 | 示例 |
|------|------|------|
| 🤖 **模式** | 当前运行模式 | `agent` / `plan` / `auto` / `yolo` |
| **模型** | 使用的模型 | `qwen3.5-9b-deepseek-v4-flash@q6_k` |
| **提供商** | API提供商 | `OpenAI` / `DeepSeek` / `Ollama` |
| **auto** | 自动模式状态 | `auto` / `high` / `max` / `off` |
| **0%** | 上下文使用率 | `0%` ~ `100%` |
| **v0.8.37** | 版本号 | `v0.8.37` |

### 2. Main Content (主内容区)

#### 2.1 Transcript (转录区) - 右侧

显示所有对话历史，包括：

**消息类型**:

```
● User Message                     # 用户消息（蓝色圆点）
  你好，请帮我分析这个代码

○ Assistant Message                # AI回复（空心圆）
  好的，让我来分析一下...

💭 Thinking Block                  # 思考块（灰色背景）
  让我先理解一下代码结构...
  [显示推理过程]

🔧 Tool Call                       # 工具调用
  ▶ read_file
    path: src/main.rs
    ✓ Success (0.3s)

⚙️ System Message                  # 系统消息（黄色）
  Session saved successfully
```

**滚动和导航**:
- `↑` / `↓` - 上下滚动
- `PageUp` / `PageDown` - 翻页
- `Home` / `End` - 跳到顶部/底部
- `Alt+V` - 查看详情（折叠内容）

#### 2.2 SidePanel (侧边栏) - 左侧（可选）

按 `Ctrl+O` 或 `Tab` 切换显示。

**Work Panel (工作面板)**:
```
┌ Work ───────────────────────────────┐
│ Strategy 67% complete (2/3)         │
│ 正在修复微信聊天记录抓取问题        │
│                                     │
│ [✓] 分析 GrabWorker 超时保护        │
│ [✓] 改进异常处理逻辑                │
│ [ ] 添加按钮状态恢复方法            │
└─────────────────────────────────────┘
```

**Task Panel (任务面板)**:
```
┌ Tasks ──────────────────────────────┐
│ turn 3e7b5dc8-... (completed)       │
│                                     │
│ Recent tools:                       │
│ • read_file (3 calls)              │
│ • grep_files (2 calls)             │
│ • edit_file (1 call)               │
└─────────────────────────────────────┘
```

### 3. Composer (底部输入区)

```
┌─────────────────────────────────────────────────────────────┐
│ 编写任务或使用 /。                            [📎] [⚙️]      │
└─────────────────────────────────────────────────────────────┘
  ↑                      ↑                      ↑
  输入框                 附件按钮               设置按钮
```

**功能**:
- 输入任务消息
- 输入 `/` 打开命令面板
- 输入 `@` 附加文件上下文
- `Tab` - 自动补全命令

---

## 🚀 启动方式

### 1. 基本启动

```bash
# 交互式 TUI（默认）
deepseek

# 指定提供商
deepseek --provider openai

# 指定模型
deepseek --model deepseek-v4-flash

# 使用 Auto 模式
deepseek --model auto
```

### 2. 不同模式启动

```bash
# Plan 模式（只读调查）
deepseek

# Agent 模式（交互式，需确认）
deepseek

# YOLO 模式（自动批准所有工具）
deepseek --yolo
```

**在 TUI 中切换模式**:
```
/mode plan          # 切换到 Plan 模式
/mode agent         # 切换到 Agent 模式
/mode auto          # 切换到 Auto 模式
/mode yolo          # 切换到 YOLO 模式
```

### 3. 本地 LLM 启动

```bash
# 方法 1: 命令行参数
deepseek --provider openai \
         --model "your-model"

# 方法 2: 环境变量
export DEEPSEEK_BASE_URL="http://localhost:1234/v1"
export DEEPSEEK_ALLOW_INSECURE_HTTP=1
deepseek --provider openai

# 方法 3: 使用启动脚本
.\start-aiclaw.ps1        # Windows PowerShell
./start-aiclaw.sh         # Linux/Mac
```

### 4. 非交互式启动

```bash
# 一次性提示
deepseek "解释这个函数"

# 从文件读取提示
deepseek -p "$(cat prompt.txt)"

# NDJSON 流输出
deepseek exec --auto --output-format stream-json "fix this bug"

# 恢复会话
deepseek exec --resume <SESSION_ID> "follow up"
```

### 5. 服务器模式启动

```bash
# HTTP/SSE API 服务器
deepseek serve --http

# ACP 适配器（用于 Zed 编辑器）
deepseek serve --acp
```

---

## 📋 命令完整列表

### 1. 斜杠命令（在 Composer 中使用）

#### 1.1 模型和提供商

| 命令 | 功能 | 示例 |
|------|------|------|
| `/model` | 打开模型选择器 | `/model` |
| `/model <id>` | 直接切换模型 | `/model deepseek-v4-pro` |
| `/model auto` | 启用自动模式 | `/model auto` |
| `/models` | 列出可用模型 | `/models` |
| `/provider` | 打开提供商选择器 | `/provider` |
| `/provider <name>` | 直接切换提供商 | `/provider openai` |

#### 1.2 会话管理

| 命令 | 功能 | 示例 |
|------|------|------|
| `/compact` | 压缩上下文（节省内存） | `/compact` |
| `/reset` | 重置当前会话 | `/reset` |
| `/clear` | 清空对话历史 | `/clear` |
| `/exit` | 退出程序 | `/exit` |
| `/quit` | 退出程序（同 /exit） | `/quit` |

#### 1.3 主题和界面

| 命令 | 功能 | 示例 |
|------|------|------|
| `/theme` | 打开主题选择器 | `/theme` |
| `/theme <name>` | 直接切换主题 | `/theme dark` |
| `/config` | 打开配置界面 | `/config` |
| `/config <key> <value>` | 设置配置 | `/config locale zh-Hans` |

**可用主题**:
- `dark` - 深色（默认）
- `light` - 浅色
- `catppuccin` - 暖色调
- `tokyo-night` - 蓝紫色
- `dracula` - 暗红色
- `gruvbox` - 暖棕色

#### 1.4 技能和工具

| 命令 | 功能 | 示例 |
|------|------|------|
| `/skills` | 列出已安装技能 | `/skills` |
| `/skill <name>` | 激活技能 | `/skill v4-best-practices` |
| `/skill new` | 创建新技能 | `/skill new` |
| `/skill install <repo>` | 安装社区技能 | `/skill install github:owner/repo` |
| `/skill update` | 更新技能 | `/skill update` |
| `/skill uninstall <name>` | 卸载技能 | `/skill uninstall my-skill` |

#### 1.5 文件操作

| 命令 | 功能 | 示例 |
|------|------|------|
| `/cd <dir>` | 切换工作目录（有限支持） | `/cd ../other-project` |
| `/restore` | 恢复工作区快照 | `/restore` |
| `/revert_turn` | 回滚上一轮修改 | `/revert_turn` |

#### 1.6 草稿管理

| 命令 | 功能 | 示例 |
|------|------|------|
| `/stash` | 暂存当前草稿 | `/stash` |
| `/stash list` | 列出暂存的草稿 | `/stash list` |
| `/stash pop` | 恢复最近的草稿 | `/stash pop` |

#### 1.7 其他命令

| 命令 | 功能 | 示例 |
|------|------|------|
| `/help` | 显示帮助 | `/help` |
| `/rename <title>` | 重命名会话 | `/rename "代码审查任务"` |
| `/status` | 显示状态信息 | `/status` |
| `/cost` | 显示成本统计 | `/cost` |

---

### 2. 终端命令（在 Shell 中使用）

#### 2.1 基本命令

| 命令 | 功能 | 示例 |
|------|------|------|
| `deepseek` | 启动交互式 TUI | `deepseek` |
| `deepseek --version` | 显示版本 | `deepseek --version` |
| `deepseek --help` | 显示帮助 | `deepseek --help` |

#### 2.2 诊断和配置

| 命令 | 功能 | 示例 |
|------|------|------|
| `deepseek doctor` | 诊断安装问题 | `deepseek doctor` |
| `deepseek doctor --json` | JSON 格式诊断 | `deepseek doctor --json` |
| `deepseek setup --status` | 查看设置状态 | `deepseek setup --status` |
| `deepseek setup --tools` | 初始化工具目录 | `deepseek setup --tools` |
| `deepseek config <key>` | 读取配置 | `deepseek config provider` |
| `deepseek config <key> <value>` | 设置配置 | `deepseek config locale zh-Hans` |

#### 2.3 认证管理

| 命令 | 功能 | 示例 |
|------|------|------|
| `deepseek auth set` | 设置 API Key | `deepseek auth set --provider deepseek` |
| `deepseek auth status` | 查看认证状态 | `deepseek auth status` |
| `deepseek auth clear` | 清除 API Key | `deepseek auth clear --provider deepseek` |
| `deepseek login` | 登录（旧版） | `deepseek login --api-key "sk-xxx"` |
| `deepseek logout` | 登出 | `deepseek logout` |

#### 2.4 模型和会话

| 命令 | 功能 | 示例 |
|------|------|------|
| `deepseek models` | 列出可用模型 | `deepseek models` |
| `deepseek sessions` | 列出已保存会话 | `deepseek sessions` |
| `deepseek resume` | 恢复会话 | `deepseek resume --last` |
| `deepseek resume <ID>` | 恢复指定会话 | `deepseek resume abc-123` |
| `deepseek fork <ID>` | 分叉会话 | `deepseek fork abc-123` |
| `deepseek model` | 解析模型 | `deepseek model auto` |

#### 2.5 执行命令

| 命令 | 功能 | 示例 |
|------|------|------|
| `deepseek exec` | 非交互式执行 | `deepseek exec "fix bug"` |
| `deepseek exec --auto` | 自动批准 | `deepseek exec --auto "fix bug"` |
| `deepseek exec --resume <ID>` | 恢复执行 | `deepseek exec --resume abc "continue"` |
| `deepseek exec --output-format json` | JSON 输出 | `deepseek exec --output-format stream-json "task"` |

#### 2.6 MCP 管理

| 命令 | 功能 | 示例 |
|------|------|------|
| `deepseek mcp list` | 列出 MCP 服务器 | `deepseek mcp list` |
| `deepseek mcp validate` | 验证 MCP 配置 | `deepseek mcp validate` |
| `deepseek mcp-server` | 运行 MCP 服务器 | `deepseek mcp-server` |

#### 2.7 代码审查

| 命令 | 功能 | 示例 |
|------|------|------|
| `deepseek review` | 代码审查 | `deepseek review` |
| `deepseek review --base main` | 对比分支 | `deepseek review --base main --head feature` |
| `deepseek apply` | 应用补丁 | `deepseek apply patch.diff` |

#### 2.8 其他命令

| 命令 | 功能 | 示例 |
|------|------|------|
| `deepseek update` | 更新到最新版 | `deepseek update` |
| `deepseek metrics` | 查看使用指标 | `deepseek metrics` |
| `deepseek features` | 查看功能标志 | `deepseek features` |
| `deepseek init` | 创建 AGENTS.md | `deepseek init` |
| `deepseek run pr <N>` | 获取 PR 并审查 | `deepseek run pr 123` |
| `deepseek completions` | 生成 Shell 补全 | `deepseek completions bash` |
| `deepseek sandbox` | 评估沙箱策略 | `deepseek sandbox` |
| `deepseek eval` | 运行离线评估 | `deepseek eval` |

---

## 🔍 查看信息的方法

### 1. 查看模型信息

**在 TUI 中**:
```
/models              # 打开模型选择器，显示所有可用模型
/model               # 同上
```

**在终端中**:
```bash
deepseek models      # 列出所有可用模型
```

**输出示例**:
```
Available models:
  deepseek-v4-pro       (1M context)  $0.435/1M in, $0.87/1M out
  deepseek-v4-flash     (1M context)  $0.14/1M in, $0.28/1M out
  deepseek-chat         (alias)       → deepseek-v4-flash
```

### 2. 查看会话信息

**在 TUI 中**:
```
/status              # 显示当前会话状态
/cost                # 显示成本统计
```

**在终端中**:
```bash
deepseek sessions    # 列出所有会话
```

**输出示例**:
```
Saved sessions:
  abc-123  2026-05-16 10:30  DeepSeek-TUI-main    45 messages  $0.12
  def-456  2026-05-16 09:15  AIClaw               23 messages  $0.08
  ghi-789  2026-05-15 16:45  Test Project         67 messages  $0.25
```

### 3. 查看成本统计

**在 TUI 中**:
```
/cost                # 显示成本统计
```

**显示内容**:
```
Session Cost Summary:
  Total tokens: 12,345
  Input tokens: 10,000 (cache hit: 8,500)
  Output tokens: 2,345
  
  Estimated cost: $0.15
  Cache savings: $0.35 (70%)
```

**顶部栏实时显示**:
```
... · 45% · $0.12 · v0.8.37
         ↑          ↑
     上下文使用   当前成本
```

### 4. 查看配置信息

**在 TUI 中**:
```
/config              # 打开配置界面
/config locale       # 查看当前语言
/config provider     # 查看当前提供商
```

**在终端中**:
```bash
deepseek config      # 查看所有配置
deepseek config provider  # 查看特定配置
deepseek doctor      # 诊断信息（包含配置）
```

### 5. 查看认证状态

**在终端中**:
```bash
deepseek auth status
```

**输出示例**:
```
Authentication Status:
  Provider: deepseek
  Source: config file (~/.deepseek/config.toml)
  Key: sk-...abcd (last 4 chars)
  
  Keyring: not found
  Env var: not set
```

### 6. 查看工具使用情况

**在 TUI 中**:
- 按 `Ctrl+O` 打开侧边栏
- 查看 `Tasks` 面板

**显示内容**:
```
Recent tools:
  • read_file (5 calls, 1.2s avg)
  • grep_files (3 calls, 0.8s avg)
  • edit_file (2 calls, 0.5s avg)
  • shell_exec (1 call, 2.3s)
```

### 7. 查看帮助信息

**在 TUI 中**:
```
F1                 # 打开帮助覆盖层
/help              # 显示帮助命令列表
```

**在终端中**:
```bash
deepseek --help    # 命令行帮助
deepseek <cmd> --help  # 特定命令帮助
```

### 8. 查看日志和调试信息

**日志文件位置**:
```
~/.deepseek/logs/deepseek.log
```

**在 TUI 中启用调试**:
```bash
# 启动时设置日志级别
RUST_LOG=debug deepseek
```

**查看实时日志**:
```bash
tail -f ~/.deepseek/logs/deepseek.log
```

---

## ⌨️ 快捷键完整列表

### 全局快捷键

| 快捷键 | 功能 | 说明 |
|--------|------|------|
| `F1` | 帮助 | 打开帮助覆盖层 |
| `Esc` | 返回/关闭 | 关闭弹窗或返回上一步 |
| `Ctrl+C` | 停止/退出 | 停止当前操作或退出程序 |
| `Ctrl+D` | 退出 | 退出程序（EOF） |
| `Ctrl+K` | 命令面板 | 打开命令搜索面板 |
| `Ctrl+O` | 侧边栏 | 切换侧边栏显示 |
| `Ctrl+R` | 恢复会话 | 打开会话恢复选择器 |
| `Ctrl+S` | 暂存草稿 | 暂存当前 Composer 草稿 |

### Composer 快捷键

| 快捷键 | 功能 | 说明 |
|--------|------|------|
| `Tab` | 自动补全 | 补全 `/` 或 `@` 命令 |
| `Shift+Tab` | 切换推理级别 | off → high → max |
| `Enter` | 提交 | 提交消息 |
| `Shift+Enter` | 换行 | 插入新行 |
| `↑` | 历史/选择 | 在历史中导航或选择附件 |
| `↓` | 历史/选择 | 在历史中导航或选择附件 |
| `Alt+R` | 搜索历史 | 搜索提示历史 |

### Transcript 导航

| 快捷键 | 功能 | 说明 |
|--------|------|------|
| `↑` / `↓` | 滚动 | 上下滚动 |
| `PageUp` / `PageDown` | 翻页 | 向上/向下翻页 |
| `Home` / `End` | 跳转 | 跳到顶部/底部 |
| `Alt+V` | 查看详情 | 展开折叠的内容 |
| `Ctrl+O` | 推理内容 | 查看完整推理（Ctrl+O） |

### 模式切换

| 快捷键 | 功能 | 说明 |
|--------|------|------|
| `Tab` (空闲时) | 切换模式 | Plan → Agent → YOLO → Plan |
| `Shift+Tab` | 推理级别 | off → high → max → off |

---

## 🎯 常见使用场景

### 场景 1: 代码审查

```bash
# 1. 启动
deepseek --model auto

# 2. 在 Composer 中输入
@src/main.rs 请审查这个文件的代码质量

# 3. 查看结果
# - 阅读 Transcript 中的分析
# - 查看工具调用记录（Ctrl+O）
# - 查看成本（/cost）
```

### 场景 2: Bug 修复

```bash
# 1. 启动 YOLO 模式（自动执行）
deepseek --yolo

# 2. 输入任务
修复微信聊天记录抓取按钮第一次点击后失效的问题

# 3. AI 会自动：
# - 读取相关文件
# - 分析问题
# - 修改代码
# - 你可以实时查看进度

# 4. 查看修改
# - Transcript 显示所有工具调用
# - Work 面板显示任务进度
```

### 场景 3: 项目分析

```bash
# 1. 切换到项目目录
cd F:\ai\tranprojects\AIClaw\aiclaw

# 2. 启动
deepseek

# 3. 附加整个项目
@. 分析项目结构并生成文档

# 4. AI 会：
# - 遍历目录
# - 读取关键文件
# - 生成分析报告
```

### 场景 4: 长时间任务

```bash
# 1. 启动并设置长超时
export DEEPSEEK_STREAM_IDLE_TIMEOUT_SECS=3600
deepseek --model auto

# 2. 提交复杂任务
分析整个项目并生成架构文档

# 3. 监控进度
# - 查看上下文使用率（顶部栏百分比）
# - 接近 60% 时压缩
/compact

# 4. 保存会话
# 会话自动保存
# 下次可以恢复
deepseek resume --last
```

---

## 📊 界面元素详解

### 1. 消息状态指示

| 符号 | 含义 |
|------|------|
| `●` | 用户消息 |
| `○` | AI 回复 |
| `💭` | 思考块 |
| `🔧` | 工具调用 |
| `▶` | 工具运行中 |
| `✓` | 工具成功 |
| `✗` | 工具失败 |
| `⚙️` | 系统消息 |

### 2. 模式指示

| 模式 | 显示 | 颜色 |
|------|------|------|
| Plan | `plan` | 蓝色 |
| Agent | `agent` | 绿色 |
| Auto | `auto` | 紫色 |
| YOLO | `yolo` | 红色 |

### 3. 推理级别

| 级别 | 显示 | 说明 |
|------|------|------|
| Off | `off` | 无推理 |
| High | `high` | 高级推理 |
| Max | `max` | 最大推理 |
| Auto | `auto` | 自动选择 |

### 4. 成本显示

**格式**:
```
$0.12          # 美元
¥0.85          # 人民币（zh-Hans 语言）
```

**位置**:
- 顶部栏右侧
- `/cost` 命令查看详细统计

---

## 🔧 高级功能

### 1. 子智能体

**在 Composer 中使用**:
```
使用子智能体并行分析以下文件：
- src/main.rs
- src/utils.rs
- src/config.rs
```

**查看子智能体状态**:
- `Ctrl+O` → Tasks 面板
- 显示运行中的子智能体

### 2. RLM (REPL 学习模块)

**使用 RLM**:
```
使用 RLM 分析大型日志文件
```

**RLM 会自动**:
- 打开 Python REPL
- 加载文件
- 执行分析
- 返回结果

### 3. LSP 诊断

**自动触发**:
- 每次 AI 修改代码后
- 自动运行语言服务器
- 显示错误/警告

**查看诊断**:
- Transcript 中显示
- 内联错误标记

---

## 📝 提示和技巧

### 1. 提高效率

- **使用 Auto 模式**: 让 AI 自动选择最经济的模型
- **压缩上下文**: 接近 60% 时使用 `/compact`
- **暂存草稿**: `Ctrl+S` 保存未完成的消息
- **使用技能**: 安装常用技能加速工作

### 2. 节省成本

- **缓存命中**: 保持上下文稳定，提高缓存命中率
- **选择合适的模型**: 简单任务用 Flash，复杂任务用 Pro
- **压缩频率**: 定期压缩，减少 token 使用

### 3. 避免问题

- **不要超过 80% 上下文**: 容易崩溃
- **每 3 轮检查**: 上下文使用率、子智能体状态
- **使用子智能体**: 并行处理独立任务
- **保存会话**: 重要工作及时保存

---

## 🆘 故障排查

### 问题 1: 程序无响应

**解决**:
```
Ctrl+C              # 停止当前操作
/exit               # 退出
deepseek            # 重新启动
```

### 问题 2: 工具执行被拒绝

**解决**:
```
/mode auto          # 切换到自动模式
# 或
/exit
deepseek --yolo     # 用 YOLO 模式启动
```

### 问题 3: 上下文满了

**解决**:
```
/compact            # 压缩上下文
# 或
/reset              # 重置会话
```

### 问题 4: 找不到命令

**解决**:
```
F1                  # 查看帮助
/help               # 列出命令
Tab                 # 自动补全
```

---

## 📚 相关文档

- [快速入门](QUICKSTART.md) - 5分钟上手
- [配置指南](CONFIGURATION_GUIDE.md) - 详细配置
- [代码结构](CODE_STRUCTURE.md) - 代码架构
- [快捷键](../KEYBINDINGS.md) - 完整快捷键列表
- [模式说明](../MODES.md) - 三种模式详解

---

*文档最后更新: 2026-05-16*
*版本: v0.8.37*
