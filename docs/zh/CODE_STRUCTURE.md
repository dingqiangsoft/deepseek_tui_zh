# DeepSeek TUI 代码结构文档

本文档详细说明 DeepSeek TUI 项目的代码结构，包括目录组织、主要模块、类/结构体和关键方法。

---

## 📁 项目总览

```
DeepSeek-TUI/
├── crates/                      # Rust Workspace（14个crate）
│   ├── cli/                     # 命令行调度器（deepseek命令）
│   ├── tui/                     # TUI运行时（deepseek-tui命令）
│   ├── core/                    # 核心共享逻辑
│   ├── config/                  # 配置管理
│   ├── agent/                   # 智能体引擎
│   ├── tools/                   # 工具注册和执行
│   ├── state/                   # 状态管理和持久化
│   ├── protocol/                # API协议定义
│   ├── mcp/                     # MCP协议支持
│   ├── hooks/                   # 钩子系统
│   ├── secrets/                 # 密钥管理
│   ├── execpolicy/              # 执行策略
│   ├── app-server/              # HTTP/SSE服务器
│   └── tui-core/                # TUI核心组件
├── docs/                        # 文档
├── web/                         # 网站（Next.js）
└── integrations/                # 集成（飞书桥接等）
```

---

## 🏗️ Crate 详细说明

### 1. `crates/cli` - 命令行调度器

**职责**: 提供 `deepseek` 命令，负责参数解析、TUI启动、非交互式执行

**目录结构**:
```
crates/cli/src/
├── main.rs          # 入口点
├── lib.rs           # 主要逻辑（94KB）
├── metrics.rs       # 使用指标统计
└── update.rs        # 自动更新功能
```

**主要功能**:
- 命令行参数解析（Clap）
- 路由到 TUI 或执行模式
- API Key 管理
- 诊断检查（doctor）
- 会话管理（resume/fork）
- 自动更新检查

**关键方法**:
```rust
// lib.rs
fn run() -> Result<()>              // 主入口
fn run_interactive() -> Result<()>  // 启动交互式TUI
fn run_exec() -> Result<()>         // 非交互式执行
fn run_doctor() -> Result<()>       // 诊断检查
fn run_auth() -> Result<()>         // 认证管理
fn run_models() -> Result<()>       # 列出模型
fn run_sessions() -> Result<()>     # 列出会话
```

---

### 2. `crates/tui` - TUI 运行时

**职责**: 终端用户界面，包含所有交互逻辑、工具执行、会话管理

**目录结构**（主要）:
```
crates/tui/src/
├── main.rs              # TUI入口（224KB）
├── client.rs            # API客户端（101KB）
├── config.rs            # 配置管理（225KB）
├── localization.rs      # 本地化（141KB）
├── mcp.rs               # MCP支持（144KB）
├── prompts.rs           # 提示词管理（79KB）
├── compaction.rs        # 上下文压缩（95KB）
├── runtime_threads.rs   # 运行时线程（196KB）
├── runtime_api.rs       # 运行时API（121KB）
├── palette.rs           # 调色板（57KB）
├── automation_manager.rs # 自动化管理（32KB）
├── cycle_manager.rs     # 周期管理（38KB）
├── task_manager.rs      # 任务管理（68KB）
├── session_manager.rs   # 会话管理（65KB）
├── working_set.rs       # 工作集（57KB）
├── command_safety.rs    # 命令安全（53KB）
├── hooks.rs             # 钩子（35KB）
├── project_context.rs   # 项目上下文（37KB）
├── settings.rs          # 设置（79KB）
├── tools/               # 工具实现（47个文件）
├── commands/            # 命令处理（29个文件）
├── tui/                 # UI组件（63个文件）
├── core/                # 核心组件（11个文件）
├── sandbox/             # 沙箱实现（7个文件）
├── skills/              # 技能系统（3个文件）
├── rlm/                 # RLM引擎（5个文件）
├── lsp/                 # LSP支持（4个文件）
├── snapshot/            # 快照系统（4个文件）
├── repl/                # REPL支持（3个文件）
├── prompts/             # 提示词（10个文件）
└── execpolicy/          # 执行策略（10个文件）
```

**主要模块**:

#### 2.1 客户端模块 (`client.rs`)
```rust
struct Client                    # API客户端主结构
  - async fn stream_chat()       # 流式聊天
  - async fn list_models()       # 列出模型
  - async fn check_auth()        # 检查认证
  
struct StreamingResponse         # 流式响应
  - fn next_chunk()              # 获取下一块数据
  - fn is_done()                 # 是否完成
```

#### 2.2 配置模块 (`config.rs`)
```rust
struct AppConfig                 # 应用配置
  - fn load()                    # 加载配置
  - fn save()                    # 保存配置
  - fn merge_project_overlay()   # 合并项目配置
  
struct ProviderConfig            # 提供商配置
  - api_key: String
  - base_url: String
  - model: String
```

#### 2.3 工具模块 (`tools/`)
```
tools/
├── shell.rs            # Shell命令执行
├── read_file.rs        # 读取文件
├── write_file.rs       # 写入文件
├── edit_file.rs        # 编辑文件
├── apply_patch.rs      # 应用补丁
├── glob_files.rs       # 文件搜索
├── grep_files.rs       # 内容搜索
├── web_search.rs       # 网络搜索
├── fetch_url.rs        # 获取URL
├── agent_spawn.rs      # 生成子智能体
├── agent_open.rs       # 打开子智能体
├── agent_wait.rs       # 等待子智能体
├── agent_close.rs      # 关闭子智能体
├── agent_eval.rs       # 评估子智能体
├── rlm_open.rs         # 打开RLM会话
├── rlm_eval.rs         # 评估RLM
├── rlm_configure.rs    # 配置RLM
├── mcp_call.rs         # 调用MCP工具
├── checklist_write.rs  # 写入检查清单
├── handle_read.rs      # 读取句柄
└── ... (47个工具)
```

**工具 trait**:
```rust
trait Tool {
    fn name(&self) -> &str;                    # 工具名称
    fn description(&self) -> &str;             # 工具描述
    fn parameters(&self) -> serde_json::Value; # 参数schema
    async fn execute(&self, args: Args) -> Result<ToolResult>; # 执行
}
```

#### 2.4 命令模块 (`commands/`)
```
commands/
├── model.rs          # /model 命令
├── provider.rs       # /provider 命令
├── theme.rs          # /theme 命令
├── compact.rs        # /compact 命令
├── reset.rs          # /reset 命令
├── exit.rs           # /exit 命令
├── config.rs         # /config 命令
├── skills.rs         # /skills 命令
├── skill.rs          # /skill 命令
├── stash.rs          # /stash 命令
├── restore.rs        # /restore 命令
└── ... (29个命令)
```

**命令 trait**:
```rust
trait Command {
    fn name(&self) -> &str;                  # 命令名称
    fn description(&self) -> &str;           # 命令描述
    fn execute(&self, args: &str) -> Result<CommandResult>; # 执行
}
```

#### 2.5 UI 模块 (`tui/`)
```
tui/
├── app.rs              # 应用主循环
├── ui.rs               # UI渲染
├── composer.rs         # Composer组件
├── transcript.rs       # 转录显示
├── statusline.rs       # 状态栏
├── help_overlay.rs     # 帮助覆盖层
├── theme_picker.rs     # 主题选择器
├── model_picker.rs     # 模型选择器
├── provider_picker.rs  # 提供商选择器
└── ... (63个UI组件)
```

**关键结构**:
```rust
struct App                         # 应用主结构
  - async fn run()                 # 运行应用
  - fn render()                    # 渲染UI
  - fn handle_event()              # 处理事件
  - fn handle_key()                # 处理按键
  
struct Composer                    # Composer组件
  - fn render()                    # 渲染composer
  - fn handle_input()              # 处理输入
  - fn submit()                    # 提交消息
  
struct Transcript                  # 转录显示
  - fn render()                    # 渲染转录
  - fn add_message()               # 添加消息
  - fn add_thinking()              # 添加思考块
```

#### 2.6 会话管理 (`session_manager.rs`)
```rust
struct SessionManager
  - fn create_session()            # 创建新会话
  - fn save_session()              # 保存会话
  - fn resume_session()            # 恢复会话
  - fn fork_session()              # 分叉会话
  - fn list_sessions()             # 列出会话
  
struct Session
  - id: Uuid                       # 会话ID
  - messages: Vec<ApiMessage>      # 消息历史
  - workspace: PathBuf             # 工作区路径
  - created_at: DateTime           # 创建时间
```

#### 2.7 任务管理 (`task_manager.rs`)
```rust
struct TaskManager
  - fn spawn_task()                # 生成任务
  - fn cancel_task()               # 取消任务
  - fn list_tasks()                # 列出任务
  - fn wait_for_task()             # 等待任务
  
struct BackgroundTask
  - id: TaskId
  - status: TaskStatus
  - result: Option<TaskResult>
```

#### 2.8 RLM 引擎 (`rlm/`)
```
rlm/
├── mod.rs            # RLM模块入口
├── session.rs        # RLM会话
├── python.rs         # Python执行
├── tools.rs          # RLM工具
└── config.rs         # RLM配置
```

```rust
struct RlmSession
  - fn open()                    # 打开会话
  - fn eval()                    # 执行Python
  - fn configure()               # 配置
  - fn close()                   # 关闭
  
# RLM工具
fn peek()                        # 预览数据
fn search()                      # 搜索
fn chunk()                       # 分块
fn sub_query_batch()             # 批量子查询
```

#### 2.9 LSP 支持 (`lsp/`)
```
lsp/
├── mod.rs            # LSP模块
├── client.rs         # LSP客户端
├── diagnostics.rs    # 诊断信息
└── config.rs         # LSP配置
```

```rust
struct LspClient
  - fn initialize()              # 初始化
  - fn get_diagnostics()         # 获取诊断
  - fn shutdown()                # 关闭
  
# 支持的语言服务器
- rust-analyzer                  # Rust
- pyright                        # Python
- typescript-language-server     # TypeScript
- gopls                          # Go
- clangd                         # C/C++
```

#### 2.10 沙箱系统 (`sandbox/`)
```
sandbox/
├── mod.rs            # 沙箱模块
├── seatbelt.rs       # macOS Seatbelt
├── landlock.rs       # Linux Landlock
├── job_object.rs     # Windows Job Objects
├── policy.rs         # 沙箱策略
├── config.rs         # 沙箱配置
└── executor.rs       # 沙箱执行器
```

```rust
trait SandboxBackend {
    fn initialize() -> Result<()>;
    fn execute_command() -> Result<CommandOutput>;
    fn restrict_filesystem() -> Result<()>;
}

enum SandboxMode {
    ReadOnly,              # 只读
    WorkspaceWrite,        # 工作区可写
    DangerFullAccess,      # 完全访问
}
```

---

### 3. `crates/core` - 核心共享逻辑

**职责**: cli 和 tui 共享的核心功能

**目录结构**:
```
crates/core/src/
└── lib.rs
```

**主要功能**:
- 共享工具函数
- 通用数据结构
- 错误处理

---

### 4. `crates/config` - 配置管理

**职责**: 配置文件解析、验证、合并

**目录结构**:
```
crates/config/src/
└── lib.rs
```

**主要功能**:
```rust
struct Config                # 配置主结构
  - fn load()                # 加载配置
  - fn save()                # 保存配置
  - fn validate()            # 验证配置
  - fn merge()               # 合并配置
  
struct ProviderRegistry      # 提供商注册表
  - fn register()            # 注册提供商
  - fn get()                 # 获取提供商配置
```

---

### 5. `crates/agent` - 智能体引擎

**职责**: AI智能体核心逻辑

**目录结构**:
```
crates/agent/src/
└── lib.rs
```

**主要功能**:
- 智能体生命周期管理
- 工具调用路由
- 上下文管理

---

### 6. `crates/tools` - 工具注册和执行

**职责**: 工具注册表和执行引擎

**目录结构**:
```
crates/tools/src/
└── lib.rs
crates/tools/tests/
└── tool_tests.rs
```

**主要功能**:
```rust
struct ToolRegistry            # 工具注册表
  - fn register()              # 注册工具
  - fn execute()               # 执行工具
  - fn list()                  # 列出工具
  
struct ToolResult              # 工具执行结果
  - status: Status
  - output: String
  - error: Option<String>
```

---

### 7. `crates/state` - 状态管理和持久化

**职责**: 应用状态持久化到 SQLite

**目录结构**:
```
crates/state/src/
└── lib.rs
crates/state/tests/
└── state_tests.rs
```

**主要功能**:
```rust
struct StateStore              # 状态存储
  - fn init_db()               # 初始化数据库
  - fn save_state()            # 保存状态
  - fn load_state()            # 加载状态
  - fn migrate()               # 数据库迁移
  
# 数据库表
- sessions                     # 会话表
- messages                     # 消息表
- tasks                        # 任务表
- settings                     # 设置表
```

---

### 8. `crates/protocol` - API协议定义

**职责**: OpenAI兼容API协议定义

**目录结构**:
```
crates/protocol/src/
└── lib.rs
crates/protocol/tests/
└── parity_protocol.rs
```

**主要功能**:
```rust
struct ChatCompletionRequest   # 聊天完成请求
  - model: String
  - messages: Vec<Message>
  - stream: bool
  - temperature: f32
  
struct ChatCompletionResponse  # 聊天完成响应
  - id: String
  - choices: Vec<Choice>
  - usage: Usage
  
struct Message                 # 消息
  - role: Role                 # system/user/assistant
  - content: Vec<ContentBlock>
  - tool_calls: Vec<ToolCall>
  
enum ContentBlock              # 内容块
  - Text(String)
  - Thinking(String)           # 推理内容
  - ToolCall(ToolCall)
```

---

### 9. `crates/mcp` - MCP协议支持

**职责**: Model Context Protocol 实现

**目录结构**:
```
crates/mcp/src/
└── lib.rs
```

**主要功能**:
```rust
struct McpClient               # MCP客户端
  - fn connect()               # 连接服务器
  - fn call_tool()             # 调用工具
  - fn list_tools()            # 列出工具
  - fn disconnect()            # 断开连接
  
struct McpServer               # MCP服务器配置
  - command: String
  - args: Vec<String>
  - env: HashMap<String, String>
```

---

### 10. `crates/hooks` - 钩子系统

**职责**: 事件钩子和回调

**目录结构**:
```
crates/hooks/src/
└── lib.rs
```

**主要功能**:
```rust
struct HookRegistry            # 钩子注册表
  - fn register()              # 注册钩子
  - fn trigger()               # 触发钩子
  
# 支持的钩子
- on_turn_start                # 轮次开始
- on_turn_end                  # 轮次结束
- on_tool_call                 # 工具调用
- on_message                   # 消息接收
```

---

### 11. `crates/secrets` - 密钥管理

**职责**: API Key 安全存储

**目录结构**:
```
crates/secrets/src/
└── lib.rs
```

**主要功能**:
```rust
struct SecretStore             # 密钥存储
  - fn save()                  # 保存密钥
  - fn load()                  # 加载密钥
  - fn delete()                # 删除密钥
  - fn list()                  # 列出密钥（仅显示后四位）
  
# 存储后端
- config_file                  # 配置文件
- keyring                      # 系统钥匙串
- environment                  # 环境变量
```

---

### 12. `crates/execpolicy` - 执行策略

**职责**: 命令执行策略和安全检查

**目录结构**:
```
crates/execpolicy/src/
├── lib.rs
└── bash_arity.rs              # Bash参数检查
```

**主要功能**:
```rust
struct ExecutionPolicy         # 执行策略
  - fn check_command()         # 检查命令
  - fn is_safe()               # 是否安全
  - fn risk_level()            # 风险等级
  
enum RiskLevel {
    Low,                       # 低风险（读取操作）
    Medium,                    # 中风险（写入工作区）
    High,                      # 高风险（系统修改）
    Danger,                    # 危险（不可逆操作）
}
```

---

### 13. `crates/app-server` - HTTP/SSE服务器

**职责**: 提供 HTTP/SSE API 用于无头模式

**目录结构**:
```
crates/app-server/src/
├── lib.rs
└── main.rs
```

**主要功能**:
```rust
struct AppServer               # 应用服务器
  - async fn start()           # 启动服务器
  - async fn stop()            # 停止服务器
  
# API端点
- POST /v1/chat/completions    # 聊天完成
- GET  /v1/models              # 列出模型
- GET  /health                 # 健康检查
  
# SSE事件
- message                      # 消息块
- thinking                     # 思考块
- tool_call                    # 工具调用
- done                         # 完成
```

---

### 14. `crates/tui-core` - TUI核心组件

**职责**: TUI的核心数据结构和算法

**目录结构**:
```
crates/tui-core/src/
└── lib.rs
crates/tui-core/tests/
└── tui_core_tests.rs
```

**主要功能**:
- 核心数据结构
- 算法实现
- 状态机

---

## 🎯 关键数据结构

### 消息结构
```rust
struct ApiMessage {
    role: Role,                      // system/user/assistant
    content: Vec<ContentBlock>,      // 内容块
    tool_calls: Option<Vec<ToolCall>>, // 工具调用
    reasoning_content: Option<String>, // 推理内容
}

enum ContentBlock {
    Text(String),                    // 文本
    Thinking(String),                // 思考
    ToolUse(ToolUse),                // 工具使用
    ToolResult(ToolResult),          // 工具结果
}
```

### 工具调用结构
```rust
struct ToolCall {
    id: String,                      // 调用ID
    name: String,                    // 工具名称
    arguments: serde_json::Value,    // 参数
}

struct ToolResult {
    status: ToolStatus,              // 状态
    output: String,                  // 输出
    error: Option<String>,           // 错误
    duration: Duration,              // 耗时
}
```

### 会话结构
```rust
struct Session {
    id: Uuid,                        // 会话UUID
    workspace: PathBuf,              // 工作区路径
    messages: Vec<ApiMessage>,       // 消息历史
    created_at: DateTime<Utc>,       // 创建时间
    updated_at: DateTime<Utc>,       // 更新时间
    model: String,                   # 使用模型
    mode: AppMode,                   # 模式（Plan/Agent/YOLO）
}
```

---

## 🔄 核心工作流

### 1. 启动流程
```
main.rs (cli)
  → parse_args()                     # 解析参数
  → load_config()                    # 加载配置
  → check_auth()                     # 检查认证
  → spawn deepseek-tui               # 启动TUI进程
```

### 2. 消息处理流程
```
Composer输入
  → parse_command()                  # 解析命令
  → build_prompt()                   # 构建提示
  → stream_chat()                    # 流式请求
  → parse_chunks()                   # 解析响应块
    → Text → 显示文本
    → Thinking → 显示思考
    → ToolCall → 执行工具
      → execute_tool()
      → add_result()
  → update_transcript()              # 更新转录
```

### 3. 工具执行流程
```
收到工具调用
  → check_safety()                   # 安全检查
  → request_approval()               # 请求批准（如果需要）
  → execute_in_sandbox()             # 在沙箱中执行
  → capture_output()                 # 捕获输出
  → return_result()                  # 返回结果
```

---

## 📊 模块依赖关系

```
cli ──────────────────┐
                      ├──→ core
tui ───┐              │
       ├──→ config ───┤
       ├──→ agent ────┤
       ├──→ tools ────┤
       ├──→ state ────┤
       ├──→ protocol ─┤
       ├──→ mcp ──────┤
       ├──→ hooks ────┤
       ├──→ secrets ──┤
       ├──→ execpolicy┤
       └──→ tui-core ─┘

app-server ──→ protocol
               core
               config
```

---

## 🎨 UI 组件层次

```
App
├── Header                         # 顶部栏
│   ├── ModelBadge                 # 模型徽章
│   ├── ModeBadge                  # 模式徽章
│   └── StatusIndicators           # 状态指示器
│
├── MainContent                    # 主内容区
│   ├── Transcript                 # 转录区
│   │   ├── MessageBlock           # 消息块
│   │   ├── ThinkingBlock          # 思考块
│   │   ├── ToolCallBlock          # 工具调用块
│   │   └── SystemMessage          # 系统消息
│   │
│   └── SidePanel (可选)           # 侧边栏
│       ├── WorkPanel              # 工作面板
│       └── TaskPanel              # 任务面板
│
├── Composer                       # 底部Composer
│   ├── InputArea                  # 输入区
│   ├── AttachmentBar              # 附件栏
│   └── CommandSuggestions         # 命令建议
│
└── Overlays                       # 覆盖层
    ├── HelpOverlay                # 帮助覆盖
    ├── ThemePicker                # 主题选择器
    ├── ModelPicker                # 模型选择器
    └── CommandPalette             # 命令面板
```

---

## 📝 命名约定

### 模块命名
- `snake_case` - 文件名、模块名
- `PascalCase` - 结构体、枚举、trait
- `snake_case` - 函数、方法、变量

### 文件组织
- 每个主要功能一个模块
- 相关功能放在子目录
- 测试文件在 `tests/` 目录

---

## 🔧 关键技术栈

| 技术 | 用途 |
|------|------|
| **ratatui** | TUI框架 |
| **tokio** | 异步运行时 |
| **serde/serde_json** | 序列化/反序列化 |
| **reqwest** | HTTP客户端 |
| **clap** | 命令行解析 |
| **rusqlite** | SQLite数据库 |
| **toml** | 配置文件解析 |
| **tracing** | 日志和追踪 |
| **uuid** | UUID生成 |
| **axum** | HTTP服务器 |

---

## 📖 相关文档

- [架构详解](../ARCHITECTURE.md) - 完整的架构 walkthrough
- [配置参考](../CONFIGURATION.md) - 配置选项详解
- [工具接口](../TOOL_SURFACE.md) - 可用工具列表
- [MCP协议](../MCP.md) - MCP集成文档

---

*文档最后更新: 2026-05-16*
*版本: v0.8.37*
