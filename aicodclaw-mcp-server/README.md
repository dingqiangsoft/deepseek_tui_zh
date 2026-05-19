# AICodeClaw MCP Server

为 WorkBuddy 提供**可定制**的本地开发工具，减少 90-95% Token 消耗。

## 🌟 核心优势

### 完全可定制的开发工具

AICodeClaw 最大的优势是**每个工具都可以根据你的项目定制**：

- ✅ **自定义文件操作**：适配你的项目结构
- ✅ **专属代码搜索**：支持你的代码库规范
- ✅ **个性化 Shell 命令**：预配置你的工作流
- ✅ **定制 AI 分析**：针对你的技术栈优化
- ✅ **私有化部署**：100% 本地，数据零泄露

**这就是我们与通用 MCP 工具的区别！**

## 🚀 快速开始

### 1. 安装依赖

```bash
npm install
```

### 2. 配置 WorkBuddy

在 WorkBuddy 的 `mcp.json` 中添加：

```json
{
  "mcpServers": {
    "aicodclaw": {
      "command": "node",
      "args": ["F:\\ai\\codes\\github\\deepseektuizh\\aicodclaw-mcp-server\\index.js"],
      "env": {
        "DEEPSEEK_MODEL": "qwen3.5-9b-deepseek-v4-flash@q6_k"
      },
      "disabled": false
    }
  }
}
```

### 3. WorkBuddy 模型配置（本地化优化）

针对本地 9B 模型 + 开发工具场景：

| 配置项 | 推荐值 | 说明 |
|--------|--------|------|
| **工具调用** | ✅ 勾选 | 必须开启，否则无法调用 MCP 工具 |
| **图片输入** | ❌ 不勾选 | 纯代码处理不需要图片识别 |
| **推理模式** | ❌ 不勾选 | 标准模式更快更省 Token |
| **输入上下文** | 32K 或 64K | 9B 模型不建议超过 64K |
| **输出长度** | 8K 或 16K | 代码生成 8K 足够 |
| **自定义协议** | ❌ 不勾选 | 使用默认值 |

### 4. 重启 WorkBuddy

重启后即可使用 7 个 MCP 工具。

---

## 🛠️ 工具清单

| 工具 | 功能 | 示例 |
|------|------|------|
| `aicodclaw_file_read` | 读取文件 | 读取代码、配置 |
| `aicodclaw_file_write` | 写入文件 | 创建、修改文件 |
| `aicodclaw_search` | 搜索代码 | 查找函数、变量 |
| `aicodclaw_apply_patch` | 应用补丁 | 代码修改 |
| `aicodclaw_shell` | 执行命令 | 编译、测试、Git |
| `aicodclaw_deepseek` | AI 查询 | 代码分析 |
| `aicodclaw_deepseek-reply` | AI 回复 | 详细解答 |

---

## 📊 Token 节省效果

| 方式 | Token 消耗 | 积分 |
|------|-----------|------|
| WorkBuddy 自己处理 | ~35,000 | 20-50 |
| 调用 AICodeClaw | ~450 | 0.5-2 |
| **节省** | **98.7%** | **90-95%** |

---

## ⚙️ 环境变量

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `DEEPSEEK_MODEL` | `qwen3.5-9b-deepseek-v4-flash@q6_k` | AI 模型 |
| `DEEPSEEK_BASE_URL` | `http://localhost:11434/v1` | API 地址 |

---

## 🔧 开发

```bash
# 运行
npm start

# 测试
node index.js
```

---

**🦞 让 WorkBuddy 更智能，让 Token 更节省！**
