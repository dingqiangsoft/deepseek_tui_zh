# DeepSeek TUI 快速入门指南

## 📦 安装

### 方法 1: npm（推荐，最简单）

```bash
npm install -g deepseek-tui
```

### 方法 2: Cargo

```bash
cargo install deepseek-tui-cli --locked   # `deepseek`（入口点）
cargo install deepseek-tui     --locked   # `deepseek-tui`（TUI 二进制文件）
```

### 方法 3: Homebrew（macOS）

```bash
brew tap Hmbown/deepseek-tui
brew install deepseek-tui
```

### 方法 4: 直接下载

访问 [GitHub Releases](https://github.com/Hmbown/DeepSeek-TUI/releases) 下载预编译二进制文件。

### 中国大陆用户加速安装

```toml
# ~/.cargo/config.toml
[source.crates-io]
replace-with = "tuna"

[source.tuna]
registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
```

然后使用 npm：
```bash
npm install -g deepseek-tui --registry=https://registry.npmmirror.com
```

---

## 🚀 快速开始

### 1. 首次启动

```bash
deepseek
```

首次启动时会提示输入 DeepSeek API Key，密钥会保存到 `~/.deepseek/config.toml`。

### 2. 设置 API Key

```bash
# 方法 1: 交互式设置
deepseek auth set --provider deepseek

# 方法 2: 环境变量
export DEEPSEEK_API_KEY="YOUR_KEY"

# 方法 3: 检查状态
deepseek auth status
```

### 3. 验证安装

```bash
deepseek --version    # 查看版本
deepseek doctor       # 检查安装和连接
```

---

## 🎯 基本使用

### 启动方式

```bash
# 交互式 TUI（默认）
deepseek

# 使用 Auto 模式（自动选择模型和推理级别）
deepseek --model auto

# YOLO 模式（自动批准所有工具）
deepseek --yolo

# 一次性提示
deepseek "解释这个函数"

# 指定模型
deepseek --model deepseek-v4-flash "总结这个文件"
```

### 本地 LLM 配置

```bash
# 配置 OpenAI 兼容的本地服务
deepseek auth set --provider openai --api-key "not-needed"

# 启动（设置环境变量）
export DEEPSEEK_BASE_URL="http://localhost:1234/v1"
export DEEPSEEK_ALLOW_INSECURE_HTTP=1
deepseek --provider openai --model "your-local-model"
```

---

## 🎮 三种模式

| 模式 | 说明 | 启动方式 |
|------|------|---------|
| **Plan** 🔍 | 只读调查模式，模型先探索并提出计划 | 默认模式 |
| **Agent** 🤖 | 默认交互模式，多步骤工具使用需确认 | `/mode agent` |
| **YOLO** ⚡ | 自动批准所有工具，适合受信任的工作区 | `deepseek --yolo` |

在 TUI 中切换模式：
```
/mode agent   # 切换到 Agent 模式
/mode auto    # 切换到 Auto 模式
/mode yolo    # 切换到 YOLO 模式
```

---

## ⌨️ 常用快捷键

| 快捷键 | 功能 |
|--------|------|
| `Tab` | 自动补全 `/` 或 `@` 命令 |
| `Shift+Tab` | 切换推理级别：off → high → max |
| `F1` | 帮助面板 |
| `Esc` | 返回/关闭 |
| `Ctrl+K` | 命令面板 |
| `Ctrl+R` | 恢复之前的会话 |
| `Ctrl+S` | 暂存当前草稿 |
| `@path` | 附加文件/目录上下文 |
| `↑` | 选择附件行以删除 |

查看完整快捷键列表：[KEYBINDINGS.md](KEYBINDINGS.md)

---

## 🔧 常用命令

### 在 TUI 中

```
/model auto          # 自动选择模型
/provider openai     # 切换提供商
/theme dark          # 切换主题
/cd <目录>           # 切换工作目录（部分支持）
/compact             # 压缩上下文
/reset               # 重置会话
/exit                # 退出程序
```

### 在终端中

```bash
deepseek doctor                    # 诊断安装问题
deepseek models                    # 列出可用模型
deepseek sessions                  # 列出已保存的会话
deepseek resume --last             # 恢复最近的会话
deepseek mcp list                  # 列出 MCP 服务器
deepseek update                    # 更新到最新版本
```

---

## 📁 配置文件

### 用户配置

位置：`~/.deepseek/config.toml`

示例配置：
```toml
provider = "deepseek"
default_text_model = "deepseek-v4-pro"
auth_mode = "api_key"

[providers.deepseek]
api_key = "your-api-key"
```

### 项目配置

位置：`<工作区>/.deepseek/config.toml`

注意：项目配置中不允许设置 `api_key`、`base_url`、`provider`。

---

## 🌐 环境变量

| 变量 | 说明 |
|------|------|
| `DEEPSEEK_API_KEY` | API 密钥 |
| `DEEPSEEK_BASE_URL` | API 基础 URL |
| `DEEPSEEK_MODEL` | 默认模型 |
| `DEEPSEEK_PROVIDER` | 提供商（deepseek/openai/ollama 等） |
| `DEEPSEEK_ALLOW_INSECURE_HTTP=1` | 允许非 localhost 的 HTTP 连接 |
| `DEEPSEEK_STREAM_IDLE_TIMEOUT_SECS` | 流空闲超时（秒），默认 300 |

---

## 🤖 支持的提供商

```bash
# DeepSeek（默认）
deepseek --provider deepseek

# NVIDIA NIM
deepseek auth set --provider nvidia-nim --api-key "YOUR_KEY"
deepseek --provider nvidia-nim

# OpenRouter
deepseek auth set --provider openrouter --api-key "YOUR_KEY"
deepseek --provider openrouter

# 本地 Ollama
ollama pull deepseek-coder:1.3b
deepseek --provider ollama --model deepseek-coder:1.3b

# 本地 SGLang
SGLANG_BASE_URL="http://localhost:30000/v1" deepseek --provider sglang

# 本地 vLLM
VLLM_BASE_URL="http://localhost:8000/v1" deepseek --provider vllm
```

---

## 📚 更多文档

| 文档 | 说明 |
|------|------|
| [ARCHITECTURE.md](ARCHITECTURE.md) | 代码架构 |
| [CONFIGURATION.md](CONFIGURATION.md) | 完整配置参考 |
| [MODES.md](MODES.md) | 模式详解 |
| [MCP.md](MCP.md) | MCP 协议集成 |
| [SUBAGENTS.md](SUBAGENTS.md) | 子智能体 |
| [MEMORY.md](MEMORY.md) | 用户记忆 |
| [DOCKER.md](DOCKER.md) | Docker 使用 |

---

## 💡 常见问题

### Q: 如何切换工作目录？

目前 `/cd` 命令支持有限。推荐方法：
1. 退出 TUI：`/exit`
2. 在终端中 `cd` 到目标目录
3. 重新启动 `deepseek`

或创建专用的启动脚本。

### Q: 工具执行被拒绝怎么办？

使用 `--yolo` 参数启动，或在 TUI 中输入：
```
/mode auto
```

### Q: 如何复制聊天区内容？

- 鼠标选择后按 `Ctrl+Shift+C`
- 或让 AI 生成文件：`请将分析保存为 report.md`

### Q: 会话变慢了怎么办？

```
/compact     # 压缩上下文
/reset       # 或重置会话
```

---

## 🆘 获取帮助

- 文档：[docs/](../docs/)
- 问题反馈：[GitHub Issues](https://github.com/Hmbown/DeepSeek-TUI/issues)
- 社区讨论：[GitHub Discussions](https://github.com/Hmbown/DeepSeek-TUI/discussions)

---

## 📝 许可

[MIT License](../LICENSE)

*本项目与 DeepSeek Inc. 无关*
