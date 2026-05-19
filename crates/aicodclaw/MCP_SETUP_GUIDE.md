# WorkBuddy MCP 配置指南

## 🎯 重要说明

**MCP 配置是在 DeepSeek TUI 内部通过 `/mcp` 命令管理的**，不是直接修改 WorkBuddy 的配置文件。

---

## 📋 配置步骤

### 1. 启动 DeepSeek TUI

```powershell
.\target\release\deepseek
```

### 2. 在 TUI 内添加 MCP Server

在输入框中输入以下命令：

```
/mcp add stdio aicodclaw F:\ai\codes\github\deepseektuizh\target\release\deepseek.exe serve --mcp
```

**命令解释：**
- `/mcp add stdio` - 添加一个 stdio 类型的 MCP 服务器
- `aicodclaw` - 服务器名称（自定义）
- `F:\ai\codes\github\deepseektuizh\target\release\deepseek.exe` - 可执行文件路径
- `serve --mcp` - 启动参数

### 3. 验证配置

```
/mcp validate
```

### 4. 查看 MCP 服务器状态

```
/mcp status
```

或

```
/mcp list
```

### 5. 重新加载 MCP 配置（如果需要）

```
/mcp reload
```

---

## 🔧 常用 MCP 命令

| 命令 | 说明 |
|------|------|
| `/mcp` 或 `/mcp status` | 显示所有 MCP 服务器状态 |
| `/mcp add stdio <name> <command> [args...]` | 添加 stdio 服务器 |
| `/mcp add http <name> <url>` | 添加 HTTP 服务器 |
| `/mcp enable <name>` | 启用服务器 |
| `/mcp disable <name>` | 禁用服务器 |
| `/mcp remove <name>` | 删除服务器 |
| `/mcp validate` | 验证所有 MCP 连接 |
| `/mcp reload` | 重新加载配置 |

---

## ⚠️ 之前的错误

之前我创建的 `mcp-config-corrected.json` 和 `quick-setup-mcp.ps1` 是**错误的配置方式**，应该删除：

```powershell
# 删除错误的配置文件
Remove-Item crates\aicodclaw\mcp-config-complete.json
Remove-Item crates\aicodclaw\quick-setup-mcp.ps1

# 如果已经覆盖了 WorkBuddy 配置，恢复备份
Copy-Item C:\Users\Administrator\.workbuddy\.mcp.json.backup C:\Users\Administrator\.workbuddy\.mcp.json -Force
```

---

## ✅ 正确的使用流程

1. **启动 TUI** → `.\target\release\deepseek`
2. **在 TUI 内配置 MCP** → `/mcp add stdio aicodclaw <path> serve --mcp`
3. **验证连接** → `/mcp validate`
4. **在 WorkBuddy 中使用** → WorkBuddy 会自动发现可用的 MCP 工具

---

## 📝 注意事项

1. **MCP 配置存储位置**：TUI 会将 MCP 配置保存在 `~/.deepseek/mcp.json`
2. **WorkBuddy 的 MCP 配置**：WorkBuddy 有自己的 `.mcp.json`，两者是不同的
3. **工具调用**：配置后，AI 会自动发现并调用 MCP 工具

---

## 🎯 参赛演示建议

在演示视频中展示：

1. 打开 TUI
2. 输入 `/mcp add stdio aicodclaw ...`
3. 输入 `/mcp validate` 显示连接成功
4. 使用 MCP 工具执行任务（如代码审查）
5. 强调："所有操作都在本地完成，数据零泄露"

---

**总结：MCP 配置在 TUI 内部完成，不是通过外部配置文件！** 🦞
