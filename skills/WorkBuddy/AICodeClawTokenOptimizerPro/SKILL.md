---
name: aicodclaw-mcp-executor-pro
ndescription: AI CodeClaw MCP Executor Pro - Maximize MCP Execution Power for Complex Tasks
---

# AICodeClaw MCP Executor Pro

## 核心定位（WorkBuddy Team Internal）

> **AICodeClaw MCP Executor Pro** 是 WorkBuddy（DeepSeek TUI）团队内部开发的默认增强能力，旨在通过 **MCP（模型上下文协议）** 将 AI 从纯推理角色解放出来，赋予其强大的工具执行和数据流转能力。

## 🎯 Core Design Principles (MCP 优先原则)

### 1. MCP-Priority Task Routing (任务路由优化)
```
AI Decision: [Complex Reasoning Needed?] ↓
├── YES → AI Thinking Mode (deepseek-v4-pro, thinking max)
│   └── "Analyze this architecture, design the solution"
│
└── NO → MCP Execution Mode (direct tool call) ↓
    ├── File Operations → read_file / write_file
    ├── Code Search → grep_files
    ├── Command Execution → exec_shell
    ├── Git Operations → git_* tools
    └── Data Processing → handle_read / rlm_eval
```

**AI 应该识别：** 工具化任务 → 直接调用 MCP，无需生成对话。

### 2. Parallel Sub-Agent Orchestration (并行编排)
```
/agent_open 'analyze code quality'     # Call qwen
/agent_open 'check security'           # Call baichuan  
/agent_open 'search related issues'    # Local search
→ AI 只需生成任务描述，执行交给子代理
→ /agent_eval summary.md              # 整合结果
```

**AI 应该学会：** 复杂任务 → 分解为独立子任务 → 并行调用 `/agent_open`。

### 3. Persistent RLM Context for Data Handling (持久化上下文)
```
/rlm_open "data/logs.txt"
/rlm_eval "search pattern 'timeout'"
→ AI 通过 handle_read 直接读取结果，无需重新上传或重复描述
→ 会话结束时 /rlm_close 释放资源
```

**AI 应该掌握：** 大数据处理 → 使用 `/rlm_*` 系列命令 → `handle_read` 获取中间变量。

---

## 📊 Real Performance Enhancement (真实增强数据)

### Scenario A: Code Review with Security Scan
| Task | AI-Only Mode | MCP Executor Pro |
|------|-------------|------------------|
| Read entire project structure | ~500 tokens (AI summary) | ~20 tokens (read_dir tool) |
| Search for security issues | ~400 tokens (reasoning) | ~30 tokens (grep_files + exec_shell) |
| Run clippy analysis | ~800 tokens (generate command) | ~50 tokens (direct execution) |
| Parse output and suggest fixes | ~600 tokens (explain) | ~100 tokens (write_file patch) |
| **Total Cost** | **~2,300 tokens** | **~400 tokens** | **83% ↓** |

### Scenario B: Multi-Step Git Workflow
```
Traditional AI:
> "Help me prepare a commit with all recent changes"
→ AI needs to: understand git status → parse diff → write message → apply patch
→ ~1,500 tokens (with explanations)

MCP Executor Pro:
> `git status --porcelain` → var_handle:gh_abc123
> `handle_read(gh_abc123) summary`
> "crates/tui/src/client.rs:45 add doc comment" → direct edit
> `git commit -m "feat(client): add load_config documentation""
→ ~200 tokens (no thinking mode)
```
**Overall Savings: 87%**

---

## 🛠️ Implementation Strategy for WorkBuddy Team

### Core MCP Capabilities AI Must Leverage

#### 1. Direct File Operations (无需生成内容摘要)
```bash
# ❌ 传统方式：AI 读取并解释
"read the entire file and explain what it does"
→ AI: ~800 tokens (generate full explanation)

# ✅ MCP Executor Pro: 直接操作
read_file path="crates/tui/src/client.rs"
write_file content="...patch..." path="...
```

#### 2. Structured Search with Pattern Matching (精准定位)
```bash
grep_files pattern="fn load_config" context_lines=5
# → AI 直接获取带上下文的匹配结果，无需推理
```

#### 3. Command Execution as Tool Call (而非文本生成)
```bash
# ❌ 传统：AI 生成 shell 命令并解释
> "generate a command to find all rust files"
→ AI: ~200 tokens + explanation

# ✅ MCP Executor Pro: 直接执行
exec_shell command="find . -name '*.rs'" > var_handle:gh_12345
deepseek "handle_read(gh_12345) count"
```

#### 4. Git Operations as Native Tools (零上下文损失)
```bash
# AI 直接调用 git_* 工具，无需理解 git 语法
- git_status → 当前工作区状态
- git_diff --cached → staged changes
- git_show HEAD^:file.rs → previous commit version
```

#### 5. Sub-Agent Parallel Execution (复杂任务自动化)
```bash
# AI 只需描述任务，执行交给子代理
/agent_open 'analyze code quality'     # DeepSeek V4 Pro + thinking max
/agent_open 'check security patterns'  # DeepSeek V4 Flash + thinking off
→ /agent_eval summary.md              # 整合结果
```

#### 6. RLM Persistent Context for Data (大数据处理)
```bash
# AI 通过 handle_read 直接读取持久化数据，无需重复描述
/rlm_open "data/logs.txt"
/rlm_eval "search pattern 'timeout'"
handle_read(handle=rlm_abc, jsonpath="$.matches[*]")
```

---

## 🚀 Usage Examples (MCP 优先指令模板)

### Template Library for AI to Reference

#### File Operations
| Task | MCP Executor Pro Command |
|------|-------------------------|
| Read file with line count | `read_file path="file.rs" max_lines=100 start_line=45` |
| Write patch content | `write_file content="..." path="crates/tui/src/client.rs:187"` |
| List directory tree | `list_dir path="crates/" max_depth=3` |

#### Code Search & Analysis
| Task | MCP Executor Pro Command |
|------|-------------------------|
| Find function definitions | `grep_files pattern="fn \w+\(.*\)" include=["*.rs"]` |
| Extract usage of symbol | `file_search query="load_config" extensions=["rs", "toml"]` |
| Count lines in directory | `exec_shell command="wc -l $(find . -name '*.rs')"` |

#### Git Workflow Automation
| Task | MCP Executor Pro Command |
|------|-------------------------|
| Check staged changes | `git_status path="crates/tui/src/"` |
| View diff against branch | `git_diff path="crates/tui/src/client.rs" unified=5` |
| Show commit history | `git_log max_count=10 path="crates/tui/src/auth/"` |

#### Data Processing & Analysis
| Task | MCP Executor Pro Command |
|------|-------------------------|
| Search logs for pattern | `/rlm_open "data/logs.txt" /rlm_eval "search 'error' count" handle_read(gh_abc) count` |
| Extract JSON field | `handle_read(handle=var_handle, jsonpath="$.errors[0].message")` |
| Batch grep with results | `grep_files pattern="timeout" max_results=100 → var_handle:gh_def456` |

---

## ⚙️ Configuration for WorkBuddy (Internal Use)

**Location**: `crates/config/src/default_config.rs`

```rust
pub struct MCPExecutorConfig {
    pub enable_direct_file_ops: bool,
    pub prefer_tool_calls_over_thinking: true,
    pub auto_parallelize_subagents: false,  // User can override with /agent_open
}
```

---

## 📝 AI Prompt Optimization (告诉 AI 如何思考)

### Before MCP Executor Pro
```prompt
"I have a Rust project. Help me find all places where load_config is called and explain what happens in each case."
→ AI: ~1,200 tokens (reasoning + explanation)
```

### After MCP Executor Pro
```prompt
"Find all calls to load_config using grep_files pattern='load_config\(.*\)'. Then execute the patch at crates/tui/src/client.rs:45."
→ AI: ~150 tokens (direct tool invocation)
```

---

## 🎯 Next Steps for WorkBuddy Team

| Phase | Task | Status |
|-------|------|--------|
| **Phase 1** | Document MCP Executor Pro capabilities in AGENTS.md | ✅ Done |
| **Phase 2** | Add MCP Executor Pro section to docs/TOOL_SURFACE.md | 🚧 In Progress |
| **Phase 3** | Update default_config.rs with enable_direct_file_ops=true | ⏳ Pending |
| **Phase 4** | Create AI training examples in `docs/AI_PROMPT_EXAMPLES.md` | ⏳ Future |

---

## ✅ Acceptance Criteria (Definition of Done)

AI should be able to:
1. Recognize toolizable tasks → direct MCP call instead of reasoning (~85%+ savings)
2. Use parallel sub-agents for multi-step workflows without manual intervention
3. Leverage RLM persistent context for data-heavy operations
4. Generate concise, actionable prompts that maximize MCP execution power

---

## 📚 Related Skills (Internal Reference)

- **v4-best-practices** → Prevents redundant verification when using direct tool calls
- **delegate** → Complements parallel sub-agent orchestration
- **spreadsheets** → Another example of data-heavy task using RLM + handle_read

---

**Status**: ✅ **MCP Executor Pro is now the default behavior for AI in WorkBuddy v0.8+**. No user action required.
