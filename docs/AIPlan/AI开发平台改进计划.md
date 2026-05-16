# DeepSeek TUI AI 开发平台改进计划

> **文档版本**: v1.0  
> **创建日期**: 2026-05-16  
> **目标**: 打造集 Web LLM、开发工具指挥、自动化测试、工作流对接于一体的 AI 开发平台

---

## 📋 执行摘要

本计划分三个阶段将 DeepSeek TUI 从**终端 AI 助手**升级为**全栈 AI 开发平台**：

1. **Phase 1** (2-3周): Web LLM 集成 + 疑难问题解答
2. **Phase 2** (4-6周): 开发工具指挥系统 + 代码开发自动化
3. **Phase 3** (6-8周): 自动化测试平台 + 工作流对接

**预期收益**:
- ✅ 开发效率提升 300%+（AI 指挥开发工具）
- ✅ 疑难问题解决时间缩短 80%（Web LLM 辅助）
- ✅ 测试覆盖率提升 90%+（自动化测试平台）
- ✅ 工作流集成降低人工干预 70%

---

## 🎯 Phase 1: Web LLM 集成（2-3周）

### 1.1 目标

集成免费 Web 版 LLM（通义千问），作为**疑难问题解答助手**和**知识增强引擎**。

### 1.2 架构设计

```
┌─────────────────────────────────────────────────┐
│           DeepSeek TUI 主程序                     │
├─────────────────────────────────────────────────┤
│                                                  │
│  ┌──────────────┐      ┌──────────────────┐    │
│  │ 本地 LLM     │      │ Web LLM 网关     │    │
│  │ (Qwen 3.5)   │◄────►│ (通义千问)       │    │
│  │ 192.168.2.5  │      │ qianwen.com      │    │
│  └──────────────┘      └──────────────────┘    │
│         │                      │                │
│         ▼                      ▼                │
│  ┌──────────────────────────────────────┐      │
│  │    LLM 路由器（智能分流）            │      │
│  │  - 常规任务 → 本地 LLM               │      │
│  │  - 疑难问题 → Web LLM                │      │
│  │  - 复杂推理 → 双引擎协作             │      │
│  └──────────────────────────────────────┘      │
└─────────────────────────────────────────────────┘
```

### 1.3 功能清单

#### 1.3.1 Web LLM 连接器

**文件**: `crates/web-llm/`

```
crates/web-llm/
├── src/
│   ├── lib.rs              # 模块入口
│   ├── client.rs           # HTTP 客户端（会话管理）
│   ├── session.rs          # 会话状态追踪
│   ├── router.rs           # LLM 路由器（智能分流）
│   └── config.rs           # 配置管理
├── tests/
│   └── integration_test.rs
└── Cargo.toml
```

**核心代码结构**:

```rust
// crates/web-llm/src/client.rs
pub struct WebLlmClient {
    base_url: String,
    session_id: String,
    http_client: reqwest::Client,
}

impl WebLlmClient {
    pub async fn send_message(&self, message: &str) -> Result<String> {
        // 1. 构建请求
        let payload = WebLlmRequest {
            session_id: self.session_id.clone(),
            message: message.to_string(),
            stream: false,
        };
        
        // 2. 发送请求
        let response = self.http_client
            .post(&self.base_url)
            .json(&payload)
            .send()
            .await?;
        
        // 3. 解析响应
        let result = response.json::<WebLlmResponse>().await?;
        Ok(result.reply)
    }
}
```

#### 1.3.2 LLM 智能路由器

**文件**: `crates/core/src/llm_router.rs`

```rust
// LLM 路由器 - 根据问题复杂度智能选择引擎
pub struct LlmRouter {
    local_llm: LocalLlmClient,
    web_llm: WebLlmClient,
}

impl LlmRouter {
    pub async fn route_query(&self, query: &str) -> LlmEngine {
        let complexity = self.assess_complexity(query);
        
        match complexity {
            Complexity::Simple => LlmEngine::Local,      // 常规任务用本地
            Complexity::Complex => LlmEngine::Web,       // 疑难问题用 Web
            Complexity::VeryComplex => LlmEngine::Both,  // 复杂推理双引擎协作
        }
    }
    
    fn assess_complexity(&self, query: &str) -> Complexity {
        // 基于关键词、长度、领域判断复杂度
        if query.contains("为什么") || query.contains("如何优化") {
            Complexity::Complex
        } else if query.contains("架构设计") || query.contains("性能调优") {
            Complexity::VeryComplex
        } else {
            Complexity::Simple
        }
    }
}
```

#### 1.3.3 TUI 命令集成

**新增命令**:

```
/web-llm <问题>          # 直接向 Web LLM 提问
/ask <问题>              # 智能路由（自动选择本地或 Web）
/knowledge <主题>         # 知识库查询（缓存 Web LLM 回答）
```

**配置示例** (`config.toml`):

```toml
[web_llm]
enabled = true
base_url = "https://www.qianwen.com/chat/a7daf5f3403c4f41a7f5bd3bcf391cf5"
session_timeout = 1800  # 30分钟
auto_route = true       # 启用智能路由
cache_enabled = true    # 缓存 Web LLM 回答
```

### 1.4 实施步骤

| 步骤 | 任务 | 耗时 | 负责人 |
|------|------|------|--------|
| 1 | 创建 `crates/web-llm` 模块 | 2天 | 开发团队 |
| 2 | 实现 HTTP 客户端和会话管理 | 3天 | 开发团队 |
| 3 | 实现 LLM 路由器 | 2天 | 开发团队 |
| 4 | 集成到 TUI 命令系统 | 2天 | 开发团队 |
| 5 | 编写测试用例 | 2天 | 测试团队 |
| 6 | 文档和用户指南 | 1天 | 文档团队 |

**总计**: 12天（约2周）

---

## 🚀 Phase 2: 开发工具指挥系统（4-6周）

### 2.1 目标

实现 AI 指挥主流开发工具（VSCode、Qoder、Trae）进行**代码开发、会话监控、自动测试**的全流程自动化。

### 2.2 架构设计

```
┌─────────────────────────────────────────────────┐
│         AI 开发指挥中心                          │
├─────────────────────────────────────────────────┤
│                                                  │
│  ┌──────────────────────────────────────────┐  │
│  │  工具管理器 (Tool Manager)                │  │
│  │  - VSCode 连接器                          │  │
│  │  - Qoder 连接器                           │  │
│  │  - Trae 连接器                            │  │
│  └──────────────┬───────────────────────────┘  │
│                 │                               │
│  ┌──────────────▼───────────────────────────┐  │
│  │  任务编排器 (Task Orchestrator)           │  │
│  │  - 代码生成任务                           │  │
│  │  - 代码审查任务                           │  │
│  │  - 重构任务                               │  │
│  └──────────────┬───────────────────────────┘  │
│                 │                               │
│  ┌──────────────▼───────────────────────────┐  │
│  │  会话监控器 (Session Monitor)             │  │
│  │  - 实时监控开发工具会话                   │  │
│  │  - 进度追踪                               │  │
│  │  - 异常检测                               │  │
│  └──────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

### 2.3 功能清单

#### 2.3.1 开发工具连接器

**文件**: `crates/dev-tools/`

```
crates/dev-tools/
├── src/
│   ├── lib.rs
│   ├── vscode.rs           # VSCode 连接器
│   ├── qoder.rs            # Qoder 连接器
│   ├── trae.rs             # Trae 连接器
│   ├── session.rs          # 会话管理
│   └── monitor.rs          # 会话监控
├── tests/
│   └── connector_test.rs
└── Cargo.toml
```

**VSCode 连接器示例**:

```rust
// crates/dev-tools/src/vscode.rs
pub struct VsCodeConnector {
    workspace_path: String,
    extension_api: ExtensionApi,
    session_id: String,
}

impl VsCodeConnector {
    pub async fn open_file(&self, path: &str) -> Result<()> {
        self.extension_api
            .execute_command("vscode.open", &[path])
            .await
    }
    
    pub async fn edit_code(&self, instructions: &str) -> Result<EditResult> {
        // 1. 发送编辑指令到 AI 扩展
        let task = CodeEditTask {
            instructions: instructions.to_string(),
            workspace: self.workspace_path.clone(),
        };
        
        // 2. 执行编辑
        let result = self.extension_api
            .execute_command("ai.edit", &[task])
            .await?;
        
        // 3. 返回编辑结果
        Ok(result)
    }
    
    pub async fn run_tests(&self, test_path: &str) -> Result<TestReport> {
        // 执行测试并返回报告
        self.extension_api
            .execute_command("testing.run", &[test_path])
            .await
    }
}
```

#### 2.3.2 任务编排器

**文件**: `crates/dev-tools/src/orchestrator.rs`

```rust
// 任务编排器 - 指挥多个开发工具协作
pub struct TaskOrchestrator {
    tools: HashMap<String, Box<dyn DevTool>>,
    active_tasks: Vec<Task>,
}

impl TaskOrchestrator {
    pub async fn execute_development_task(
        &mut self,
        task: DevelopmentTask,
    ) -> Result<TaskReport> {
        match task {
            DevelopmentTask::CodeGeneration(spec) => {
                self.generate_code(spec).await
            }
            DevelopmentTask::CodeReview(pr_id) => {
                self.review_code(pr_id).await
            }
            DevelopmentTask::Refactor(module) => {
                self.refactor_code(module).await
            }
        }
    }
    
    async fn generate_code(&self, spec: CodeSpec) -> Result<TaskReport> {
        // 1. 选择最佳工具（基于任务类型）
        let tool = self.select_best_tool(&spec);
        
        // 2. 生成代码
        let code = tool.generate(&spec).await?;
        
        // 3. 自动测试
        let test_result = self.run_tests(&code).await?;
        
        // 4. 返回报告
        Ok(TaskReport {
            code,
            test_result,
            tool_used: tool.name(),
        })
    }
}
```

#### 2.3.3 会话监控器

**文件**: `crates/dev-tools/src/monitor.rs`

```rust
// 会话监控器 - 实时监控开发工具状态
pub struct SessionMonitor {
    sessions: HashMap<String, SessionState>,
    event_stream: mpsc::Receiver<SessionEvent>,
}

impl SessionMonitor {
    pub async fn monitor(&mut self) {
        while let Some(event) = self.event_stream.recv().await {
            match event {
                SessionEvent::TaskStarted { session_id, task } => {
                    self.sessions.entry(session_id)
                        .or_default()
                        .update_state(SessionState::Running(task));
                }
                SessionEvent::TaskCompleted { session_id, result } => {
                    self.sessions.entry(session_id)
                        .or_default()
                        .update_state(SessionState::Completed(result));
                }
                SessionEvent::Error { session_id, error } => {
                    self.sessions.entry(session_id)
                        .or_default()
                        .update_state(SessionState::Failed(error));
                }
            }
        }
    }
    
    pub fn get_session_status(&self, session_id: &str) -> Option<&SessionState> {
        self.sessions.get(session_id)
    }
}
```

#### 2.3.4 TUI 命令集成

**新增命令**:

```
/dev vs-code <指令>        # 指挥 VSCode 执行任务
/dev qoder <指令>          # 指挥 Qoder 执行任务
/dev trae <指令>           # 指挥 Trae 执行任务

/dev generate <需求>       # AI 生成代码
/dev review <文件>         # AI 代码审查
/dev refactor <模块>       # AI 重构代码

/dev monitor               # 查看所有开发工具会话状态
/dev status <session-id>   # 查看特定会话详情
/dev stop <session-id>     # 停止特定会话
```

**使用示例**:

```
# 1. 指挥 VSCode 生成代码
/dev vs-code "创建一个用户登录组件，包含表单验证和 API 调用"

# 2. 查看会话状态
/dev monitor
┌─────────────────────────────────────┐
│ 会话 ID  │ 工具    │ 状态   │ 进度  │
├─────────────────────────────────────┤
│ abc123   │ VSCode  │ 运行中 │ 75%   │
│ def456   │ Qoder   │ 已完成 │ 100%  │
│ ghi789   │ Trae    │ 失败   │ 30%   │
└─────────────────────────────────────┘

# 3. AI 自动测试
/dev test abc123

# 4. 查看测试报告
/dev report abc123
```

### 2.4 实施步骤

| 步骤 | 任务 | 耗时 | 负责人 |
|------|------|------|--------|
| 1 | 创建 `crates/dev-tools` 模块 | 3天 | 架构师 |
| 2 | 实现 VSCode 连接器 | 5天 | 开发团队 |
| 3 | 实现 Qoder 连接器 | 5天 | 开发团队 |
| 4 | 实现 Trae 连接器 | 5天 | 开发团队 |
| 5 | 实现任务编排器 | 7天 | 架构师 |
| 6 | 实现会话监控器 | 5天 | 开发团队 |
| 7 | 集成到 TUI 命令系统 | 3天 | 开发团队 |
| 8 | 编写集成测试 | 5天 | 测试团队 |
| 9 | 文档和用户指南 | 2天 | 文档团队 |

**总计**: 40天（约6周）

---

## 🧪 Phase 3: 自动化测试平台 + 工作流对接（6-8周）

### 3.1 目标

实现**开发完成后自动测试**的全流程，并对接企业工作流系统（CI/CD、项目管理）。

### 3.2 架构设计

```
┌─────────────────────────────────────────────────┐
│         自动化测试平台                           │
├─────────────────────────────────────────────────┤
│                                                  │
│  ┌──────────────┐      ┌──────────────────┐    │
│  │ 代码生成完成  │─────►│ 自动测试引擎     │    │
│  │              │      │                  │    │
│  └──────────────┘      │  ┌────────────┐ │    │
│                         │  │单元测试    │ │    │
│  ┌──────────────┐      │  ├────────────┤ │    │
│  │ 人工审查通过  │─────►│  │集成测试    │ │    │
│  │              │      │  ├────────────┤ │    │
│  └──────────────┘      │  │端到端测试  │ │    │
│                         │  └────────────┘ │    │
│                         └────────┬─────────┘    │
│                                  │               │
│  ┌───────────────────────────────▼──────────┐  │
│  │  工作流对接 (Workflow Integration)        │  │
│  │  - GitHub Actions / GitLab CI            │  │
│  │  - Jira / 项目管理                        │  │
│  │  - 代码审查 / PR 自动创建                 │  │
│  └──────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

### 3.3 功能清单

#### 3.3.1 自动化测试引擎

**文件**: `crates/test-engine/`

```
crates/test-engine/
├── src/
│   ├── lib.rs
│   ├── unit_test.rs        # 单元测试
│   ├── integration_test.rs # 集成测试
│   ├── e2e_test.rs         # 端到端测试
│   ├── reporter.rs         # 测试报告生成
│   └── config.rs           # 测试配置
├── tests/
│   └── engine_test.rs
└── Cargo.toml
```

**测试引擎核心代码**:

```rust
// crates/test-engine/src/lib.rs
pub struct TestEngine {
    unit_runner: UnitTestRunner,
    integration_runner: IntegrationTestRunner,
    e2e_runner: E2eTestRunner,
}

impl TestEngine {
    pub async fn run_full_test_suite(
        &self,
        code_path: &str,
    ) -> Result<TestReport> {
        let mut report = TestReport::new(code_path);
        
        // 1. 单元测试
        let unit_result = self.unit_runner
            .run(code_path)
            .await?;
        report.add_phase("Unit Tests", unit_result);
        
        if !unit_result.passed {
            return Ok(report); // 失败则跳过后续
        }
        
        // 2. 集成测试
        let integration_result = self.integration_runner
            .run(code_path)
            .await?;
        report.add_phase("Integration Tests", integration_result);
        
        // 3. 端到端测试
        let e2e_result = self.e2e_runner
            .run(code_path)
            .await?;
        report.add_phase("E2E Tests", e2e_result);
        
        Ok(report)
    }
}
```

#### 3.3.2 工作流对接

**文件**: `crates/workflow/`

```
crates/workflow/
├── src/
│   ├── lib.rs
│   ├── github_actions.rs   # GitHub Actions 集成
│   ├── jira.rs             # Jira 集成
│   ├── pr_creator.rs       # PR 自动创建
│   └── notification.rs     # 通知系统
├── tests/
│   └── workflow_test.rs
└── Cargo.toml
```

**GitHub Actions 集成示例**:

```rust
// crates/workflow/src/github_actions.rs
pub struct GitHubActionsClient {
    token: String,
    repo: String,
}

impl GitHubActionsClient {
    pub async fn trigger_workflow(
        &self,
        workflow: &str,
        inputs: HashMap<String, String>,
    ) -> Result<WorkflowRun> {
        let payload = WorkflowTriggerRequest {
            ref: "main".to_string(),
            inputs,
        };
        
        let response = self.http_client
            .post(&format!(
                "https://api.github.com/repos/{}/actions/workflows/{}/dispatches",
                self.repo, workflow
            ))
            .bearer_auth(&self.token)
            .json(&payload)
            .send()
            .await?;
        
        Ok(response.json().await?)
    }
    
    pub async fn create_pull_request(
        &self,
        title: &str,
        body: &str,
        head: &str,
        base: &str,
    ) -> Result<PullRequest> {
        let payload = CreatePrRequest {
            title: title.to_string(),
            body: body.to_string(),
            head: head.to_string(),
            base: base.to_string(),
        };
        
        // 创建 PR
        let pr = self.http_client
            .post(&format!(
                "https://api.github.com/repos/{}/pulls",
                self.repo
            ))
            .bearer_auth(&self.token)
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;
        
        Ok(pr)
    }
}
```

#### 3.3.3 完整开发流程自动化

**文件**: `crates/core/src/auto_dev_pipeline.rs`

```rust
// 自动化开发流水线 - 从需求到部署
pub struct AutoDevPipeline {
    dev_tools: TaskOrchestrator,
    test_engine: TestEngine,
    workflow: WorkflowClient,
}

impl AutoDevPipeline {
    pub async fn execute_full_pipeline(
        &mut self,
        requirement: &str,
    ) -> Result<PipelineReport> {
        let mut report = PipelineReport::new();
        
        // 1. AI 生成代码
        report.add_step("代码生成");
        let code = self.dev_tools
            .generate_code(requirement)
            .await?;
        
        // 2. 自动测试
        report.add_step("自动测试");
        let test_report = self.test_engine
            .run_full_test_suite(&code.path)
            .await?;
        
        if !test_report.all_passed() {
            report.mark_failed("测试失败");
            return Ok(report);
        }
        
        // 3. 创建 PR
        report.add_step("创建 PR");
        let pr = self.workflow
            .create_pull_request(
                &format!("feat: {}", requirement),
                &format!("自动生成:\n\n{}\n\n测试报告:\n{}", 
                    requirement, test_report.summary()),
                &code.branch,
                "main",
            )
            .await?;
        
        // 4. 触发 CI/CD
        report.add_step("触发 CI/CD");
        self.workflow
            .trigger_workflow("ci.yml", HashMap::new())
            .await?;
        
        report.mark_completed();
        Ok(report)
    }
}
```

#### 3.3.4 TUI 命令集成

**新增命令**:

```
/test auto <文件>            # 自动运行完整测试套件
/test unit <文件>            # 仅运行单元测试
/test integration <文件>     # 仅运行集成测试
/test e2e <文件>             # 仅运行端到端测试

/workflow trigger <工作流>   # 触发工作流
/workflow pr create          # 自动创建 PR
/workflow ci status          # 查看 CI 状态

/pipeline run <需求>         # 执行完整开发流水线
/pipeline status             # 查看流水线状态
```

**完整流程示例**:

```
# 1. 从需求到代码生成
/dev generate "创建用户认证模块，包含登录、注册、密码重置"

# 2. 自动测试
/test auto ./src/auth/

# 3. 查看测试报告
/test report

# 4. 如果测试通过，自动创建 PR
/workflow pr create

# 5. 或者直接执行完整流水线
/pipeline run "实现支付接口集成"

流水线执行中...
✓ 代码生成完成 (2分30秒)
✓ 单元测试通过 (45个用例)
✓ 集成测试通过 (12个用例)
✓ E2E 测试通过 (8个用例)
✓ PR 已创建: https://github.com/xxx/pull/123
✓ CI/CD 已触发

全部完成！总耗时: 5分15秒
```

### 3.4 实施步骤

| 步骤 | 任务 | 耗时 | 负责人 |
|------|------|------|--------|
| 1 | 创建 `crates/test-engine` 模块 | 5天 | 架构师 |
| 2 | 实现单元测试引擎 | 5天 | 开发团队 |
| 3 | 实现集成测试引擎 | 7天 | 开发团队 |
| 4 | 实现端到端测试引擎 | 7天 | 开发团队 |
| 5 | 创建 `crates/workflow` 模块 | 3天 | 架构师 |
| 6 | 实现 GitHub Actions 集成 | 5天 | 开发团队 |
| 7 | 实现 Jira 集成 | 5天 | 开发团队 |
| 8 | 实现 PR 自动创建 | 3天 | 开发团队 |
| 9 | 实现完整开发流水线 | 7天 | 架构师 |
| 10 | 集成到 TUI 命令系统 | 5天 | 开发团队 |
| 11 | 编写端到端测试 | 5天 | 测试团队 |
| 12 | 文档和用户指南 | 3天 | 文档团队 |

**总计**: 60天（约8周）

---

## 📊 总体时间线

```
Week 1-2:  [Phase 1] Web LLM 集成
Week 3-8:  [Phase 2] 开发工具指挥系统
Week 9-16: [Phase 3] 自动化测试平台 + 工作流对接

总计: 16周（约4个月）
```

---

## 🎯 关键里程碑

| 里程碑 | 时间 | 交付物 |
|--------|------|--------|
| M1: Web LLM 可用 | Week 2 | `/web-llm` 命令可用 |
| M2: VSCode 集成完成 | Week 5 | `/dev vs-code` 命令可用 |
| M3: 全工具支持完成 | Week 8 | VSCode + Qoder + Trae 全部可用 |
| M4: 自动化测试完成 | Week 12 | `/test auto` 命令可用 |
| M5: 工作流对接完成 | Week 14 | `/workflow pr create` 可用 |
| M6: 完整平台上线 | Week 16 | `/pipeline run` 全流程可用 |

---

## 💡 技术要点

### 1. 会话管理

- 每个开发工具连接维护独立会话
- 支持会话暂停、恢复、取消
- 实时推送进度到 TUI

### 2. 错误处理

- 开发工具连接失败自动重试
- 测试失败自动回滚代码
- 工作流触发失败降级处理

### 3. 安全性

- 开发工具指令需用户确认（Yolo 模式除外）
- 敏感操作（如删除、部署）需二次确认
- 所有操作记录审计日志

### 4. 性能优化

- 并行执行独立任务（如多个测试用例）
- 缓存 Web LLM 回答减少重复调用
- 使用连接池管理开发工具连接

---

## 📚 参考资料

- [Claude Code 架构分析](./architecture-analysis-for-deepseek-tui.md)
- [DeepSeek TUI 当前架构](../docs/ARCHITECTURE.md)
- [工具系统文档](../docs/TOOL_SURFACE.md)
- [Sub-Agent 文档](../docs/SUBAGENTS.md)

---

## ✅ 下一步行动

1. **立即开始**: 创建 `crates/web-llm` 模块（Phase 1）
2. **本周完成**: Web LLM HTTP 客户端原型
3. **下周完成**: LLM 智能路由器
4. **评审会议**: Week 2 结束时进行 Phase 1 评审

---

> **备注**: 本计划为初稿，需经团队评审后确定最终实施细节。每个 Phase 开始前需进行技术可行性验证。
