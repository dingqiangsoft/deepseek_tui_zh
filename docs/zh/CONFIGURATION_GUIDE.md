# DeepSeek TUI 配置指南

## 📍 配置文件位置

### 全局配置
```
~/.deepseek/config.toml
```

### 项目配置（可选）
```
<工作区>/.deepseek/config.toml
```

### 命令行覆盖
```bash
deepseek --config /path/to/config.toml
```

### 环境变量覆盖
```bash
export DEEPSEEK_CONFIG_PATH=/path/to/config.toml
```

优先级：**命令行 > 环境变量 > 配置文件**

---

## 🔑 基本配置示例

### 最小配置

```toml
provider = "deepseek"
default_text_model = "deepseek-v4-pro"
auth_mode = "api_key"

[providers.deepseek]
api_key = "your-api-key-here"
```

### 本地 LLM 配置

```toml
provider = "openai"
default_text_model = "your-local-model"
auth_mode = "api_key"

[providers.openai]
base_url = "http://localhost:1234/v1"
api_key = "not-needed"
```

### 多个提供商配置

```toml
provider = "deepseek"
default_text_model = "deepseek-v4-pro"

[providers.deepseek]
api_key = "your-deepseek-key"

[providers.openai]
base_url = "http://localhost:1234/v1"
api_key = "not-needed"

[providers.ollama]
base_url = "http://localhost:11434/v1"
```

---

## 🌐 环境变量

### 核心变量

| 变量 | 说明 | 示例 |
|------|------|------|
| `DEEPSEEK_API_KEY` | API 密钥 | `sk-xxx` |
| `DEEPSEEK_BASE_URL` | API 基础 URL | `https://api.deepseek.com/beta` |
| `DEEPSEEK_MODEL` | 默认模型 | `deepseek-v4-pro` |
| `DEEPSEEK_PROVIDER` | 提供商 | `deepseek`/`openai`/`ollama` |
| `DEEPSEEK_ALLOW_INSECURE_HTTP` | 允许 HTTP | `1` |

### 本地 LLM 专用

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `OPENAI_BASE_URL` | OpenAI 兼容端点 | `https://api.openai.com/v1` |
| `OPENAI_API_KEY` | OpenAI 密钥 | - |
| `OLLAMA_BASE_URL` | Ollama 端点 | `http://localhost:11434/v1` |
| `SGLANG_BASE_URL` | SGLang 端点 | - |
| `VLLM_BASE_URL` | vLLM 端点 | - |

### 性能调优

| 变量 | 说明 | 默认值 | 范围 |
|------|------|--------|------|
| `DEEPSEEK_STREAM_IDLE_TIMEOUT_SECS` | 流空闲超时 | `300` | `1-3600` |
| `DEEPSEEK_MEMORY` | 启用用户记忆 | `off` | `on`/`off` |
| `NO_ANIMATIONS` | 无障碍模式 | `0` | `1` |

### 自定义请求头

```bash
export DEEPSEEK_HTTP_HEADERS="X-Model-Provider-Id=your-provider"
```

---

## 🎛️ 项目配置覆盖

工作区中的 `.deepseek/config.toml` 可以覆盖以下设置：

| 键 | 效果 |
|---|------|
| `provider` | 切换后端（如 `"nvidia-nim"`） |
| `model` | 覆盖 `default_text_model` |
| `api_key` | 使用项目专用密钥（通常从 `.env` 读取） |
| `base_url` | 指向自托管端点 |
| `reasoning_effort` | 强制 `"high"`/`"max"` |
| `approval_policy` | `"never"`/`"on-request"`/`"untrusted"` |
| `sandbox_mode` | `"read-only"`/`"workspace-write"`/`"danger-full-access"` |
| `max_subagents` | 限制并发数（1-20） |
| `allow_shell` | 控制 shell 工具访问 |

**注意**：项目配置中**不允许**设置敏感字段。

跳过项目配置：
```bash
deepseek --no-project-config
```

---

## 🔐 认证管理

### 设置 API Key

```bash
# DeepSeek
deepseek auth set --provider deepseek

# OpenAI 兼容
deepseek auth set --provider openai --api-key "your-key"

# NVIDIA NIM
deepseek auth set --provider nvidia-nim --api-key "your-key"

# OpenRouter
deepseek auth set --provider openrouter --api-key "your-key"
```

### 查看状态

```bash
deepseek auth status
```

显示：
- 配置文件中的密钥
- 系统钥匙串中的密钥
- 环境变量中的密钥
- 当前使用的来源
- 密钥后四位（不显示完整密钥）

### 清除密钥

```bash
deepseek auth clear --provider deepseek
```

---

## 🎨 界面配置

### 语言设置

```toml
# config.toml
[settings]
locale = "zh-Hans"
```

或在 TUI 中：
```
/config locale zh-Hans
```

支持的语言：
- `en` - 英语
- `zh-Hans` - 简体中文
- `ja` - 日语
- `pt-BR` - 葡萄牙语（巴西）

### 主题设置

在 TUI 中：
```
/theme dark          # 深色
/theme light         # 浅色
/theme catppuccin    # Catppuccin
/theme tokyo-night   # Tokyo Night
/theme dracula       # Dracula
/theme gruvbox       # Gruvbox
```

---

## 🤖 模型配置

### 支持的模型

```toml
# DeepSeek V4 Pro（推荐）
default_text_model = "deepseek-v4-pro"

# DeepSeek V4 Flash（快速/便宜）
default_text_model = "deepseek-v4-flash"

# 旧版别名（2026年7月24日后废弃）
default_text_model = "deepseek-chat"      # 映射到 flash
default_text_model = "deepseek-reasoner"  # 映射到 flash
```

### Auto 模式

```toml
# 让 TUI 自动选择
default_text_model = "auto"
```

Auto 模式会根据任务复杂度自动选择：
- **简单任务**：`deepseek-v4-flash` + 推理关闭
- **复杂任务**：`deepseek-v4-pro` + 高级推理

---

## ⚙️ 高级配置

### 推理级别

```toml
[settings]
reasoning_effort = "high"  # off / high / max
```

在 TUI 中用 `Shift+Tab` 快速切换。

### MCP 服务器

```toml
[mcp]
config_path = "~/.deepseek/mcp_config.json"
```

详见 [MCP.md](MCP.md)。

### 子智能体

```toml
[settings]
max_subagents = 10  # 最大并发子智能体数（1-20）
```

### 沙箱模式

```toml
[settings]
sandbox_mode = "workspace-write"  # read-only / workspace-write / danger-full-access
```

### 审批策略

```toml
[settings]
approval_policy = "on-request"  # never / on-request / untrusted
```

---

## 📝 完整配置示例

```toml
# 全局配置
provider = "deepseek"
default_text_model = "deepseek-v4-pro"
auth_mode = "api_key"
reasoning_effort = "auto"

# DeepSeek 提供商
[providers.deepseek]
api_key = "sk-your-deepseek-key"

# OpenAI 兼容（本地 LLM）
[providers.openai]
base_url = "http://localhost:1234/v1"
api_key = "not-needed"

# Ollama
[providers.ollama]
base_url = "http://localhost:11434/v1"

# 设置
[settings]
locale = "zh-Hans"
max_subagents = 10
sandbox_mode = "workspace-write"
approval_policy = "on-request"
```

---

## 🔍 诊断配置

### 检查配置

```bash
# 查看当前配置
deepseek doctor

# JSON 格式输出
deepseek doctor --json

# 查看认证状态
deepseek auth status

# 查看模型列表
deepseek models
```

### 常见问题

**Q: 配置不生效？**

检查优先级：
1. 命令行参数
2. 环境变量
3. 项目配置
4. 全局配置

**Q: 如何查看实际使用的配置？**

```bash
deepseek doctor
```

**Q: 配置文件语法错误？**

TOML 文件要求严格，检查：
- 引号是否配对
- 键值对用 `=`
- 节用 `[section]`

---

## 📚 相关文档

- [完整配置参考](../CONFIGURATION.md)
- [安装指南](../INSTALL.md)
- [模式说明](../MODES.md)
- [MCP 集成](../MCP.md)

---

最后更新：2026-05-16
