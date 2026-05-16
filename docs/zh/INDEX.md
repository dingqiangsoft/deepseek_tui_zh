# DeepSeek TUI 中文文档中心

欢迎来到 DeepSeek TUI 中文文档！🎉

---

## 📖 新手入门

如果您是第一次使用，建议按以下顺序阅读：

1. **[快速入门指南](QUICKSTART.md)** - 5分钟上手
2. **[配置指南](CONFIGURATION_GUIDE.md)** - 详细配置说明
3. **[文档索引](README.md)** - 完整文档列表

---

## 🚀 快速链接

### 安装与配置
- [快速入门](QUICKSTART.md) - 安装、启动、基本使用
- [配置指南](CONFIGURATION_GUIDE.md) - 配置文件、环境变量
- [安装说明](../INSTALL.md) - 平台特定安装（英文原版）

### 使用指南
- [模式说明](../MODES.md) - Plan/Agent/YOLO 模式
- [快捷键](../KEYBINDINGS.md) - 完整快捷键列表
- [工具接口](../TOOL_SURFACE.md) - 可用工具说明

### 高级功能
- [架构设计](../ARCHITECTURE.md) - 代码架构
- [MCP 集成](../MCP.md) - Model Context Protocol
- [子智能体](../SUBAGENTS.md) - 并发后台执行
- [用户记忆](../MEMORY.md) - 跨会话偏好

### 部署与运维
- [Docker 使用](../DOCKER.md) - 容器化部署
- [CNB 镜像](../CNB_MIRROR.md) - 中国友好安装
- [腾讯云部署](../TENCENT_CLOUD_REMOTE_FIRST.md) - 远程工作区
- [运维手册](../OPERATIONS_RUNBOOK.md) - 运维与恢复

---

## 💡 常见场景

### 场景 1：使用 DeepSeek 官方 API

```bash
# 1. 安装
npm install -g deepseek-tui

# 2. 设置 API Key
deepseek auth set --provider deepseek

# 3. 启动
deepseek --model auto
```

📖 详见：[快速入门 - 基本使用](QUICKSTART.md#-基本使用)

---

### 场景 2：使用本地 LLM

```bash
# 1. 配置本地服务
deepseek auth set --provider openai --api-key "not-needed"

# 2. 设置环境变量
$env:DEEPSEEK_BASE_URL = "http://localhost:1234/v1"
$env:DEEPSEEK_ALLOW_INSECURE_HTTP = "1"

# 3. 启动
deepseek --provider openai --model "your-model"
```

📖 详见：[配置指南 - 本地 LLM](CONFIGURATION_GUIDE.md#本地-llm-配置)

---

### 场景 3：切换工作项目

```powershell
# 方法 1: 使用专用启动脚本
.\start-aiclaw.ps1

# 方法 2: 手动切换
/exit
cd F:\your\project\path
deepseek --provider openai
```

📖 详见：[快速入门 - 常见问题](QUICKSTART.md#-常见问题)

---

### 场景 4：解除工具限制

```bash
# 启动时指定 YOLO 模式
deepseek --yolo

# 或在 TUI 中切换
/mode auto
```

📖 详见：[快速入门 - 三种模式](QUICKSTART.md#-三种模式)

---

## 🔧 本地 LLM 专区

### 支持的本地服务

| 服务 | 配置方式 | 文档 |
|------|---------|------|
| **Ollama** | `--provider ollama` | [配置指南](CONFIGURATION_GUIDE.md) |
| **SGLang** | `--provider sglang` | [配置指南](CONFIGURATION_GUIDE.md) |
| **vLLM** | `--provider vllm` | [配置指南](CONFIGURATION_GUIDE.md) |
| **LM Studio** | `--provider openai` | [配置指南](CONFIGURATION_GUIDE.md) |
| **其他 OpenAI 兼容** | `--provider openai` | [配置指南](CONFIGURATION_GUIDE.md) |

### 快速配置示例

**Ollama:**
```bash
ollama pull qwen2.5-coder:7b
deepseek --provider ollama --model qwen2.5-coder:7b
```

**LM Studio:**
```bash
$env:DEEPSEEK_BASE_URL = "http://localhost:1234/v1"
$env:DEEPSEEK_ALLOW_INSECURE_HTTP = "1"
deepseek --provider openai --model "your-model"
```

---

## 📋 常用命令速查

### 终端命令

```bash
deepseek                    # 启动 TUI
deepseek --model auto       # Auto 模式
deepseek --yolo             # YOLO 模式
deepseek doctor             # 诊断
deepseek models             # 列出模型
deepseek auth status        # 查看认证
deepseek update             # 更新
```

### TUI 命令

```
/model auto          # 切换模型
/provider openai     # 切换提供商
/theme dark          # 切换主题
/compact             # 压缩上下文
/reset               # 重置会话
/exit                # 退出
```

### 快捷键

```
Tab                # 自动补全
Shift+Tab          # 切换推理级别
F1                 # 帮助
Ctrl+K             # 命令面板
Ctrl+R             # 恢复会话
Ctrl+C             # 停止当前操作
```

📖 完整列表：[快捷键文档](../KEYBINDINGS.md)

---

## 🌐 环境变量速查表

| 变量 | 说明 | 示例 |
|------|------|------|
| `DEEPSEEK_API_KEY` | API 密钥 | `sk-xxx` |
| `DEEPSEEK_BASE_URL` | API 地址 | `http://localhost:1234/v1` |
| `DEEPSEEK_MODEL` | 默认模型 | `deepseek-v4-pro` |
| `DEEPSEEK_PROVIDER` | 提供商 | `openai` |
| `DEEPSEEK_ALLOW_INSECURE_HTTP` | 允许 HTTP | `1` |
| `DEEPSEEK_STREAM_IDLE_TIMEOUT_SECS` | 超时（秒） | `300` |

📖 完整列表：[配置指南 - 环境变量](CONFIGURATION_GUIDE.md#-环境变量)

---

## 🆘 获取帮助

### 文档
- 中文文档：本目录 (`docs/zh/`)
- 英文原文：[docs/](../docs/)

### 社区
- GitHub Issues: https://github.com/Hmbown/DeepSeek-TUI/issues
- GitHub Discussions: https://github.com/Hmbown/DeepSeek-TUI/discussions

### 本地帮助
```bash
deepseek --help          # 命令行帮助
F1 (在 TUI 中)           # 快捷键帮助
```

---

## 📝 文档贡献

欢迎贡献中文翻译！

### 如何贡献
1. Fork 本仓库
2. 在 `docs/zh/` 目录下创建翻译文件
3. 提交 Pull Request

### 翻译规范
- 保持原文结构和格式
- 技术术语保留英文或使用通用译名
- 代码示例保持原样
- 链接指向原文档

---

## 📊 文档状态

| 文档 | 状态 | 最后更新 |
|------|------|---------|
| 快速入门 | ✅ 完成 | 2026-05-16 |
| 配置指南 | ✅ 完成 | 2026-05-16 |
| 文档索引 | ✅ 完成 | 2026-05-16 |
| 架构设计 | 📝 待翻译 | - |
| 模式说明 | 📝 待翻译 | - |
| MCP 集成 | 📝 待翻译 | - |
| 其他文档 | 📝 待翻译 | - |

---

## 🎯 下一步

- **新用户**：阅读 [快速入门](QUICKSTART.md)
- **配置问题**：查看 [配置指南](CONFIGURATION_GUIDE.md)
- **高级功能**：浏览 [文档索引](README.md)

---

*本文档由社区贡献者翻译，如有不准确之处欢迎指正。*

*最后更新：2026-05-16*
