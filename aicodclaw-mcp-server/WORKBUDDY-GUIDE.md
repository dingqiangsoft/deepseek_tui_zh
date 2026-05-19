# 🚀 WorkBuddy 使用 AICodeClaw 自动开发指南

## 📋 目录

1. [快速开始](#快速开始)
2. [核心功能](#核心功能)
3. [使用方法](#使用方法)
4. [后台自动开发（解决超时）](#后台自动开发解决超时)
5. [监控进度](#监控进度)
6. [常见问题](#常见问题)

---

## 🎯 快速开始

### 前提条件

✅ MCP Server 已配置并运行  
✅ 工作目录存在项目文件  
✅ 任务文档已准备好（.md 格式）

### 第一次使用

在 WorkBuddy 中发送：

```
使用 aicodclaw_shell 测试连接：

命令: echo "MCP Server 正常"
工作目录: f:\ai\codes\github\AILeaningApp
```

如果返回成功，说明 MCP Server 正常工作。

---

## 🔧 核心功能

### 工具列表

| 工具名称 | 功能 | 适用场景 |
|---------|------|---------|
| `aicodclaw_file_read` | 读取文件 | 查看代码、配置 |
| `aicodclaw_file_write` | 写入文件 | 创建/修改文件 |
| `aicodclaw_search` | 代码搜索 | 查找代码位置 |
| `aicodclaw_shell` | 执行命令 | 运行脚本、构建 |
| `aicodclaw_deepseek` | AI 查询 | 智能问答 |
| `aicodclaw_yolo` | 批量操作 | 多文件处理 |
| `aicodclaw_autocode` | 一键生成 | 自动开发项目 |

---

## 📖 使用方法

### 方法 1：使用 aicodclaw_autocode（推荐新手）

**适用场景**：完整的自动开发任务

**在 WorkBuddy 中发送**：

```
使用 aicodclaw_autocode 工具：

任务文档: f:\ai\codes\github\deepseektuizh\前端任务分配计划.md
工作目录: f:\ai\codes\github\AILeaningApp
```

**特点**：
- ✅ 自动读取任务文档
- ✅ 自动分析项目结构
- ✅ 自动调用 AI 生成代码
- ✅ 自动写入文件
- ✅ 生成开发报告

**限制**：
- ⚠️ 受 MCP 60 秒超时限制
- ⚠️ 大型项目可能中断

---

### 方法 2：使用后台自动开发（推荐生产环境）

**适用场景**：大型项目、长时间任务

**在 WorkBuddy 中发送**：

```
使用 aicodclaw_shell 工具执行后台自动开发：

命令: start /B node f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\autocode-background-run.cjs "f:\ai\codes\github\deepseektuizh\前端任务分配计划.md" "f:\ai\codes\github\AILeaningApp"

工作目录: f:\ai\codes\github\AILeaningApp
```

**特点**：
- ✅ 永不超时
- ✅ 永不中断
- ✅ 后台独立运行
- ✅ 自动生成报告
- ✅ 适合 5-10 分钟的长任务

**工作流程**：
```
1. WorkBuddy 调用 aicodclaw_shell
2. 执行 start /B 命令（后台启动）
3. 立即返回成功（1-2 秒）
4. 后台进程独立运行
5. 自动完成开发任务
6. 生成报告文件
```

---

## 🔥 后台自动开发（解决超时）

### 为什么需要后台运行？

MCP 协议有 **60 秒超时限制**，超过这个时间的任务会被强制中断，返回 `40504` 错误。

**解决方案**：使用 `start /B` 在 Windows 后台启动独立进程，不受 MCP 超时限制。

### 使用步骤

#### 步骤 1：发送命令

在 WorkBuddy 中发送：

```
使用 aicodclaw_shell 执行后台自动开发任务。

命令参数：
- 命令: start /B node f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\autocode-background-run.cjs "f:\ai\codes\github\deepseektuizh\前端任务分配计划.md" "f:\ai\codes\github\AILeaningApp"
- 工作目录: f:\ai\codes\github\AILeaningApp
```

#### 步骤 2：确认启动

WorkBuddy 会**立即返回成功**（通常 1-2 秒），这是正常的，因为 `start /B` 是后台启动。

#### 步骤 3：查看进度

在 WorkBuddy 中请求查看日志：

```
使用 aicodclaw_file_read 读取日志文件：

文件路径: f:\ai\codes\github\AILeaningApp\autocode-background.log
```

**日志内容示例**：
```
[2024-01-01T12:00:00.000Z] 🚀 后台 Autocode 已启动
[2024-01-01T12:00:01.000Z] 📁 工作目录: f:\ai\codes\github\AILeaningApp
[2024-01-01T12:00:02.000Z] ✅ 文件验证通过
[2024-01-01T12:00:03.000Z] 📖 任务文档读取完成 (5000 字符)
[2024-01-01T12:00:10.000Z] 🤖 AI 生成开发计划...
[2024-01-01T12:00:15.000Z] ✅ 计划生成完成：共 15 个文件
[2024-01-01T12:00:15.000Z] 🚀 开始 CI 批量开发...
[2024-01-01T12:00:20.000Z] [1/15] 生成 → src/pages/index/index.vue
[2024-01-01T12:00:35.000Z] ✅ 成功: src/pages/index/index.vue (12.5KB)
...
```

#### 步骤 4：等待完成

当看到日志中出现：
```
[2024-01-01T12:10:00.000Z] ✅ 后台任务完成，进程已清理
```

说明任务已完成。

#### 步骤 5：查看报告

```
使用 aicodclaw_file_read 读取开发报告：

文件路径: f:\ai\codes\github\AILeaningApp\autocode-background-report.md
```

**报告内容**：
- 项目名称
- 技术栈
- 总文件数
- 成功/失败统计
- 错误详情
- 完整日志

---

## 📊 监控进度

### 方法 1：实时查看日志

在 WorkBuddy 中定期请求：

```
读取 autocode-background.log 的最后 20 行
```

### 方法 2：检查报告文件是否存在

```
检查文件是否存在: f:\ai\codes\github\AILeaningApp\autocode-background-report.md
```

如果文件存在，说明已完成。

### 方法 3：查看生成的文件

```
使用 aicodclaw_shell 查看项目文件：

命令: dir /s /b *.vue
工作目录: f:\ai\codes\github\AILeaningApp
```

---

## 💡 实用示例

### 示例 1：读取任务文档

```
使用 aicodclaw_file_read 读取任务：

文件路径: f:\ai\codes\github\deepseektuizh\前端任务分配计划.md
```

### 示例 2：查看项目结构

```
使用 aicodclaw_shell 查看目录结构：

命令: tree /F /A
工作目录: f:\ai\codes\github\AILeaningApp
```

### 示例 3：AI 咨询问题

```
使用 aicodclaw_deepseek 咨询：

问题: 如何在 UniApp 中实现下拉刷新？
```

### 示例 4：批量文件操作

```
使用 aicodclaw_yolo 批量创建文件：

操作列表:
- 类型: file_write
  参数:
    path: src/components/Header.vue
    content: <template><view>Header</view></template>
- 类型: file_write
  参数:
    path: src/components/Footer.vue
    content: <template><view>Footer</view></template>

工作目录: f:\ai\codes\github\AILeaningApp
```

### 示例 5：执行构建命令

```
使用 aicodclaw_shell 构建项目：

命令: npm run build
工作目录: f:\ai\codes\github\AILeaningApp
```

---

## 🎯 完整工作流示例

### 场景：开发教育类小程序前端

#### 步骤 1：准备任务文档

创建 `前端任务分配计划.md`，包含：
- 项目概述
- 技术栈要求
- 页面列表
- 组件列表
- 接口规范

#### 步骤 2：启动后台自动开发

```
使用 aicodclaw_shell 启动后台自动开发：

命令: start /B node f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\autocode-background-run.cjs "f:\ai\codes\github\deepseektuizh\前端任务分配计划.md" "f:\ai\codes\github\AILeaningApp"

工作目录: f:\ai\codes\github\AILeaningApp
```

#### 步骤 3：监控进度（可选）

每隔 1-2 分钟查看日志：

```
读取 autocode-background.log 最后 10 行
```

#### 步骤 4：查看完成报告

```
读取 autocode-background-report.md
```

#### 步骤 5：验证生成的代码

```
使用 aicodclaw_file_read 查看关键文件：
- src/pages/index/index.vue
- src/pages/list/list.vue
- src/components/Player.vue
```

---

## ❓ 常见问题

### Q1: MCP 工具不可用怎么办？

**原因**：MCP Server 未启动或配置错误

**解决**：
1. 检查 Qoder 的 MCP 配置
2. 重启 Qoder
3. 测试简单命令：`echo test`

### Q2: 返回 40504 超时错误？

**原因**：任务超过 60 秒

**解决**：使用后台运行方式（`start /B`）

### Q3: 如何停止后台任务？

**解决**：在 WorkBuddy 中发送：

```
使用 aicodclaw_shell 停止后台任务：

命令: taskkill /F /IM node.exe /FI "WINDOWTITLE eq autocode*"
工作目录: f:\ai\codes\github\AILeaningApp
```

### Q4: 任务失败如何查看错误？

**解决**：

```
读取错误日志：
文件路径: f:\ai\codes\github\AILeaningApp\autocode-background.log
```

### Q5: 可以自定义任务文件和目录吗？

**可以**！修改命令中的路径参数：

```
start /B node autocode-background-run.js "你的任务.md" "你的工作目录"
```

### Q6: 后台运行会影响性能吗？

**不会**！后台进程是独立的，不影响 WorkBuddy 的正常运行。

### Q7: 如何确认后台进程正在运行？

**解决**：

```
使用 aicodclaw_shell 查看进程：

命令: tasklist | findstr node.exe
工作目录: f:\ai\codes\github\AILeaningApp
```

### Q8: 报告文件在哪里？

**位置**：工作目录下的 `autocode-background-report.md`

例如：`f:\ai\codes\github\AILeaningApp\autocode-background-report.md`

---

## 📝 注意事项

1. **路径格式**：使用双反斜杠 `\\` 或正斜杠 `/`
2. **文件编码**：任务文档使用 UTF-8 编码
3. **磁盘空间**：确保有足够空间生成文件
4. **网络连接**：AI 服务需要网络连接
5. **权限问题**：确保有写入权限

---

## 🎯 快速参考卡片

### 后台自动开发（推荐）

```
工具: aicodclaw_shell
命令: start /B node f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\autocode-background-run.js "任务文档.md" "工作目录"
特点: 永不超时，适合大型项目
```

### 直接调用（小型任务）

```
工具: aicodclaw_autocode
参数: taskDocPath="任务文档.md", workDir="工作目录"
特点: 简单直接，但受 60 秒限制
```

### 查看进度

```
工具: aicodclaw_file_read
文件: 工作目录/autocode-background.log
```

### 查看报告

```
工具: aicodclaw_file_read
文件: 工作目录/autocode-background-report.md
```

---

## 🚀 总结

| 场景 | 推荐方式 | 原因 |
|------|---------|------|
| 小型项目（< 5 个文件） | aicodclaw_autocode | 简单直接 |
| 中型项目（5-15 个文件） | 后台运行 | 避免超时 |
| 大型项目（> 15 个文件） | 后台运行 | 永不中断 |
| 测试/调试 | aicodclaw_shell | 灵活控制 |
| AI 咨询 | aicodclaw_deepseek | 智能问答 |

---

**有问题？查看完整文档：** `f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\BACKGROUND-RUN-GUIDE.md`

---

**WorkBuddy + AICodeClaw = 全自动开发！** 🚀
