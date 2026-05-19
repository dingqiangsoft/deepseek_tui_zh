# 🚀 AICodeClaw MCP Server 完整技能指南

**版本**: v1.0  
**更新日期**: 2026-05-19  
**适用对象**: WorkBuddy AI 助手

---

## 📋 **目录**

1. [概述](#概述)
2. [工具列表](#工具列表)
3. [工具详细说明](#工具详细说明)
4. [使用场景](#使用场景)
5. [最佳实践](#最佳实践)
6. [常见问题](#常见问题)
7. [快速参考](#快速参考)

---

## 🎯 **概述**

AICodeClaw MCP Server 是一个为 WorkBuddy 提供的专业开发工具集，包含 9 个核心工具，用于：

- ✅ 文件读写操作
- ✅ 代码搜索和分析
- ✅ Shell 命令执行
- ✅ AI 智能查询
- ✅ 批量自动化操作
- ✅ 一键自动开发

**核心优势**:
- 🚀 减少 WorkBuddy token 消耗 90-95%
- 🎯 专为教育项目（课本/题库/语音/视频）优化
- 🔧 支持 Windows 环境
- 📊 完整的错误处理和验证

---

## 📦 **工具列表**

| 工具名称 | 功能 | 适用场景 | 状态 |
|---------|------|---------|------|
| `aicodclaw_file_read` | 读取文件内容 | 查看代码、配置、文档 | ✅ 可用 |
| `aicodclaw_file_write` | 写入文件内容 | 创建/修改文件 | ✅ 可用 |
| `aicodclaw_search` | 搜索代码 | 查找代码位置、模式匹配 | ✅ 可用 |
| `aicodclaw_apply_patch` | 应用代码补丁 | 精确修改代码 | ✅ 可用 |
| `aicodclaw_shell` | 执行 Shell 命令 | 运行脚本、构建、测试 | ✅ 可用 |
| `aicodclaw_deepseek` | AI 智能查询 | 代码分析、问题解答 | ✅ 可用 |
| `aicodclaw_deepseek-reply` | AI 对话回复 | 多轮对话、上下文保持 | ✅ 可用 |
| `aicodclaw_yolo` | 批量操作模式 | 多文件处理、自动化流程 | ✅ 可用 |
| `aicodclaw_autocode` | 一键自动开发 | 读取任务文档自动生成代码 | ✅ 已修复 |

---

## 🔧 **工具详细说明**

### **1. aicodclaw_file_read**

**功能**: 读取文件内容

**参数**:
```json
{
  "path": "文件路径（支持绝对路径和相对路径）"
}
```

**示例**:
```json
{
  "path": "f:\\ai\\codes\\github\\AILeaningApp\\src\\pages\\index\\index.vue"
}
```

**返回**:
```
📄 文件路径: f:\ai\codes\github\AILeaningApp\src\pages\index\index.vue

<template>
  ...
</template>
```

**限制**:
- 文件大小限制：10MB
- 支持所有文本文件

**注意事项**:
- 返回结果包含完整解析路径
- 文件不存在会返回错误信息

---

### **2. aicodclaw_file_write**

**功能**: 写入文件内容（自动创建目录）

**参数**:
```json
{
  "path": "文件路径",
  "content": "要写入的内容"
}
```

**示例**:
```json
{
  "path": "f:\\ai\\codes\\github\\AILeaningApp\\src\\components\\Header.vue",
  "content": "<template>\n  <view>Header</view>\n</template>"
}
```

**返回**:
```
✅ 文件已写入: f:\ai\codes\github\AILeaningApp\src\components\Header.vue
📊 大小: 1234 字符
```

**特性**:
- 自动创建不存在的目录
- 覆盖已有文件（谨慎使用）
- 返回文件大小信息

---

### **3. aicodclaw_search**

**功能**: 在文件中搜索模式

**参数**:
```json
{
  "pattern": "搜索模式（正则表达式）",
  "path": "搜索路径（目录）"
}
```

**示例**:
```json
{
  "pattern": "import.*from.*vue",
  "path": "f:\\ai\\codes\\github\\AILeaningApp\\src"
}
```

**返回**: 匹配结果列表

**支持**:
- 正则表达式
- 递归搜索子目录
- 显示行号和匹配内容

---

### **4. aicodclaw_apply_patch**

**功能**: 应用代码补丁（精确修改）

**参数**:
```json
{
  "filePath": "文件路径",
  "oldStr": "要替换的原始代码",
  "newStr": "新代码"
}
```

**示例**:
```json
{
  "filePath": "f:\\ai\\codes\\github\\AILeaningApp\\src\\main.ts",
  "oldStr": "const app = createApp(App)",
  "newStr": "const app = createApp(App).use(router)"
}
```

**返回**: 补丁应用结果

**注意事项**:
- oldStr 必须在文件中唯一存在
- 建议先用 search 确认代码位置
- 支持多次替换

---

### **5. aicodclaw_shell**

**功能**: 执行 Shell 命令（Windows 已优化）

**参数**:
```json
{
  "command": "要执行的命令",
  "cwd": "工作目录（必填）"
}
```

**示例**:
```json
{
  "command": "npm run build",
  "cwd": "f:\\ai\\codes\\github\\AILeaningApp"
}
```

**返回**:
```
📁 工作目录: f:\ai\codes\github\AILeaningApp
🖥️  平台: win32

> aileaning-app@1.0.0 build
> vite build

✓ built in 12.34s
```

**Windows 命令自动适配**:
- `mkdir -p` → `New-Item -ItemType Directory -Force`
- `rm -rf` → `Remove-Item -Recurse -Force`
- `cp -r` → `Copy-Item -Recurse`
- `&&` → `;`
- `ls` → `dir`
- `cat` → `type`

**安全特性**:
- cwd 必填且验证
- 工作目录必须在路径中显示
- 命令执行超时 60 秒

---

### **6. aicodclaw_deepseek**

**功能**: AI 智能查询（代码分析、问题解答）

**参数**:
```json
{
  "query": "查询内容"
}
```

**示例**:
```json
{
  "query": "如何在 UniApp 中实现下拉刷新？"
}
```

**返回**: AI 回答

**AI 服务**:
- 地址: `http://192.168.2.5:1234/v1/chat/completions`
- 模型: `qwen3.5-9b-deepseek-v4-flash@q6_k`
- 超时: 30 秒

**适用场景**:
- 代码分析
- Bug 诊断
- 最佳实践咨询
- 技术方案评估

---

### **7. aicodclaw_deepseek-reply**

**功能**: AI 多轮对话（保持上下文）

**参数**:
```json
{
  "message": "回复内容",
  "conversationId": "对话 ID（可选）"
}
```

**示例**:
```json
{
  "message": "能给我完整的代码示例吗？",
  "conversationId": "conv-123"
}
```

**返回**: AI 回复 + 对话 ID

**特性**:
- 支持多轮对话
- 保持上下文
- 对话 ID 用于追踪

---

### **8. aicodclaw_yolo**

**功能**: 批量操作模式（自动批准所有操作）

**参数**:
```json
{
  "operations": [
    {
      "type": "file_read|file_write|shell|search",
      "params": { ... }
    }
  ],
  "cwd": "工作目录（必填）"
}
```

**示例**:
```json
{
  "operations": [
    {
      "type": "file_write",
      "params": {
        "path": "src/test1.vue",
        "content": "<template><view>Test 1</view></template>"
      }
    },
    {
      "type": "file_write",
      "params": {
        "path": "src/test2.vue",
        "content": "<template><view>Test 2</view></template>"
      }
    }
  ],
  "cwd": "f:\\ai\\codes\\github\\AILeaningApp"
}
```

**返回**:
```
🚀 YOLO 模式执行完成
📁 工作目录: f:\ai\codes\github\AILeaningApp
🖥️  平台: win32

执行 2 个操作:
  [0] ✅ 已写入: f:\ai\codes\github\AILeaningApp\src\test1.vue
  [1] ✅ 已写入: f:\ai\codes\github\AILeaningApp\src\test2.vue
```

**操作类型**:
- `file_read` - 读取文件
- `file_write` - 写入文件
- `shell` - 执行命令
- `search` - 搜索代码

**特性**:
- 自动批准，无需确认
- 路径基于 cwd 解析（支持相对路径）
- 批量执行，提高效率
- Windows 命令自动适配

**注意事项**:
- 所有操作自动执行，谨慎使用
- 失败的操作会标记为 ❌
- 适合已验证的批量操作

---

### **9. aicodclaw_autocode** ⭐

**功能**: 一键自动开发（读取任务文档，自动生成代码）

**参数**:
```json
{
  "taskDocPath": "任务文档路径（.md 或 .xlsx）",
  "workDir": "工作目录（必填）"
}
```

**示例**:
```json
{
  "taskDocPath": "f:\\ai\\codes\\github\\deepseektuizh\\前端任务分配计划.md",
  "workDir": "f:\\ai\\codes\\github\\AILeaningApp"
}
```

**工作流程**:
1. ✅ 验证工作目录
2. ✅ 读取任务文档（.md/.xlsx）
3. ✅ 分析项目结构
4. ✅ AI 生成开发计划
5. ✅ 循环执行任务：
   - 生成代码
   - 写入文件
   - 验证文件（三重验证）
   - 失败重试（最多 10 次）
6. ✅ 生成开发报告

**返回**: 完整开发报告

**报告内容**:
- 项目名称
- 技术栈
- 总文件数
- 成功/失败统计
- 错误详情
- 完整日志

**安全特性**（已修复）:
- ✅ 严格路径验证（防止路径穿越）
- ✅ 强制相对路径（拒绝盘符）
- ✅ 三重文件验证（存在 + 大小 + 内容）
- ✅ Windows 大小写处理
- ✅ 真实报告（防止假报告）

**错误处理**:
- 自动重试最多 10 次
- 超过 10 次记录错误，继续下一个任务
- 最终报告包含所有错误信息

**已知限制**:
- ⚠️ 受 MCP 60 秒超时限制
- ⚠️ 大型项目建议使用后台脚本
- ⚠️ Excel 文件需要额外依赖

---

## 🎯 **使用场景**

### **场景 1: 查看和修改代码**

```
1. 使用 file_read 查看文件
2. 使用 search 查找代码位置
3. 使用 apply_patch 精确修改
4. 使用 file_write 创建新文件
```

### **场景 2: 批量文件操作**

```
使用 yolo 模式：
- 批量创建组件
- 批量修改配置
- 批量执行命令
```

### **场景 3: AI 辅助开发**

```
1. 使用 deepseek 咨询技术方案
2. 使用 deepseek-reply 深入讨论
3. 使用 shell 执行测试
4. 使用 file_write 应用方案
```

### **场景 4: 一键自动开发** ⭐

```
使用 autocode：
1. 准备任务文档（.md）
2. 调用 autocode 工具
3. 等待开发完成
4. 查看开发报告
5. 验证生成的文件
```

**任务文档格式**:
```markdown
# 任务名称

## 任务 1
- 文件路径: src/pages/xxx.vue
- 功能描述: ...
- 技术要求: ...

## 任务 2
- 文件路径: src/components/xxx.vue
- 功能描述: ...
- 技术要求: ...
```

---

## 💡 **最佳实践**

### **1. 文件路径**

✅ **推荐**:
```
f:\ai\codes\github\AILeaningApp\src\pages\index\index.vue
```

❌ **避免**:
```
src\pages\index\index.vue  （相对路径可能不准确）
```

### **2. Shell 命令**

✅ **推荐**:
```json
{
  "command": "npm run build",
  "cwd": "f:\\ai\\codes\\github\\AILeaningApp"
}
```

**Windows 命令已自动适配，可以直接使用 Linux 语法**:
```json
{
  "command": "mkdir -p src/components",
  "cwd": "f:\\ai\\codes\\github\\AILeaningApp"
}
```

### **3. 批量操作**

✅ **使用 yolo 模式**:
```json
{
  "operations": [
    { "type": "file_write", "params": { "path": "file1.txt", "content": "..." } },
    { "type": "file_write", "params": { "path": "file2.txt", "content": "..." } }
  ],
  "cwd": "f:\\ai\\codes\\github\\AILeaningApp"
}
```

### **4. 自动开发**

✅ **小项目（< 10 个文件）**: 直接调用 autocode

✅ **大项目（> 10 个文件）**: 使用后台脚本
```
start /B node f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\autocode-background-run.cjs "任务.md" "工作目录"
```

### **5. 错误处理**

✅ **检查返回结果**:
- 查看 `isError` 字段
- 检查错误信息
- 根据错误调整策略

✅ **验证文件**:
- 使用 file_read 确认文件内容
- 使用 shell 的 Test-Path 验证文件存在
- 检查文件大小是否合理

---

## ❓ **常见问题**

### **Q1: MCP 工具不可用怎么办？**

**原因**:
- MCP Server 未启动
- 配置错误
- 网络问题

**解决**:
1. 检查 Qoder 的 MCP 配置
2. 重启 Qoder
3. 测试简单命令：`echo test`

### **Q2: 文件路径错误？**

**症状**: 文件写入到错误位置

**解决**:
- 使用绝对路径
- 验证路径格式（使用双反斜杠 `\\`）
- 检查工作目录是否正确

### **Q3: autocode 超时？**

**原因**: MCP 协议 60 秒超时限制

**解决**: 使用后台脚本
```
start /B node f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\autocode-background-run.cjs "任务.md" "工作目录"
```

### **Q4: shell 命令失败？**

**可能原因**:
- 命令语法错误
- 路径不存在
- 权限问题

**解决**:
- 检查命令格式
- 验证工作目录
- 查看详细错误信息

### **Q5: 如何查看后台任务进度？**

**方法**:
```
读取文件: f:\ai\codes\github\AILeaningApp\autocode-background.log
```

### **Q6: autocode 生成的文件在哪？**

**位置**: 工作目录下

**验证**:
```
工作目录 + 文件路径 = 完整路径
例如: f:\ai\codes\github\AILeaningApp + src/pages/test.vue
```

### **Q7: 如何停止后台任务？**

```powershell
Get-Process node | Where-Object { $_.CommandLine -like "*autocode*" } | Stop-Process
```

---

## 📋 **快速参考**

### **工具速查表**

| 工具 | 用途 | 必填参数 |
|------|------|---------|
| file_read | 读文件 | path |
| file_write | 写文件 | path, content |
| search | 搜索代码 | pattern, path |
| apply_patch | 应用补丁 | filePath, oldStr, newStr |
| shell | 执行命令 | command, cwd |
| deepseek | AI 查询 | query |
| deepseek-reply | AI 对话 | message |
| yolo | 批量操作 | operations, cwd |
| autocode | 一键开发 | taskDocPath, workDir |

### **常用命令**

**查看文件**:
```json
{ "tool": "aicodclaw_file_read", "args": { "path": "文件路径" } }
```

**创建文件**:
```json
{ "tool": "aicodclaw_file_write", "args": { "path": "文件路径", "content": "内容" } }
```

**执行命令**:
```json
{ "tool": "aicodclaw_shell", "args": { "command": "命令", "cwd": "工作目录" } }
```

**批量操作**:
```json
{ "tool": "aicodclaw_yolo", "args": { "operations": [...], "cwd": "工作目录" } }
```

**一键开发**:
```json
{ "tool": "aicodclaw_autocode", "args": { "taskDocPath": "任务.md", "workDir": "工作目录" } }
```

### **错误代码**

| 错误 | 原因 | 解决 |
|------|------|------|
| File not found | 文件不存在 | 检查路径 |
| File too large | 文件 > 10MB | 分段读取 |
| Command failed | 命令执行失败 | 查看错误信息 |
| AI API error | AI 服务不可用 | 检查网络连接 |
| 路径安全错误 | 路径越界 | 使用相对路径 |
| 文件内容验证失败 | 写入异常 | 重试或检查权限 |

---

## 📞 **技术支持**

**文档位置**:
- `WORKBUDDY-GUIDE.md` - WorkBuddy 使用指南
- `BACKGROUND-RUN-GUIDE.md` - 后台运行指南
- `TEST-REPORT.md` - 测试报告
- `AUTOCODE-DEEP-FIX.md` - autocode 修复报告

**测试脚本**:
```bash
cd f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server
node test-all-tools.cjs
```

**后台脚本**:
```
autocode-background-run.cjs
```

---

## 🎯 **总结**

### **核心优势**

1. ✅ **9 个专业工具** - 覆盖开发全流程
2. ✅ **Windows 优化** - 命令自动适配
3. ✅ **安全防护** - 路径验证、内容验证
4. ✅ **自动化** - yolo 批量操作、autocode 一键开发
5. ✅ **AI 集成** - 智能查询、代码分析
6. ✅ **错误处理** - 重试机制、详细日志

### **推荐使用顺序**

**新手**:
1. file_read → 查看文件
2. file_write → 创建文件
3. shell → 执行简单命令
4. deepseek → 咨询问题

**进阶**:
5. search → 搜索代码
6. apply_patch → 精确修改
7. yolo → 批量操作
8. deepseek-reply → 深入讨论

**专家**:
9. autocode → 一键自动开发

---

**WorkBuddy + AICodeClaw = 高效开发！** 🚀
