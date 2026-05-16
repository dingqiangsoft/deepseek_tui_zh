# Web LLM 集成需求规格 - 概要

> **版本**: v2.0 (网页抓取方案)  
> **日期**: 2026-05-16  
> **状态**: 需求完成，待技术验证

---

## 🎯 核心需求

### 1. Web 会话集成

**目标平台**：
- ✅ 通义千问：https://www.qianwen.com/chat/a7daf5f3403c4f41a7f5bd3bcf391cf5
- ✅ 豆包：https://www.doubao.com/chat/?channel=sysceo&source=mf_db_sysceo

**工作方式**：
```
用户提问 → 生成提示词 → 发送到 Web LLM → 等待响应完成 
  → 抓取网页内容 → 解析 HTML → 提取文本 → 返回格式化结果
```

---

### 2. 核心功能

#### 功能 1: 单个 Web LLM 查询

```bash
/web qianwen "如何优化 Rust 性能？"
```

**流程**：
1. 发送提示词到通义千问
2. 等待 AI 响应完成
3. 抓取网页 HTML
4. 解析并提取回答文本
5. 返回格式化结果

---

#### 功能 2: 并行查询多个 Web LLM

```bash
/web all "什么是生命周期？"
```

**流程**：
1. 同时发送到通义千问和豆包
2. 等待两个都响应完成
3. 抓取两个网页
4. 解析并对比结果
5. 返回所有结果

**输出示例**：
```
┌─ 多 LLM 结果对比 ─────────────────────┐
│ 【通义千问】(2.3s)                    │
│ 生命周期是 Rust 的核心概念...         │
│                                       │
│ 【豆包】(1.8s)                        │
│ 生命周期用于管理引用有效性...         │
└───────────────────────────────────────┘
```

---

#### 功能 3: 多轮优化生成最优结果

```bash
/web optimize "设计一个缓存系统"
```

**流程**：
```
Round 1: 提示词 → doubao 生成初稿
Round 2: doubao 结果 → qianwen 分析和优化
Round 3: (可选) 再次迭代
最终: 返回 qianwen 的最终结果
```

**输出示例**：
```
┌─ 多轮优化完成 ──────────────────────┐
│ 迭代: 2轮 | 豆包 → 通义千问          │
├─────────────────────────────────────┤
│ 【最终结果】                        │
│                                     │
│ 缓存系统完整设计方案：              │
│ 1. 数据结构设计                     │
│ 2. 淘汰策略                         │
│ 3. 并发处理                         │
│ ...                                 │
│                                     │
│ 总耗时: 4.5s                        │
└─────────────────────────────────────┘
```

---

## 🔧 技术要点

### 1. Cookie 认证

- 用户手动登录 Web LLM
- 导出 Cookie 到 JSON 文件
- 程序使用 Cookie 维持登录状态

**文件位置**：
```
~/.deepseek/cookies/
├── qianwen.json
└── doubao.json
```

---

### 2. 网页抓取

**核心步骤**：
1. **注入消息**：通过 JavaScript 或 API 发送提示词
2. **等待完成**：轮询检测 AI 是否响应完成
3. **抓取 HTML**：获取完整网页内容
4. **解析提取**：使用 CSS 选择器定位回答区域，提取文本

**关键技术**：
- HTTP 客户端：`reqwest`
- HTML 解析：`scraper` 或 `kuchiki`
- Cookie 管理：`reqwest::cookie::CookieStore`

---

### 3. 多 LLM 协作

**并行调用**：
```rust
let results = futures::future::join_all(futures).await;
```

**多轮优化**：
```rust
// Round 1
let draft = doubao.send_prompt(prompt).await?;

// Round 2
let optimized = qianwen.send_prompt(
    &format!("请优化：{}", draft)
).await?;
```

---

## 📋 配置示例

```toml
[web_llm.qianwen]
enabled = true
web_url = "https://www.qianwen.com/chat/a7daf5f3403c4f41a7f5bd3bcf391cf5"
cookies_file = "~/.deepseek/cookies/qianwen.json"
response_timeout = 120

[web_llm.doubao]
enabled = true
web_url = "https://www.doubao.com/chat/?channel=sysceo&source=mf_db_sysceo"
cookies_file = "~/.deepseek/cookies/doubao.json"
response_timeout = 120

[web_llm.optimizer]
enabled = true
first_draft_llm = "doubao"
optimizer_llm = "qianwen"
max_rounds = 2
```

---

## 🎮 TUI 命令

| 命令 | 功能 |
|------|------|
| `/web qianwen <问题>` | 向通义千问提问 |
| `/web doubao <问题>` | 向豆包提问 |
| `/web all <问题>` | 同时查询所有 Web LLM |
| `/web optimize <问题>` | 多轮优化生成最优结果 |
| `/web status` | 查看 Web LLM 状态 |
| `/web cookie set <平台>` | 设置 Cookie |

---

## ⏱️ 开发计划

| 阶段 | 时间 | 内容 |
|------|------|------|
| 技术验证 | 3-5天 | 验证网页抓取可行性 |
| Phase 1.1 | 1周 | Cookie 管理 + 基础客户端 |
| Phase 1.2 | 1周 | 网页抓取和解析器 |
| Phase 1.3 | 1周 | 多 LLM 协作编排器 |
| Phase 1.4 | 1周 | TUI 集成 + 测试 |
| **总计** | **4-5周** | 完整实现 |

---

## 📂 文档索引

- [完整需求规格](./README.md) - 详细技术文档
- [Cookie 配置指南](./config/COOKIE_SETUP.md) - 如何获取 Cookie
- [配置模板](./config/web_llm.toml.example) - 配置文件示例
- [项目状态](./PROJECT_STATUS.md) - 进度追踪

---

## ✅ 下一步

1. **技术验证**：验证网页抓取方案可行性
   - 分析通义千问网页结构
   - 分析豆包网页结构
   - 确定消息注入方式
   - 确定响应完成检测方法

2. **创建待办事项**：基于验证结果创建详细开发任务

3. **开始开发**：按 Phase 1.1 ~ 1.4 顺序实施

---

> **备注**: 需求已冻结，进入技术验证阶段
