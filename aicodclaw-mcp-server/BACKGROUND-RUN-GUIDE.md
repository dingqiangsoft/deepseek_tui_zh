# 🚀 AICodeClaw 后台自动运行指南

## 解决 MCP 60 秒超时问题

### 问题根源
MCP 协议有最大超时限制（60 秒），长任务一定会被掐断，返回 40504 错误。

### 终极解决方案
**后台异步运行 + 不等待返回 = 永远不超时**

---

## 📋 使用方法（2 步）

### 步骤 1：确保文件存在

确认以下文件存在：
- `autocode-background-run.cjs` - 后台运行脚本（已创建）
- `前端任务分配计划.md` - 你的任务文档
- `index.js` - AICodeClaw MCP Server

### 步骤 2：在 WorkBuddy/Qoder 中执行

使用 `aicodclaw_shell` 工具执行：

```json
{
  "command": "start /B node f:\\ai\\codes\\github\\deepseektuizh\\aicodclaw-mcp-server\\autocode-background-run.cjs \"f:\\ai\\codes\\github\\deepseektuizh\\前端任务分配计划.md\" \"f:\\ai\\codes\\github\\AILeaningApp\"",
  "cwd": "f:\\ai\\codes\\github\\AILeaningApp"
}
```

---

## 🎯 工作原理

```
┌─────────────────────────────────────────────┐
│  WorkBuddy / Qoder                          │
│  (MCP Client)                               │
│                                             │
│  1. 调用 aicodclaw_shell                    │
│  2. 执行 start /B node autocode-...         │
│  3. 立即返回（不等待）                      │
│  4. MCP 连接关闭                            │
└──────────────┬──────────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────────┐
│  Windows 后台独立进程                        │
│  (不受 MCP 超时限制)                         │
│                                             │
│  ✅ 读取任务文档                             │
│  ✅ 启动 MCP Server                          │
│  ✅ 调用 aicodclaw_autocode                  │
│  ✅ 自动开发（5-10 分钟）                    │
│  ✅ 生成报告                                 │
│  ✅ 自动退出                                 │
└─────────────────────────────────────────────┘
```

---

## 📊 输出文件

### 1. 日志文件
**位置**: `f:\ai\codes\github\AILeaningApp\autocode-background.log`

**内容**: 实时运行日志
```
[2024-01-01T12:00:00.000Z] 🚀 后台 Autocode 已启动
[2024-01-01T12:00:01.000Z] 📁 工作目录: f:\ai\codes\github\AILeaningApp
[2024-01-01T12:00:02.000Z] ✅ 文件验证通过
[2024-01-01T12:00:03.000Z] 📖 任务文档读取完成 (5000 字符)
...
```

### 2. 报告文件
**位置**: `f:\ai\codes\github\AILeaningApp\autocode-background-report.md`

**内容**: 完整的开发报告（包含统计、错误、日志）

---

## 🔍 监控进度

### 方法 1：查看日志文件
```powershell
Get-Content f:\ai\codes\github\AILeaningApp\autocode-background.log -Wait
```

### 方法 2：查看进程
```powershell
Get-Process node | Where-Object { $_.CommandLine -like "*autocode*" }
```

### 方法 3：等待报告生成
```powershell
while (-not (Test-Path "f:\ai\codes\github\AILeaningApp\autocode-background-report.md")) {
    Start-Sleep -Seconds 5
    Write-Host "等待中..."
}
Write-Host "✅ 完成！"
```

---

## 💡 优势

| 特性 | 直接调用 | 后台运行 |
|------|---------|---------|
| 超时限制 | ❌ 60 秒 | ✅ 无限制 |
| 中断风险 | ❌ 高 | ✅ 无 |
| 长时间任务 | ❌ 失败 | ✅ 成功 |
| 实时监控 | ✅ 可以 | ⚠️ 查看日志 |
| 资源占用 | ✅ 低 | ⚠️ 独立进程 |

---

## ⚙️ 自定义参数

### 修改任务文件
```powershell
node autocode-background-run.js "你的任务.md" "工作目录"
```

### 修改 MCP Server 路径
编辑 `autocode-background-run.js` 第 51 行：
```javascript
const mcpServer = path.join(__dirname, 'index.js');
// 改为你的路径
const mcpServer = 'f:\\your\\path\\index.js';
```

---

## 🛑 停止后台任务

### 方法 1：通过进程 ID
```powershell
# 查看日志获取 PID
Get-Content autocode-background.log | Select-String "PID"

# 终止进程
Stop-Process -Id <PID>
```

### 方法 2：终止所有相关进程
```powershell
Get-Process node | Where-Object { $_.CommandLine -like "*autocode*" } | Stop-Process
```

---

## 🎯 完整示例

### 在 WorkBuddy 中发送：

```
使用 aicodclaw_shell 工具执行以下命令：

命令: start /B node f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\autocode-background-run.js "f:\ai\codes\github\deepseektuizh\前端任务分配计划.md" "f:\ai\codes\github\AILeaningApp"

工作目录: f:\ai\codes\github\AILeaningApp
```

### 然后：

1. ✅ WorkBuddy 立即返回成功
2. ✅ 后台进程开始运行
3. ✅ 自动开发整个项目（不受超时限制）
4. ✅ 完成后生成 `autocode-background-report.md`
5. ✅ 你可以随时查看日志监控进度

---

## 📝 注意事项

1. **不要关闭终端** - 后台进程需要持续运行
2. **检查磁盘空间** - 确保有足够空间生成文件
3. **监控日志** - 定期检查 `autocode-background.log`
4. **等待报告** - 完成后会生成完整报告

---

**这就是终极解决方案：永不超时，永不中断！** 🚀
