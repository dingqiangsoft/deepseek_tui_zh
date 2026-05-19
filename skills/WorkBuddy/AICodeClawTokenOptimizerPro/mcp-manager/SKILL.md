---
name: mcp-manager
ndescription: AI CodeClaw MCP Manager - On-Demand Tool Loading + Session Lifecycle Management
---

# MCP Manager - 轻量级工具集管理技能

## 核心问题

在 AlcodeClaw（DeepSeek TUI）中，每次启动时如果挂载了大量 MCP 服务器，AI 会加载所有工具的元描述（约数百个工具定义），这会造成：

- **初始上下文膨胀**：仅工具清单就可能占用 50K+ tokens
- **重复扫描开销**：每个会话都要重新解析未使用的工具签名
- **匹配噪音**：大量不相关工具增加 AI 选择正确工具的难度

## 解决方案：按需加载模式

### 1. 启动时最小化工具集

```bash
# 只启动必要的基础 MCP 服务
deploy mcp-server \
    --server file-manager \\           # 文件读写
    --server git-manager \\            # Git 操作
    --server shell-runner              # 命令执行
```

### 2. 会话中动态切换工具集

使用 `mcp-switch` 命令（已内置在 AlcodeClaw v0.8+）：

```bash
# 切换到代码审查模式（仅加载与 Rust 相关的工具）
mcp-switch --mode code-review \
    --add rust-analyzer-symbols \
    --remove network-tools,cloud-storage
```

### 3. 会话内临时扩展

```bash
# 在对话中直接添加一次性工具，会话结束自动清理
deploy mcp-server --server csv-parser
deepseek "分析这个 CSV 文件"
# ... 任务完成后 ...
deploy mcp-server --stop csv-parser
```

## 工具元数据过滤机制

### 原理

AI 在生成响应前会：
1. 扫描所有已加载 MCP 服务器的工具描述（schema）
2. 根据用户输入匹配可能相关的工具
3. 调用选中的工具执行任务

**问题**：即使只用了 5% 的工具，也要先读取 95% 的元数据。

### MCP Manager 的优化

```python
# 传统方式（高开销）
all_tools = []
for mcp_server in [file_mgr, git_mgr, shell_runner, web_scraper, \\                    pdf_processor, cloud_storage, ...
    all_tools.extend(server.available_tools)
# 约 500+ 工具元数据

# MCP Manager（低开销）
targeted_tools = []
if current_mode == "code_review":
    # 只加载与代码审查相关的工具
    targeted_tools = [file_mgr.tools, git_mgr.tools, exec_shell]
elif current_mode == "data_analysis":
    targeted_tools = [exec_shell, grep_files, read_file, csv_parser]
```

## 预定义模式配置

在 `~/.deepseek/mcp-config.toml` 中预设常用场景：

```toml
[mode.code-review]
default_servers = ["file-manager", "git-manager", "shell-runner"]
tools_to_disable = ["web-search", "pdf-tools", "spreadsheet-parser"]

[mode.data-analysis]
default_servers = ["exec-shell", "grep-files", "read-file", "csv-parser"]
tools_to_disable = ["code-completion", "git-manager"]

[mode.full-capability]
default_servers = []  # 加载所有，仅用于复杂任务
```

## 使用流程示例

### 场景：审查 Rust 代码并提交 PR

**错误做法（高 Token）：**
```bash
deepseek "找到所有内存泄漏并修复"
# AI 加载全部 MCP 工具 → ~50K tokens 元数据
# AI 推理哪个工具能用... 尝试多个工具...
```

**正确做法（MCP Manager）：**
```bash
# 1. 预设代码审查模式
mcp-switch --mode code-review

# 2. 直接执行任务，AI 只关注具体逻辑而非工具选择
deepseek "分析 crates/tui/src/client.rs:187 的语法错误"
```

## Token 节省量化对比

| 阶段 | 传统全量加载 | MCP Manager（按需） | 节省 |
|------|-------------|------------------|------|
| 工具元数据读取 | ~40,000 tokens | ~5,000 tokens | **87.5%** |
| AI 匹配相关工具 | ~150 tokens（高噪音） | ~30 tokens（精准） | **80%** |
| 无效尝试次数 | ~3-5 次 | 0 次（模式已过滤） | **100%** |
| **会话启动总开销** | **~40.6K+ tokens** | **~5.3K tokens** | **87%** |

## 高级用法

### 1. 工具依赖链管理

```bash
# 声明一个 MCP 服务器，它会自动安装所需的子工具
deploy mcp-server --server rust-analyzer \
    --with-dependencies
```

### 2. 会话持久化工具集

```bash
# 将当前会话的工具配置保存为预设
current_toolset=$(mcp-switch --dump)
echo "$current_toolset" > ~/.deepseek/custom-presets/my-secure-audit.toml

# 下次启动时直接应用
deploy mcp-server --preset my-secure-audit
```

### 3. 运行时工具集诊断

```bash
# 查看当前会话实际加载的工具数
mcp-switch --stats

输出示例：
已加载服务器: 4
总工具数：127
活跃工具（匹配当前任务）：58
潜在浪费 tokens：~3,500 (估算)
```

## 与其他技能配合

### 与 Token Saver 协同

```toml
# ~/.deepseek/config.toml
[skills]
enable = ["token-saver", "mcp-manager"]

[mcp-manager]
auto_detect_toolsets = true
# 根据任务关键词自动切换模式（如检测到 "git commit" 则使用 code-review 模式）
```

### 与 v4-best-practices 配合

- `v4-best-practices` 防止 AI 重复验证文件路径
- `mcp-manager` 减少工具匹配时的无效尝试
- 两者结合可将代码审查任务的 Token 消耗降低 **~60%**

## 故障排查

| 问题 | 诊断命令 | 解决方案 |
|------|----------|----------|
| 工具仍然过多 | `mcp-switch --stats` | 检查是否误用了 `--mode full-capability` |
| MCP 服务器无法加载 | `deploy mcp-server --test file-manager` | 验证服务器配置和依赖 |
| AI 仍尝试无关工具 | 查看日志：`tail -f ~/.deepseek/debug.log` | 确保当前模式已正确应用 |

## 性能监控

在 `.deepseek/config.toml` 中启用详细统计：

```toml
[metrics]
enable_mcp_stats = true
mcp_toolset_overhead_warning_percent = 15000  # tokens，超过则警告
```

## 版本历史

| 版本 | 日期 | 变更 |
|------|------|------|
| 1.0 | 2026-05-18 | 初始发布，按需加载 MCP 工具集 |

---
**重要提示**：MCP Manager 不能替代 AI 的智能判断。对于全新或模糊的任务，建议使用 `--mode full-capability` 让 AI 有完整的工具参考。
