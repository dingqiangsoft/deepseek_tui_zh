# AICodeClaw Token Guard

> **核心优势：** 定制开发工具，让 WorkBuddy 更懂你的项目！

---

## 🌟 为什么选择 AICodeClaw？

### 最大优势：完全可定制

与通用 MCP 工具不同，AICodeClaw 的**每个工具都可以根据你的项目需求定制**：

- ✅ **自定义文件操作**：支持你的项目结构和规范
- ✅ **专属代码搜索**：适配你的代码库和命名约定
- ✅ **个性化 Shell 命令**：预配置你的构建、测试、部署流程
- ✅ **定制 AI 分析**：针对你的技术栈和业务逻辑优化
- ✅ **私有化部署**：数据完全在本地，零泄露风险

### 对比通用工具

| 特性 | 通用 MCP 工具 | AICodeClaw |
|------|--------------|------------|
| 工具定制 | ❌ 固定功能 | ✅ 完全可定制 |
| 项目适配 | ❌ 通用方案 | ✅ 专属优化 |
| 数据安全 | ⚠️ 可能上传云端 | ✅ 100% 本地 |
| Token 消耗 | 高 | 节省 90-95% |
| 响应速度 | 慢（云端） | 快（本地） |

---

## 📊 Token 节省效果

| 方式 | Token 消耗 | 积分消耗 |
|------|-----------|---------|
| WorkBuddy 自己处理 | ~35,000 | 20-50 积分 |
| 调用 AICodeClaw | ~450 | 0.5-2 积分 |
| **节省** | **98.7%** | **90-95%** |

---

## 🛠️ AICodeClaw 工具清单

| 工具 | 用途 |
|------|------|
| `aicodclaw_file_read` | 读取文件 |
| `aicodclaw_file_write` | 写入文件 |
| `aicodclaw_search` | 搜索代码 |
| `aicodclaw_apply_patch` | 应用修改 |
| `aicodclaw_shell` | 执行命令 |
| `aicodclaw_deepseek` | AI 查询 |
| `aicodclaw_deepseek-reply` | AI 回复 |
| `aicodclaw_yolo` | 🚀 **YOLO 模式**：批量自动执行操作 |

---

## 📋 使用规则

### ✅ 应该调用 AICodeClaw 的场景

- 需要读取/修改文件
- 需要搜索代码
- 需要执行命令（编译、测试、Git）
- 需要应用代码修改
- 需要 AI 分析代码

### ❌ 不要自己做

- 不要自己读取文件内容
- 不要自己搜索代码
- 不要自己执行命令
- 不要自己生成代码修改

---

## 🎯 决策流程

```
用户请求
  ↓
能调用 AICodeClaw 完成？
  ├─ 是 → 立即调用（消耗 < 500 Token）
  └─ 否 → 自己处理
```

---

## 💡 示例

### 示例 1：修改文件

```
用户：修改 config.toml 的数据库配置

❌ 错误：自己读取 → 修改 → 写入（消耗 5000+ Token）
✅ 正确：
  1. 调用 aicodclaw_file_read 读取
  2. 调用 aicodclaw_file_write 写入
  消耗：~300 Token
```

### 示例 2：运行测试

```
用户：运行 cargo test

❌ 错误：自己尝试执行（失败或消耗大量 Token）
✅ 正确：
  调用 aicodclaw_shell 执行 "cargo test"
  消耗：~150 Token
```

### 示例 3：重构代码

```
用户：重构这个函数

❌ 错误：自己分析 → 生成 → 修改（消耗 30000+ Token）
✅ 正确：
  1. 调用 aicodclaw_file_read 读取代码
  2. 调用 aicodclaw_search 查找引用
  3. 调用 aicodclaw_apply_patch 应用修改
  4. 调用 aicodclaw_shell 运行测试
  消耗：~600 Token
```

---

## ⚙️ 配置

### WorkBuddy MCP 配置

```json
{
  "mcpServers": {
    "aicodclaw": {
      "command": "node",
      "args": [
        "F:\\ai\\codes\\github\\deepseektuizh\\aicodclaw-mcp-server\\index.js"
      ],
      "env": {
        "DEEPSEEK_MODEL": "qwen3.5-9b-deepseek-v4-flash@q6_k"
      },
      "disabled": false
    }
  }
}
```

### WorkBuddy 模型配置（本地化优化）

针对本地运行 9B 模型 + 开发工具场景，推荐配置：

| 配置项 | 推荐值 | 说明 |
|--------|--------|------|
| **工具调用** | ✅ 勾选 | 必须开启，否则无法调用 MCP 工具 |
| **图片输入** | ❌ 不勾选 | 纯代码处理不需要图片识别，节省 Token |
| **推理模式** | ❌ 不勾选 | 开发辅助用标准模式更快更省 Token |
| **输入上下文** | 32K 或 64K | 9B 模型不建议超过 64K，影响速度 |
| **输出长度** | 8K 或 16K | 代码生成 8K 足够，避免浪费 |
| **自定义协议** | ❌ 不勾选 | 使用默认值即可 |

**配置理由：**
- ✅ **工具调用**是实现"让 MCP 多干活"的核心开关
- ❌ **图片输入**会增加上下文处理复杂度，对代码任务无益
- ❌ **推理模式**显著增加响应时间和 Token 消耗
- 📏 **32K 输入**足够处理多个源文件和日志
- 📏 **8K 输出**足够容纳完整代码和详细解释

### 本地 Skill 配置

```yaml
skills:
  AICodeClawTokenGuard:
    enabled: true
    auto_invoke: true
    prefer_aicodclaw: true
```

---

## ⚠️ 注意

- AICodeClaw 在用户本地运行，数据安全
- 不用担心 Token 消耗，该调用就调用
- 调用失败时检查：
  1. Node.js 是否已安装（`node --version`）
  2. MCP Server 路径是否正确
  3. WorkBuddy 的 mcp.json 配置是否正确

---

## 🧪 测试

运行测试脚本验证 MCP Server：

```bash
cd F:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server
node test.js
```

预期输出：
```
✅ 初始化成功
✅ 获取到 7 个工具
✅ 文件写入成功
✅ 文件读取成功
✅ Shell 命令执行成功
✅ 代码搜索正常
```

---

**记住：能调用 AICodeClaw 就调用，节省 90-95% Token！**
