# 🔧 后台脚本 ESM/CommonJS 修复

## 🐛 **问题**

**错误信息**:
```
ReferenceError: require is not defined in ES module scope
This file is being treated as an ES module because ... package.json contains "type": "module"
```

**原因**:
- `autocode-background-run.js` 使用了 CommonJS 语法（`require()`）
- 但 `package.json` 声明了 `"type": "module"`
- Node.js 按 ES Module 解析，导致 `require` 未定义

**影响**:
- 后台脚本启动即崩溃
- 日志文件不生成
- 之前跑的 14 分钟其实是前台 MCP 调用，不是后台脚本

---

## ✅ **修复方案**

### **方案 A：改后缀为 .cjs（已采用 ✅）**

**操作**:
```bash
Move-Item autocode-background-run.js autocode-background-run.cjs
```

**原理**:
- `.cjs` 后缀告诉 Node.js 按 CommonJS 解析
- 不需要修改任何代码
- 最简单、最安全

**状态**: ✅ 已完成

---

### **方案 B：改为 ESM 语法（备选）**

需要修改所有 `require()` 为 `import`：

```javascript
// 旧代码（CommonJS）
const { exec, spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// 新代码（ESM）
import { exec, spawn } from 'child_process';
import path from 'path';
import fs from 'fs';
```

**缺点**:
- 需要改多处
- 可能引入新 bug
- 不如方案 A 简单

---

## 📝 **已更新的文件**

### **1. 文件重命名**
- ❌ `autocode-background-run.js`（已删除）
- ✅ `autocode-background-run.cjs`（新文件）

### **2. 文档更新**
- ✅ `WORKBUDDY-GUIDE.md` - 所有命令改为 `.cjs`
- ✅ `BACKGROUND-RUN-GUIDE.md` - 所有命令改为 `.cjs`

---

## 🚀 **使用方法（更新后）**

### **在 WorkBuddy 中发送**

```
使用 aicodclaw_shell 执行后台自动开发：

命令: start /B node f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\autocode-background-run.cjs "f:\ai\codes\github\deepseektuizh\前端任务分配计划.md" "f:\ai\codes\github\AILeaningApp"

工作目录: f:\ai\codes\github\AILeaningApp
```

**注意**: 现在是 `.cjs` 后缀！

---

## 🧪 **验证测试**

### **1. 测试后台脚本是否能启动**

```
使用 aicodclaw_shell 测试：

命令: node f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\autocode-background-run.cjs "f:\ai\codes\github\deepseektuizh\前端任务分配计划.md" "f:\ai\codes\github\AILeaningApp"

工作目录: f:\ai\codes\github\AILeaningApp
```

应该看到日志输出，而不是 `require is not defined` 错误。

### **2. 测试后台启动**

```
使用 aicodclaw_shell 启动后台：

命令: start /B node f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\autocode-background-run.cjs "f:\ai\codes\github\deepseektuizh\前端任务分配计划.md" "f:\ai\codes\github\AILeaningApp"

工作目录: f:\ai\codes\github\AILeaningApp
```

然后检查日志：

```
使用 aicodclaw_file_read 读取：

文件路径: f:\ai\codes\github\AILeaningApp\autocode-background.log
```

应该能看到日志内容。

---

## 📊 **修复前后对比**

| 项目 | 修复前 | 修复后 |
|------|--------|--------|
| 文件名 | `.js` | `.cjs` |
| 语法 | CommonJS (`require`) | CommonJS (`require`) |
| Node 解析 | ESM ❌ | CommonJS ✅ |
| 启动结果 | 崩溃 | 正常 |
| 日志生成 | 无 | 有 |

---

## ⚠️ **重要说明**

### **为什么主 MCP Server 用 ESM，后台脚本用 CommonJS？**

- **主 MCP Server** (`index.js`): 使用 ESM（`import`），因为 MCP SDK 要求
- **后台脚本** (`autocode-background-run.cjs`): 使用 CommonJS（`require`），因为：
  - 更简单
  - 不需要 MCP SDK
  - 只用到 Node.js 内置模块

### **是否可以统一？**

可以，但没必要：
- 后台脚本很简单，CommonJS 更直观
- `.cjs` 后缀明确标识了模块类型
- 混合使用是 Node.js 的标准做法

---

## ✅ **修复完成**

**状态**: ✅ 已完成  
**影响范围**: 仅后台启动脚本  
**风险**: 极低（只改后缀名）  
**测试**: 需要验证后台脚本能正常启动

---

**现在可以重新测试后台 autocode 了！** 🚀
