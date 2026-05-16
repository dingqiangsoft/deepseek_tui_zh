# Modes and Approvals

DeepSeek TUI has two related concepts:

- **TUI mode**: what kind of visible interaction you're in (Plan/Agent/YOLO).
- **Approval mode**: how aggressively the UI asks before executing tools.

## TUI Modes

### 切换模式的方法

#### 方法 1: 快捷键

- **`Tab`**: 循环切换模式（Plan → Agent → YOLO → Plan）
- **`Shift+Tab`**: 循环切换推理强度（off → high → max → off）

#### 方法 2: Slash 命令

```bash
# 打开模式选择器
/mode

# 直接切换到指定模式
/mode plan      # 切换到 Plan 模式
/mode agent     # 切换到 Agent 模式
/mode yolo      # 切换到 YOLO 模式

# 使用数字快捷方式
/mode 1         # = Plan 模式
/mode 2         # = Agent 模式
/mode 3         # = YOLO 模式
```

#### 方法 3: 启动参数

```bash
# 启动时直接进入 YOLO 模式
deepseek --yolo

# 启动时指定模式
deepseek --mode agent
```

### 模式说明

- **Plan** (计划模式): 设计优先。只读工具可用，Shell 和文件写入关闭。适合思考和制定计划。
- **Agent** (代理模式): 多步工具使用。Shell 和付费工具需要批准，文件写入允许。
- **YOLO** (自由模式): 自动批准所有工具，启用 Shell 和信任模式。仅在可信仓库使用。

All three modes have access to persistent RLM sessions through `rlm_open`, `rlm_eval`, `rlm_configure`, and `rlm_close`. Inside an RLM Python REPL, `sub_query_batch` fans out 1-16 cheap parallel child calls pinned to `deepseek-v4-flash`. The model reaches for it when work is too large or repetitive for the parent transcript.

## Compatibility Notes

- Older settings files with `default_mode = "normal"` still load as `agent`; saving rewrites the normalized value.

## Escape Key Behavior

`Esc` is a cancel stack, not a mode switch.

- Close slash menus or transient UI first.
- Cancel the active request if a turn is running.
- Discard a queued draft if the composer is empty.
- Clear the current input if text is present.
- Otherwise it is a no-op.

## Approval Mode (批准模式)

### 设置方法

#### 方法 1: 使用 /config 命令

```bash
# 查看当前配置
/config

# 设置批准模式（编辑 approval_mode 行）
/config approval_mode auto      # 自动批准所有工具
/config approval_mode suggest   # 建议批准（默认）
/config approval_mode never     # 从不执行需要批准的工具
```

#### 方法 2: 使用别名

```bash
# 这些命令等价于 /config approval_mode suggest
/config approval_mode on-request
/config approval_mode untrusted

# 这些命令等价于 /config approval_mode never
/config approval_mode deny
/config approval_mode denied
```

### 批准模式说明

| 模式 | 说明 | 适用场景 |
|------|------|---------|
| `suggest` (默认) | 非安全工具需要确认 | 日常使用，安全优先 |
| `auto` | 自动批准所有工具 | 高效开发，可信环境 |
| `never` | 阻止所有需要批准的工具 | 只读场景，严格安全 |

### 与 TUI 模式的关系

- **Plan 模式**: 无论 approval_mode 如何，Shell 和写入工具都被禁用
- **Agent 模式**: 遵循 approval_mode 设置
- **YOLO 模式**: 自动设置为 `auto`，忽略 approval_mode 设置

### 示例

```bash
# 场景 1: 高效开发（自动批准）
/mode agent
/config approval_mode auto

# 场景 2: 安全优先（每次确认）
/mode agent
/config approval_mode suggest

# 场景 3: 只读调查
/mode plan
# 无需设置，Plan 模式本身就不允许写入

# 场景 4: 完全自由
/mode yolo
# 自动启用 auto 批准
```

---

**历史说明**: `/set approval_mode ...` 命令已废弃，请使用 `/config` 代替。

## Small-Screen Status Behavior

When terminal height is constrained, the status area compacts first so header/chat/composer/footer remain visible:

- Loading and queued status rows are budgeted by available height.
- Queued previews collapse to compact summaries when full previews do not fit.
- `/queue` workflows remain available; compact status only affects rendering density.

## Workspace Boundary and Trust Mode

By default, file tools are restricted to the `--workspace` directory. Enable trust mode to allow file access outside the workspace:

```text
/trust
```

YOLO mode enables trust mode automatically.

## MCP Behavior

MCP tools are exposed as `mcp_<server>_<tool>` and use the same approval flow as built-in tools. Read-only MCP helpers may auto-run in suggestive approval modes; MCP tools with possible side effects require approval.

See `MCP.md`.

## Related CLI Flags

Run `deepseek --help` for the canonical list. Common flags:

- `-p, --prompt <TEXT>`: one-shot prompt mode (prints and exits)
- `deepseek exec --output-format stream-json <PROMPT>`: emit one JSON object per line for harnesses and backend wrappers
- `deepseek exec --resume <ID|PREFIX> <PROMPT>` / `--session-id <ID|PREFIX>`: continue a saved session non-interactively
- `deepseek exec --continue <PROMPT>`: continue the most recent saved session for this workspace non-interactively
- `--model <MODEL>`: when using the `deepseek` facade, forward a DeepSeek model override to the TUI
- `--workspace <DIR>`: workspace root for file tools
- `--yolo`: start in YOLO mode
- `-r, --resume <ID|PREFIX|latest>`: resume a saved session
- `-c, --continue`: resume the most recent session in this workspace
- `--max-subagents <N>`: clamp to `1..=20`
- `--mouse-capture` / `--no-mouse-capture`: opt in or out of internal mouse scrolling, transcript selection, right-click context actions, and transcript scrollbar dragging. Mouse capture is enabled by default on non-Windows terminals and on Windows Terminal/ConEmu/Cmder so drag selection copies only transcript text and stays scoped to the transcript pane; hold Shift while dragging or use `--no-mouse-capture` for raw terminal selection. It defaults off on legacy Windows console (CMD without `WT_SESSION` / `ConEmuPID`) and inside JetBrains JediTerm — PyCharm/IDEA/CLion/etc. — where the terminal advertises mouse support but forwards SGR mouse events as raw text (#878, #898). Use `--mouse-capture` to opt in anywhere it's defaulted off. Raw terminal selection may cross the right sidebar because the terminal, not the TUI, owns the selection.
- `--profile <NAME>`: select config profile
- `--config <PATH>`: config file path
- `-v, --verbose`: verbose logging
