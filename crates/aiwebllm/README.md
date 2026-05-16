# Web LLM 集成需求规格文档

> **文档版本**: v1.0  
> **创建日期**: 2026-05-16  
> **状态**: 需求整理阶段（未开始开发）  
> **优先级**: 🔴 High（Phase 1 核心功能）

---

## 📋 执行摘要

本需求文档定义了 **Web LLM 集成模块**的完整功能需求，目标是将多个免费 Web 版 LLM（通义千问、豆包、GitHub Models、Gitee AI 等）集成到 DeepSeek TUI 中，作为**疑难问题解答助手**和**知识增强引擎**。

### 核心价值

| 价值点 | 预期收益 |
|--------|---------|
| 疑难问题解决 | 解决时间缩短 80% |
| 知识增强 | AI 可访问最新技术文档 |
| 成本优化 | 免费 Web LLM 降低 API 费用 |
| 智能路由 | 自动选择最佳 LLM 引擎 |

---

## 🎯 项目目标

### 主要目标

1. **Web 会话集成**：支持通义千问、豆包的 Web 版会话（已登录状态）
2. **网页抓取解析**：发送提示词 → 等待响应 → 抓取网页 → 解析结果
3. **多结果聚合**：同时调用多个 Web LLM，返回多个结果对比
4. **多轮优化**：一个提示词，多个 LLM 协作生成最优结果
5. **无缝集成**：与现有 DeepSeek TUI 命令系统完美融合

### 核心工作流程

```
用户提问 → 生成提示词 → 发送到 Web LLM → 等待响应完成 
  → 抓取网页内容 → 解析 HTML → 提取文本 → 返回格式化结果
```

### 多轮优化流程

```
Round 1: 提示词 → doubao 生成初稿
Round 2: doubao 结果 → qianwen 分析和优化
Round 3: qianwen 优化 → (可选) 再次迭代
最终: 返回 qianwen 的最终结果
```

### 非目标（本期不做）

- ❌ Web LLM 模型训练或微调
- ❌ 自定义 LLM 模型部署
- ❌ GitHub Models / Gitee AI 集成（后续版本）
- ❌ 多模态支持（图像、视频）

---

## 📁 目录结构设计

```
crates/aiwebllm/
├── src/
│   ├── lib.rs                  # 模块入口
│   ├── config.rs               # 配置管理（Web URL、Cookie、会话）
│   ├── client/
│   │   ├── mod.rs              # 客户端模块
│   │   ├── base.rs             # 基础客户端 trait
│   │   ├── qianwen.rs          # 通义千问 Web 客户端
│   │   └── doubao.rs           # 豆包 Web 客户端
│   ├── scraper.rs              # 网页抓取和解析器
│   ├── orchestrator.rs         # 多 LLM 协作编排器
│   ├── session.rs              # 会话管理
│   ├── cache.rs                # 结果缓存
│   ├── models.rs               # 数据模型
│   └── error.rs                # 错误处理
├── tests/
│   ├── qianwen_test.rs         # 通义千问集成测试
│   ├── doubao_test.rs          # 豆包集成测试
│   ├── scraper_test.rs         # 网页解析测试
│   └── orchestrator_test.rs    # 协作编排测试
├── examples/
│   └── basic_usage.rs          # 使用示例
└── Cargo.toml
```

---

## 🔧 功能需求

### FR-1: Web 会话客户端

#### FR-1.1: 通义千问 Web 客户端

**需求描述**：
- 集成通义千问 Web 版（已登录会话）
- 通过 Cookie 维持登录状态
- 发送提示词，等待响应完成，抓取结果

**Web URL 配置**：
```
https://www.qianwen.com/chat/a7daf5f3403c4f41a7f5bd3bcf391cf5
```

**技术要求**：
```rust
pub struct QianwenWebClient {
    web_url: String,              // 完整 Web 会话 URL
    cookies: CookieJar,           // 登录 Cookie
    http_client: reqwest::Client,
}

impl QianwenWebClient {
    /// 发送提示词并等待响应
    pub async fn send_prompt(&self, prompt: &str) -> Result<String> {
        // 1. 通过 JavaScript 或 API 发送消息
        self.inject_message(prompt).await?;
        
        // 2. 等待 AI 响应完成
        self.wait_for_completion().await?;
        
        // 3. 抓取网页内容
        let html = self.fetch_page().await?;
        
        // 4. 解析 HTML 提取回答文本
        let answer = self.parse_response(&html)?;
        
        Ok(answer)
    }
    
    /// 注入消息到网页
    async fn inject_message(&self, message: &str) -> Result<()>;
    
    /// 等待响应完成
    async fn wait_for_completion(&self) -> Result<()>;
    
    /// 抓取网页内容
    async fn fetch_page(&self) -> Result<String>;
    
    /// 解析 HTML 提取回答
    fn parse_response(&self, html: &str) -> Result<String>;
}
```

**配置示例**：
```toml
[web_llm.qianwen]
enabled = true
web_url = "https://www.qianwen.com/chat/a7daf5f3403c4f41a7f5bd3bcf391cf5"
cookies_file = "~/.deepseek/cookies/qianwen.json"
response_timeout = 120  # 最大等待时间（秒）
retry_count = 3
```

**验收标准**：
- ✅ 能发送提示词到 Web 会话
- ✅ 能等待响应完成
- ✅ 能抓取并解析网页内容
- ✅ 返回格式化的文本结果

---

#### FR-1.2: 豆包 Web 客户端

**需求描述**：
- 集成豆包 Web 版（已登录会话）
- 与通义千问相同的工作流程

**Web URL 配置**：
```
https://www.doubao.com/chat/?channel=sysceo&source=mf_db_sysceo
```

**技术要求**：
```rust
pub struct DoubaoWebClient {
    web_url: String,
    cookies: CookieJar,
    http_client: reqwest::Client,
}

impl DoubaoWebClient {
    pub async fn send_prompt(&self, prompt: &str) -> Result<String> {
        // 相同的工作流程
        self.inject_message(prompt).await?;
        self.wait_for_completion().await?;
        let html = self.fetch_page().await?;
        let answer = self.parse_response(&html)?;
        Ok(answer)
    }
}
```

**配置示例**：
```toml
[web_llm.doubao]
enabled = true
web_url = "https://www.doubao.com/chat/?channel=sysceo&source=mf_db_sysceo"
cookies_file = "~/.deepseek/cookies/doubao.json"
response_timeout = 120
retry_count = 3
```

**验收标准**：
- ✅ 能发送提示词到 Web 会话
- ✅ 能等待响应完成
- ✅ 能抓取并解析网页内容
- ✅ 返回格式化的文本结果

---

#### FR-1.3: GitHub Models 客户端

**需求描述**：
- 集成 GitHub Models（免费额度）
- 支持多种开源模型切换

**技术要求**：
```rust
pub struct GitHubModelsClient {
    api_key: String,            // GitHub Token
    model: String,              // 模型名称
    http_client: reqwest::Client,
}

impl GitHubModelsClient {
    pub async fn list_models(&self) -> Result<Vec<ModelInfo>>;
    pub async fn send_message(&self, message: &str) -> Result<String>;
    pub async fn switch_model(&mut self, model: &str) -> Result<()>;
}
```

**配置示例**：
```toml
[web_llm.github]
enabled = true
api_key = "${GITHUB_TOKEN}"
default_model = "meta-llama/meta-llama-3-70b-instruct"
available_models = [
    "meta-llama/meta-llama-3-70b-instruct",
    "mistralai/mixtral-8x7b-instruct",
    "microsoft/phi-3-medium"
]
```

**验收标准**：
- ✅ 能列出可用模型
- ✅ 能切换模型
- ✅ 能发送消息并收到回复
- ✅ 支持免费额度监控

---

#### FR-1.4: Gitee AI 客户端

**需求描述**：
- 集成 Gitee AI（国内免费）
- 支持中文优化模型

**技术要求**：
```rust
pub struct GiteeAIClient {
    api_key: String,
    model: String,
    http_client: reqwest::Client,
}

impl GiteeAIClient {
    pub async fn send_message(&self, message: &str) -> Result<String>;
}
```

**配置示例**：
```toml
[web_llm.gitee]
enabled = true
api_key = "${GITEE_TOKEN}"
default_model = "qwen-72b-chat"
```

**验收标准**：
- ✅ 能发送消息并收到回复
- ✅ 中文回答质量良好
- ✅ 国内访问速度快

---

### FR-2: 网页抓取和解析器

#### FR-2.1: 核心抓取流程

**需求描述**：
封装完整的"发送提示词 → 等待响应 → 抓取网页 → 解析结果"流程。

**工作流程**：
```
1. 生成提示词（从用户输入）
2. 注入到 Web LLM 会话
3. 等待 AI 响应完成（轮询或事件监听）
4. 抓取完整网页 HTML
5. 解析 HTML，提取回答文本
6. 清理格式，返回结构化数据
```

**技术要求**：
```rust
pub struct WebScraper {
    http_client: reqwest::Client,
    parser: HtmlParser,
}

pub struct WebScraper {
    /// 完整流程：发送提示词 → 返回结果
    pub async fn fetch_llm_response(
        &self,
        client: &dyn WebLlmClient,
        prompt: &str,
    ) -> Result<LlmResponse> {
        // 1. 发送提示词
        client.inject_message(prompt).await?;
        
        // 2. 等待响应完成
        client.wait_for_completion().await?;
        
        // 3. 抓取网页
        let html = client.fetch_page().await?;
        
        // 4. 解析 HTML
        let answer = self.parse_html_response(&html)?;
        
        // 5. 返回格式化结果
        Ok(LlmResponse {
            text: answer,
            source: client.name(),
            timestamp: chrono::Local::now(),
        })
    }
    
    /// 解析 HTML 提取 AI 回答
    fn parse_html_response(&self, html: &str) -> Result<String> {
        // 1. 定位回答区域（特定的 CSS 选择器）
        // 2. 提取文本内容
        // 3. 清理 HTML 标签
        // 4. 格式化输出
        unimplemented!()
    }
}
```

**HTML 解析策略**：
```rust
// 通义千问的回答选择器
const QIANWEN_ANSWER_SELECTORS: &[&str] = &[
    ".message-content",
    ".ai-response",
    "[data-role='assistant']",
];

// 豆包的回答选择器
const DOUBAO_ANSWER_SELECTORS: &[&str] = &[
    ".answer-content",
    ".bot-message",
    "[data-type='response']",
];
```

**验收标准**：
- ✅ 能正确抓取网页内容
- ✅ 能准确定位 AI 回答区域
- ✅ 能清理 HTML 标签提取纯文本
- ✅ 能处理网络异常和超时

---

### FR-3: 多 LLM 协作编排器

#### FR-3.1: 并行调用（返回多个结果）

**需求描述**：
同时调用多个 Web LLM，返回所有结果供对比。

**技术要求**：
```rust
pub struct LlmOrchestrator {
    clients: HashMap<String, Box<dyn WebLlmClient>>,
    scraper: WebScraper,
}

impl LlmOrchestrator {
    /// 并行调用多个 Web LLM
    pub async fn query_all(
        &self,
        prompt: &str,
    ) -> Result<Vec<LlmResponse>> {
        let futures: Vec<_> = self.clients
            .values()
            .map(|client| {
                self.scraper.fetch_llm_response(client.as_ref(), prompt)
            })
            .collect();
        
        // 并行执行
        let results = futures::future::join_all(futures).await;
        
        // 收集成功结果
        Ok(results
            .into_iter()
            .filter_map(|r| r.ok())
            .collect())
    }
}
```

**输出示例**：
```
┌─ 多 LLM 结果对比 ─────────────────────┐
│                                       │
│ 【通义千问】                          │
│ 优化 Rust 性能的方法：                │
│ 1. 使用零拷贝技术                     │
│ 2. 避免不必要的内存分配               │
│ ...                                   │
│                                       │
│ 【豆包】                              │
│ Rust 性能优化建议：                   │
│ - 减少堆分配                          │
│ - 使用 SIMD 指令                      │
│ ...                                   │
│                                       │
│ 耗时: 通义千问 2.3s | 豆包 1.8s      │
└───────────────────────────────────────┘
```

**验收标准**：
- ✅ 能并行调用多个 Web LLM
- ✅ 返回所有成功结果
- ✅ 显示对比格式

---

#### FR-3.2: 多轮优化（生成最优结果）

**需求描述**：
一个提示词，多个 LLM 协作，来回迭代 1-2 次，返回最优结果。

**工作流程**：
```
Round 1: 提示词 → doubao 生成初稿
Round 2: doubao 结果 → qianwen 分析和优化
Round 3: qianwen 优化结果 → (可选) 再次迭代
最终: 返回 qianwen 的最终结果
```

**技术要求**：
```rust
pub struct MultiRoundOptimizer {
    qianwen: QianwenWebClient,
    doubao: DoubaoWebClient,
    scraper: WebScraper,
    max_rounds: u32,  // 最大迭代次数（1-2次）
}

impl MultiRoundOptimizer {
    /// 多轮优化生成最优结果
    pub async fn optimize(
        &self,
        prompt: &str,
    ) -> Result<OptimizedResponse> {
        // Round 1: doubao 生成初稿
        let draft = self.scraper
            .fetch_llm_response(&self.doubao, prompt)
            .await?;
        
        // 生成分析提示词
        let analysis_prompt = format!(
            "请分析并优化以下回答，使其更准确、完整、专业：\n\n{}",
            draft.text
        );
        
        // Round 2: qianwen 分析和优化
        let optimized = self.scraper
            .fetch_llm_response(&self.qianwen, &analysis_prompt)
            .await?;
        
        // 可选 Round 3: 再次迭代
        if self.max_rounds > 2 {
            let refinement_prompt = format!(
                "请进一步完善以下回答：\n\n{}",
                optimized.text
            );
            
            let final_result = self.scraper
                .fetch_llm_response(&self.qianwen, &refinement_prompt)
                .await?;
            
            return Ok(OptimizedResponse {
                final_text: final_result.text,
                rounds: 3,
                contributors: vec!["doubao", "qianwen"],
            });
        }
        
        Ok(OptimizedResponse {
            final_text: optimized.text,
            rounds: 2,
            contributors: vec!["doubao", "qianwen"],
        })
    }
}
```

**配置示例**：
```toml
[web_llm.optimizer]
enabled = true
first_draft_llm = "doubao"        # 初稿生成器
optimizer_llm = "qianwen"         # 优化器
max_rounds = 2                    # 最大迭代次数（1-2）
return_final_only = true          # 只返回最终结果
```

**输出示例**：
```
┌─ 多轮优化完成 ──────────────────────┐
│                                       │
│ 迭代次数: 2 轮                        │
│ 参与者: 豆包 → 通义千问               │
│                                       │
│ 【最终结果】（通义千问优化）          │
│                                       │
│ Rust 性能优化的完整指南：             │
│                                       │
│ ## 1. 内存优化                        │
│ - 使用栈分配而非堆分配                │
│ - 避免不必要的克隆                    │
│ ...                                   │
│                                       │
│ ## 2. 编译优化                        │
│ - 开启 LTO                            │
│ - 使用 profile.release                │
│ ...                                   │
│                                       │
│ 总耗时: 4.5s                          │
└───────────────────────────────────────┘
```

**验收标准**：
- ✅ 能执行多轮迭代优化
- ✅ 迭代次数可控制（1-2次）
- ✅ 返回最终优化结果
- ✅ 显示优化过程信息

---

### FR-3: 会话管理

#### FR-3.1: 会话生命周期

**需求描述**：
管理 Web LLM 会话的创建、保持、超时、重建。

**会话状态机**：
```
[未创建] ──创建──► [活跃] ──超时──► [过期]
                    │                │
                 消息交互         自动重建
                    │                │
                    ▼                ▼
                 [活跃] ◄──────── [活跃]
```

**技术要求**：
```rust
pub struct SessionManager {
    sessions: HashMap<String, WebLlmSession>,
    default_timeout: Duration,
}

pub struct WebLlmSession {
    pub id: String,
    pub platform: String,          // "qianwen", "doubao", etc.
    pub created_at: Instant,
    pub last_activity: Instant,
    pub message_count: u32,
    pub state: SessionState,
}

pub enum SessionState {
    Active,
    Expired,
    Rebuilding,
    Failed,
}

impl SessionManager {
    /// 创建新会话
    pub async fn create_session(&mut self, platform: &str) -> Result<String>;
    
    /// 获取或创建会话
    pub async fn get_or_create(&mut self, platform: &str) -> Result<String>;
    
    /// 检查并刷新过期会话
    pub async fn refresh_expired(&mut self);
    
    /// 关闭会话
    pub async fn close_session(&mut self, session_id: &str);
}
```

**验收标准**：
- ✅ 会话超时自动重建
- ✅ 支持多平台并行会话
- ✅ 会话状态持久化（可选）

---

### FR-4: 会话管理

#### FR-4.1: Cookie 管理

**需求描述**：
管理 Web LLM 的登录 Cookie，维持会话状态。

**技术要求**：
```rust
pub struct CookieManager {
    cookies_dir: PathBuf,
    sessions: HashMap<String, CookieJar>,
}

impl CookieManager {
    /// 加载 Cookie
    pub fn load_cookies(&self, platform: &str) -> Result<CookieJar>;
    
    /// 保存 Cookie
    pub fn save_cookies(&self, platform: &str, cookies: &CookieJar) -> Result<()>;
    
    /// 检查 Cookie 是否过期
    pub fn is_cookie_expired(&self, platform: &str) -> bool;
}
```

**Cookie 存储格式**：
```json
// ~/.deepseek/cookies/qianwen.json
{
  "cookies": [
    {
      "name": "session_id",
      "value": "abc123...",
      "domain": ".qianwen.com",
      "path": "/",
      "expires": 1735689600
    }
  ],
  "last_updated": "2026-05-16T17:00:00Z"
}
```

**验收标准**：
- ✅ 能从文件加载 Cookie
- ✅ 能保存 Cookie 到文件
- ✅ 能检测 Cookie 过期

---

### FR-5: TUI 命令集成

#### FR-5.1: 命令列表

| 命令 | 功能 | 示例 |
|------|------|------|
| `/web qianwen <问题>` | 向通义千问提问 | `/web qianwen 如何优化 Rust 性能？` |
| `/web doubao <问题>` | 向豆包提问 | `/web doubao 解释借用检查` |
| `/web all <问题>` | 同时查询所有 Web LLM | `/web all 什么是生命周期？` |
| `/web optimize <问题>` | 多轮优化生成最优结果 | `/web optimize 设计一个缓存系统` |
| `/web status` | 查看 Web LLM 状态 | 显示会话状态 |
| `/web cookie set <平台>` | 设置 Cookie | `/web cookie set qianwen` |

#### FR-5.2: 输出格式

**单 LLM 回答**：
```
┌─ 通义千问 ──────────────────────────────┐
│                                         │
│ 优化 Rust 性能的几种方法：              │
│                                         │
│ 1. 使用零拷贝技术                       │
│ 2. 避免不必要的内存分配                 │
│ 3. 使用 SIMD 指令集                     │
│ ...                                     │
│                                         │
│ 耗时: 2.3s                              │
└─────────────────────────────────────────┘
```

**多 LLM 对比**：
```
┌─ 多 LLM 结果对比 ─────────────────────┐
│                                       │
│ 【通义千问】(2.3s)                    │
│ 优化方法：                            │
│ 1. 使用零拷贝技术                     │
│ 2. 避免不必要的内存分配               │
│                                       │
│ 【豆包】(1.8s)                        │
│ 优化建议：                            │
│ - 减少堆分配                          │
│ - 使用 SIMD 指令                      │
└───────────────────────────────────────┘
```

**多轮优化结果**：
```
┌─ 多轮优化完成 ──────────────────────┐
│ 迭代: 2轮 | 豆包 → 通义千问          │
├─────────────────────────────────────┤
│ 【最终结果】                        │
│                                     │
│ Rust 性能优化完整指南：             │
│                                     │
│ ## 1. 内存优化                      │
│ - 使用栈分配而非堆分配              │
│ - 避免不必要的克隆                  │
│ ...                                 │
│                                     │
│ 总耗时: 4.5s                        │
└─────────────────────────────────────┘
```

**验收标准**：
- ✅ 所有命令可用
- ✅ 输出格式美观
- ✅ 错误提示清晰

---

## 🔒 非功能需求

### NFR-1: 性能要求

| 指标 | 要求 | 说明 |
|------|------|------|
| 路由决策时间 | < 10ms | 复杂度评估 + 平台选择 |
| 缓存查询时间 | < 1ms | 缓存命中时 |
| 会话创建时间 | < 2s | 新会话创建 |
| 内存占用 | < 50MB | 所有客户端 + 缓存 |

### NFR-2: 可靠性要求

| 指标 | 要求 | 说明 |
|------|------|------|
| 可用性 | > 95% | 至少一个平台可用 |
| 自动降级 | 必须 | 平台不可用时自动切换 |
| 会话恢复 | 必须 | 超时后自动重建 |
| 错误处理 | 优雅降级 | 不阻塞主流程 |

### NFR-3: 安全要求

| 要求 | 说明 |
|------|------|
| API Key 保护 | 不硬编码，从环境变量或配置文件读取 |
| 数据传输 | 使用 HTTPS |
| 敏感信息 | 不在日志中输出 API Key |
| 访问控制 | 支持 YOLO/Agent/Plan 模式权限控制 |

### NFR-4: 兼容性要求

| 要求 | 说明 |
|------|------|
| Rust 版本 | >= 1.88 |
| 操作系统 | Windows / Linux / macOS |
| 网络环境 | 支持代理配置 |
| 向后兼容 | 不影响现有功能 |

---

## 📊 验收标准总览

### 功能验收

- [ ] FR-1: 4 个平台客户端全部可用
- [ ] FR-2: 智能路由准确率 > 85%
- [ ] FR-3: 会话管理稳定，无内存泄漏
- [ ] FR-4: 缓存命中率 > 30%
- [ ] FR-5: 7 个 TUI 命令全部可用

### 性能验收

- [ ] 路由决策 < 10ms
- [ ] 缓存查询 < 1ms
- [ ] 内存占用 < 50MB
- [ ] 并发会话数 >= 4

### 可靠性验收

- [ ] 连续运行 24 小时无崩溃
- [ ] 平台不可用时自动降级
- [ ] 会话超时自动重建
- [ ] 错误不阻塞主流程

---

## 🎯 开发优先级

### Phase 1.1: 基础框架（1周）

- [ ] 创建 `crates/aiwebllm` 目录结构
- [ ] 定义基础 trait 和数据模型
- [ ] 实现配置管理
- [ ] 实现错误处理

### Phase 1.2: 平台集成（2周）

- [ ] 实现通义千问客户端
- [ ] 实现豆包客户端
- [ ] 实现 GitHub Models 客户端
- [ ] 实现 Gitee AI 客户端

### Phase 1.3: 核心功能（1周）

- [ ] 实现智能路由器
- [ ] 实现会话管理器
- [ ] 实现健康检查
- [ ] 实现知识缓存

### Phase 1.4: TUI 集成（1周）

- [ ] 实现 TUI 命令
- [ ] 实现输出格式化
- [ ] 编写集成测试
- [ ] 编写文档

**总计**: 5 周

---

## 📝 配置模板

### 完整配置示例 (`config.toml`)

```toml
[web_llm]
enabled = true
auto_route = true               # 启用智能路由
health_check_interval = 300     # 健康检查间隔（秒）
default_timeout = 1800          # 默认会话超时（秒）

[web_llm.qianwen]
enabled = true
base_url = "https://www.qianwen.com/chat/{session_id}"
session_timeout = 1800
max_retries = 3
priority = 1                    # 优先级（1最高）

[web_llm.doubao]
enabled = true
base_url = "https://www.doubao.com/chat/{session_id}"
session_timeout = 1800
support_file_upload = true
priority = 2

[web_llm.github]
enabled = true
api_key = "${GITHUB_TOKEN}"
default_model = "meta-llama/meta-llama-3-70b-instruct"
session_timeout = 3600
priority = 3

[web_llm.gitee]
enabled = true
api_key = "${GITEE_TOKEN}"
default_model = "qwen-72b-chat"
session_timeout = 1800
priority = 4

[web_llm.cache]
enabled = true
max_entries = 1000
ttl_hours = 24
similarity_threshold = 0.9
persistent = true
cache_dir = "~/.deepseek/web_llm_cache"

[web_llm.router]
# 复杂度评估规则
simple_max_length = 50
moderate_max_length = 200
complex_keywords = ["架构", "性能优化", "算法设计", "系统设计"]
moderate_keywords = ["如何实现", "为什么", "对比", "最佳实践"]
simple_keywords = ["什么是", "怎么用", "语法", "示例"]

# 降级策略
fallback_order = ["qianwen", "doubao", "github", "gitee"]
max_consecutive_failures = 3
retry_after_seconds = 900
```

---

## 🔗 依赖关系

### Rust 依赖

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json", "cookies", "stream"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tracing = "0.1"
chrono = "0.4"
sha2 = "0.10"                    # 缓存键生成
similarity = "2.1"               # 语义相似度计算
lru = "0.12"                     # LRU 缓存
```

### 外部依赖

| 依赖 | 用途 | 必需 |
|------|------|------|
| 通义千问 Web 端 | LLM 平台 | 推荐 |
| 豆包 Web 端 | LLM 平台 | 推荐 |
| GitHub Token | GitHub Models 认证 | 可选 |
| Gitee Token | Gitee AI 认证 | 可选 |

---

## 🚀 下一步行动

1. ✅ **需求评审**：团队评审本需求文档
2. ⏳ **技术验证**：验证各平台 API 可行性
3. ⏳ **原型开发**：创建最小可行原型（MVP）
4. ⏳ **正式开发**：按 Phase 1.1 ~ 1.4 顺序开发

---

> **备注**: 本文档为需求规格，不含具体实现代码。开发前需进行技术可行性验证，特别是 Web 端 API 的逆向工程。
