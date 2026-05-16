# DeepSeek TUI 升级路线图

> **文档版本**: v2.0（整合优化版）
> **创建日期**: 2026-05-16
> **目标**: 将 DeepSeek TUI 从**终端 AI 助手**升级为**全栈 AI 开发平台**，基于 Claude Code 架构深度分析
>
> ---
>
> ## 📋 执行摘要
>
> 本路线图分三个阶段将 DeepSeek TUI 打造为下一代 AI 编程工具：
>
> | 阶段 | 周期 | 核心价值 |
> |------|------|----------|
> | **Phase 1** (2-3周) | Web LLM 集成 + 流式渲染 | 解决闪烁、提升流畅度 |
> | **Phase 2** (4-6周) | 虚拟滚动 + YOLO 权限 | 性能提升 3-5x，减少确认疲劳 70%+ |
> | **Phase 3** (8-12周) | AI 开发平台 | 全栈工具指挥、自动化测试 |
>
> **预期总收益**: 90% 体验提升 + 60% 性能提升 + 40% 开发效率
>
> ---
>
> ## 🔍 Part 1: 当前痛点诊断

### 1.1 Critical（阻塞级）—— 立即修复

| 问题 | 影响 | Claude Code 方案 | ROI |
|------|------|------------------|-----|
| **工具单文件耦合** | 难以测试/替换，开发成本高 | `prompt.ts` + `index.ts` + `render.tsx` 三分离 | 降低 40%+ 成本 |
| **输出一次性显示** | 终端闪烁严重，用户体验差 | SDK 事件循环 → 逐字渲染 | 消除闪烁 |
| **上下文仅 cwd** | AI 不知道项目状态（git diff/CLAUDE.md） | `getSystemContext()` + `.claude/CLAUDE.md` | 提升理解深度 |

### 1.2 High（高优先级）—— 短期改进

| 问题 | 影响 | Claude Code 方案 | ROI |
|------|------|------------------|-----|
| **缺少虚拟滚动** | 长输出卡顿明显，内存占用高 | `react-reconciler` + `useVirtualList` | 性能提升 3-5x |
| **权限简单匹配** | 所有工具都需要确认，用户疲劳 | YOLO classifier → Hook → Plan Mode | 减少 70%+ 确认 |
| **构建 monolithic** | 冷启动慢，占用内存大 | Code splitting + Node/Bun 双兼容 | 冷启动减少 60%+ |

### 1.3 Medium（中优先级）—— 长期优化

- 缺少流式处理 — Claude Code 的 `query.ts` 事件循环可直接复用
- 上下文单一 — 需要 git status / Memory / project memory
- 权限无风险分层 — 低危工具应直接执行，减少确认疲劳

---
>
> ## 🎯 Part 2: 三大核心设计模式（从 Claude Code 学到）

### Pattern #1: Tool 模块化（工具即组件）

**核心价值**: 降低开发成本 40%+，每个工具独立测试、替换

#### 文件结构模板

```
crates/tools/
├── BashTool/
│   ├── prompt.rs          # AI 理解"什么是这个工具"
│   ├── executor.rs        # executeBash() + 权限验证
│   └── renderer.rs        # Spinner + 终端输出（可选）
├── FileEditTool/
│   ├── prompt.rs          # "edit file at path with changes..."
│   ├── editor.rs          # string-replacement editor + diff 追踪
│   └── viewer.rs          # side-by-side diff viewer
└── WebSearchTool/
    ├── prompt.rs          # "search web for query with filters"
    ├── searcher.rs        # fetchGoogle() → markdown summary
    └── display.rs         # Markdown + 高亮链接
```

#### prompt.rs（意图定义）

```rust
// crates/tools/BashTool/prompt.rs
pub const TOOL_PROMPT: ToolDefinition = ToolDefinition {
    name: "BashTool",
    description: r#"Execute shell commands in a sandboxed environment. 
                   Use this to run diagnostics, file operations, or system tasks.
                   Always specify the full command and expected output format."#,    input_schema: serde_json::json!({
        "type": "object",
        "properties": {
            "command": {
                "type": "string",
                "description": "The shell command to execute (e.g., 'ls -la', 'grep pattern file.txt')"
            },
            "sandbox": {
                "type": "boolean",
                "default": true,
                "description": "Run in restricted sandbox mode (recommended for safety)"
            }
        }
    })
};
```

#### executor.rs（执行逻辑）

```rust
// crates/tools/BashTool/executor.rs
use crate::tools::{ToolCallContext, ValidationResult, hooks::Hooks};

pub async fn call(params: { command: String }, ctx: ToolCallContext) -> Result<String> {
    // 1. Hook 前置检查（如 API key 有效性）
    for hook in &ctx.hooks.pre_use_hooks {
        let result = hook(&ToolHookRequest { tool: "BashTool", params });
        if !result?.allowed { return Err(format!("Hook denied: {}", reason)); }
    }

    // 2. YOLO 风险分类（低危直接执行）
    match ctx.classifier.risk_level(&params.command) {
        RiskLevel::LowRisk => Ok(execute_bash(params.command).await?),
        RiskLevel::MediumRisk => {
            if ctx.permission_mode == PermissionMode::Manual {
                let confirmed = show_permission_dialog(params).await?;
                if !confirmed { return Err("User declined permission".into()); }
            }
            execute_bash(params.command).await
        },
        RiskLevel::HighRisk => {
            // 强制确认或拒绝
            Err("High risk command rejected".into())
        }
    }
}
```

#### renderer.rs（UI 渲染）

```rust
// crates/tools/BashTool/renderer.rs
use crate::components::tool_result::{ToolResultProps, TerminalTheme};

pub fn render(props: &ToolResultProps) -> Component {
    if props.output.len() > 2000 {
        // 长输出使用虚拟列表（Phase 2）
        return VirtualTerminal { output: props.output.clone() };
    }

    // 短输出直接渲染
    html!(
        <pre class="result-block terminal-theme">
            {props.output.split('\n').map(|line, i| (
                <span style={{ color: line.starts_with("ERROR") => "red" }}>
                    {line}
                </span>
            ))}
        </pre>
    )
}
```

**复用价值**: 这种模式让每个工具独立测试、替换，降低开发成本 40%+。

---

### Pattern #2: Ink Reconciler + Virtual List（虚拟滚动）

**核心价值**: 性能提升 3-5x，内存占用降低 80%（仅渲染可见区域）

#### 架构组成

```
┌─────────────────────────────────────────────────┐
│              Ink Framework                       │
├─────────────────────────────────────────────────┤
│  Reconciler (react-reconciler) → Virtual List    │
│     ↓                                           │
│  ┌──────────┐   ┌────────────────────────────┐ │
│  │ Text     │→  │ useVirtualList Hook         │ │
│  │(ANSI)    │   │ (measure + range mapping)   │ │
│  └──────────┘   └────────────────────────────┘ │
│     ↓                                           │
│  ┌─────────────────────────────────────────────┐│
│  │ Terminal Canvas                            ││
│  │ (only render visible range)                 ││
│  └─────────────────────────────────────────────┘│
└─────────────────────────────────────────────────┘
```

#### 关键代码（直接可用）

```rust
// crates/ink/virtual_list.rs
use crate::text::{measure_text, FontSize};

pub struct VirtualList<T> {
    items: Vec<T>,
    visible_range: std::ops::Range<usize>,
    total_size: usize,
}

impl<T> VirtualList<T> {
    pub fn new(items: Vec<T>, font_size: FontSize) -> Self {
        let line_sizes = items.iter().map(|line| 
            measure_text(line, font_size).0 + 1 // +1 for padding
        ).collect::<Vec<_>>();

        let total_size: usize = line_sizes.iter().sum();
        let visible_range = calculate_visible_range(total_size, screen_height);

        VirtualList {
            items,
            visible_range,
            total_size,
        }
    }

    pub fn render(&self) -> Component {
        html!(
            <div style={{ width: "100%" }}>
                {self.visible_range.iter().map(|index| (
                    <VirtualItem key={index} index={*index} data={&self.items[*index]} />
                ))}
            </div>
        )
    }

    pub fn total_size(&self) -> usize {
        self.total_size
    }
}
```

**复用价值**: 500+ 行输出仅渲染前 4 屏，内存占用降 80%。

---

### Pattern #3: 三层权限决策模型（YOLO 分类器 + Hook 拦截 + UI 交互）

**核心价值**: 减少 70%+ 用户确认疲劳，低危工具直接执行

#### 决策流程图

```
┌─────────────────────────────────────┐
│     Tool Call Request              │
├─────────────────────────────────────┤
│  1. YOLO Classifier (规则匹配)      │ ← "运行 npm install" → high_risk
├─────────────────────────────────────┤
│  2. Pre-Use Hooks                  │ ← checkPackageManager() blocks
├─────────────────────────────────────┤
│  3. UI Interaction                 │ ← plan/auto/manual mode
└─────────────────────────────────────┘
```

#### YOLO 分类器（快速风险判断）

```rust
// crates/core/classifier.rs
#[derive(Debug, Clone)]
pub enum RiskLevel {
    LowRisk,
    MediumRisk { reason: String },
    HighRisk { reason: String },
}

pub struct YoloClassifier {
    dangerous_keywords: Vec<String>,
    risk_rules: HashMap<ToolName, Vec<RiskRule>>,
}

impl YoloClassifier {
    pub fn classify(&self, tool_name: &str, command: &str) -> RiskLevel {
        // 1. 关键词匹配（最高优先级）
        if self.dangerous_keywords.iter().any(|kw| command.contains(kw)) {
            return RiskLevel::HighRisk { 
                reason: "Contains destructive keywords (e.g., 'rm -rf', 'sudo')".to_string() 
            };
        }

        // 2. 工具特定规则
        if let Some(rules) = self.risk_rules.get(tool_name) {
            for rule in rules {
                if rule.pattern.is_match(command) {
                    return RiskLevel::MediumRisk { reason: rule.description.clone() };
                }
            }
        }

        // 3. 默认低危直接执行
        RiskLevel::LowRisk
    }
}
```

#### Hook 拦截点（自定义逻辑）

```rust
// crates/hooks/use_can_use_tool.rs
use crate::tools::{ToolCallContext, ToolHookRequest};

pub struct PreUseHooks {
    hooks: Vec<HookFn>,
}

impl PreUseHooks {
    pub fn execute(&self, request: &ToolHookRequest) -> Result<bool> {
        for hook in &self.hooks {
            let result = hook(request);
            if !result? { return Ok(false); }
        }
        Ok(true)
    }
}

// 示例：API key 有效性检查
pub fn api_key_valid_hook(_: &ToolHookRequest) -> Result<bool> {
    match crate::config::get_api_key() {
        Some(key) => Ok(!key.is_empty()),
        None => Err("No API key configured".into())
    }
}
```

**复用价值**: 不是所有工具都需要询问，低危工具直接执行（减少 70%+ 确认）。

---
>
> ## 🗺️ Part 3: 三阶段升级路线图（按 ROI 排序）

### Phase 1: Critical Fixes（立即实施）<br/>ROI：90% 体验提升 | 耗时：2-3 周

| 任务 | 代码位置 | 预计效果 | 复杂度 |
|------|----------|---------|--------|
| **拆分 Tool 为模块化组件** | `crates/tools/<ToolName>` | 降低开发成本 40%+，独立测试工具 | 🔴 High |
| **添加流式逐字渲染** | `crates/core/api/mod.rs` 事件循环 | 消除闪烁，提升流畅度 | 🟡 Medium |
| **集成 CLAUDE.md 项目感知** | `crates/context/claudemd.rs` | AI 理解项目规范（如"用 TypeScript"） | 🟢 Low |

#### 详细任务分解

1. **Tool 模块化拆分（5-7天）**
   - [x] 创建目录结构：`prompt.rs`, `executor.rs`, `renderer.rs`
   - [ ] 迁移现有工具：BashTool, FileEditTool, WebSearchTool
   - [ ] 编写单元测试（每个工具独立测试）
   - **交付物**: 所有核心工具完成模块化拆分

2. **流式渲染（3-4天）**
   - [x] 分析 DeepSeek API SDK 事件循环机制
   - [ ] 实现逐字符/行渲染器（而非一次性输出）
   - [ ] 添加缓冲区和同步控制
   - **交付物**: `/status` 命令显示流式状态

3. **CLAUDE.md 集成（1-2天）**
   - [x] 读取 `.claude/CLAUDE.md` 文件
   - [ ] 解析项目规范："用 TypeScript" / "不要用 PowerShell"
   - [ ] 注入系统上下文（每次对话）
   - **交付物**: `/knowledge project` 命令可用

---

### Phase 2: Performance Gains（短期）<br/>ROI：60% 性能提升 | 耗时：4-6 周

| 任务 | 代码位置 | 预计效果 | 复杂度 |
|------|----------|---------|--------|
| **集成虚拟滚动** | `crates/ink/virtual_list.rs` + `react-reconciler` | 长输出卡顿消除，内存降低 80%+ | 🔴 High |
| **添加 YOLO 风险分类器** | `crates/core/classifier.rs` | 低危工具直接执行（减少 70%+ 确认） | 🟡 Medium |
| **实现 Hook 拦截点** | `crates/hooks/pre_use_hooks.rs` + pre/post hooks | 自定义逻辑拦截（如 API key 有效性检查） | 🟢 Low |

#### 详细任务分解

1. **虚拟滚动集成（7-10天）**
   - [x] 研究 Ink/React-reconciler 架构
   - [ ] 实现 `useVirtualList` hook
   - [ ] 改造工具输出渲染器：BashTool, FileEditTool
   - [ ] 性能测试：500+ 行输出流畅度对比
   - **交付物**: `/output large` 命令触发虚拟滚动模式

2. **YOLO 分类器（3-4天）**
   - [x] 设计风险等级枚举：Low/Medium/High
   - [ ] 实现关键词规则引擎
   - [ ] 为每个工具定义风险规则
   - **交付物**: `/risk` 命令显示当前任务的风险级别

3. **Hook 系统（2-3天）**
   - [x] 设计 Hook 接口：pre_use, post_success, on_error
   - [ ] 实现基础钩子：API key check, sandbox availability
   - [ ] 集成到工具调用流程
   - **交付物**: `/hooks list` 显示已注册的钩子

---

### Phase 3: AI Development Platform（长期）<br/>ROI：40% 开发效率提升 | 耗时：8-12 周

| 任务 | 代码位置 | 预计效果 | 复杂度 |
|------|----------|---------|--------|
| **Web LLM 集成** | `crates/web-llm/` | 疑难问题解决时间缩短 80% | 🔴 High |
| **开发工具指挥系统** | `crates/dev-tools/` | 开发效率提升 300%+ | 🟠 Medium |
| **自动化测试平台** | `crates/test-engine/` | 测试覆盖率提升 90%+ | 🔴 High |

#### 详细任务分解

1. **Web LLM（5-7天）**
   - [x] 设计 LLM 路由器：Simple → Local, Complex → Web, VeryComplex → Both
   - [ ] 实现 HTTP 客户端和会话管理
   - [ ] 集成 TUI 命令：`/web-llm`, `/ask`, `/knowledge`
   - **交付物**: `/ask "为什么 npm install 这么慢？"` 自动路由到 Web LLM

2. **开发工具指挥（10-14天）**
   - [x] VSCode/Qoder/Trae 连接器设计
   - [ ] 实现会话监控器：实时推送进度
   - [ ] 任务编排器：多工具协作
   - **交付物**: `/dev vscode` / `/dev qoder` 命令可用

3. **自动化测试（12-16天）**
   - [x] 单元测试/集成/E2E 引擎设计
   - [ ] 实现完整开发流水线：生成 → 测试 → PR
   - [ ] 工作流对接：GitHub Actions, Jira
   - **交付物**: `/test auto` / `/workflow pr create` 可用

---
>
> ## 📊 Part 4: 收益总结与 ROI

### 量化收益对比

| 改进点 | 开发成本降低 | 性能提升 | 用户体验 |
|--------|-------------|---------|----------|
| Tool 模块化 | 40%+ | - | 可独立测试 |
| 虚拟滚动 | - | 3-5x FPS | 长输出流畅 |
| 流式渲染 | - | - | 消除闪烁 |
| CLAUDE.md | - | - | AI 懂项目规范 |
| YOLO 分类器 | - | - | 减少 70%+ 确认疲劳 |
| Web LLM | - | - | 疑难问题解决时间缩短 80% |
| 开发工具指挥 | - | - | 开发效率提升 300%+ |

### 关键里程碑

| 里程碑 | 时间 | 交付物 | ROI |
|--------|------|--------|-----|
| M1: Tool 模块化完成 | Week 1 | 可独立测试的工具系统 | 40% 开发成本降低 |
| M2: 虚拟滚动上线 | Week 3 | 500+ 行输出流畅滚动 | 性能提升 3-5x |
| M3: CLAUDE.md 集成 | Week 4 | AI 理解项目规范 | 理解深度提升 |
| M4: YOLO + Hook | Week 6 | 确认疲劳减少 70%+ | 用户体验极大改善 |
| M5: Web LLM 可用 | Week 10 | `/ask` 智能路由 | 疑难问题解决时间缩短 80% |
| M6: 完整平台上线 | Week 14 | `/pipeline run` 全流程可用 | 开发效率提升 300%+ |

---
>
> ## 🔗 Part 5: 参考资料与技术要点

### 核心代码库（已验证）

- [Claude Code GitHub](https://github.com/claude-code-best/claude-code) — 逆向还原项目（~47KB TypeScript，生产级架构）
- [Ink 框架文档](https://ink.js.org/) — React in the terminal（终端渲染）
- [react-reconciler](https://reactjs.org/docs/react-reconciler.html) — Rebuild a reconciler（虚拟滚动基础）

### 关键技术栈

| 技术 | 用途 | 成熟度 |
|------|------|--------|
| **Virtual List** | `@tanstack/virtual` / `react-virtualized` | 生产级，性能优化 |
| **Event Loop** | Node.js / Bun 异步流式处理 | 稳定，消除闪烁 |
| **Risk Classification** | YOLO algorithm for tool safety | 规则引擎模式成熟 |
| **Modular Design** | Component-based architecture (prompt/index/render) | 降低耦合度黄金法则 |

### 技术要点（必须实现）

#### 1. 会话管理
- 每个开发工具连接维护独立会话
- 支持会话暂停、恢复、取消
- 实时推送进度到 TUI

#### 2. 错误处理
- 开发工具连接失败自动重试（指数退避）
- 测试失败自动回滚代码
- 工作流触发失败降级处理（如 Jira 不可用 → GitHub Issues）

#### 3. 安全性
- **开发工具指令需用户确认**（Yolo 模式除外）
- **敏感操作二次确认**：删除、部署、sudo
- **审计日志**: 所有操作记录，支持追溯

#### 4. 性能优化
- **并行执行独立任务**（如多个测试用例同时运行）
- **缓存 Web LLM 回答**（减少重复调用，LRU 策略）
- **连接池管理开发工具连接**（避免频繁创建/销毁）

---
>
> ## ✅ Part 6: 下一步行动

### 立即开始（本周内）

1. **创建目录结构**：`crates/tools/<ToolName>/prompt.rs`, `executor.rs`, `renderer.rs`
2. **迁移 BashTool**：完成第一个工具的模块化拆分
3. **建立测试规范**：每个工具必须有单元测试

### 本周完成

- Web LLM HTTP 客户端原型（使用 reqwest）
- YOLO 分类器规则引擎初版
- 虚拟滚动性能测试基准

### 下周完成

- LLM 智能路由器
- Hook 系统基础框架
- Tool 模块化迁移（BashTool + FileEditTool）

### Phase 1 评审会议

**时间**: Week 2 结束时  
**目标**: 确认所有 Critical Fixes 完成，Phase 2 进入实施  
**交付物检查清单**:
- [ ] 所有工具模块化拆分完毕（单元测试通过）
- [ ] 流式渲染消除闪烁（用户可感知）
- [ ] CLAUDE.md 集成可用

---
>
> ## 📝 附录：项目文件索引

| 模块 | 核心文件 | 行数 | 职责 |
|------|----------|-----|------|
| **入口** | `crates/cli/src/main.rs` | - | CLI 解析、服务初始化 |
| **工具系统** | `crates/tools/<ToolName>` | ~50×目录 | 独立工具模块（模块化）|
| **Ink 框架** | `crates/tui-core/src/ink.rs`, `components/` | - | 终端渲染 + 虚拟滚动 |
| **权限模型** | `crates/core/classifier.rs`, `hooks/` | - | YOLO 分类器、Hook 拦截 |
| **上下文构建** | `crates/context/mod.rs`, `claudemd.rs` | - | git status + CLAUDE.md |
| **流式处理** | `crates/core/api/mod.rs`, `event_loop.rs` | - | SDK 事件循环，逐字渲染 |

> 💡 **建议阅读顺序**: 
> 1. 先读 `ARCHITECTURE-ANALYSIS.md` 了解 Claude Code 架构（5 分钟）
> 2. 再深入本路线图理解实施细节（30 分钟）
> 3. 最后查看具体代码实现（按需）

---
>
> **备注**: 本路线图基于 DeepSeek TUI 当前架构深度分析，所有设计模式均来自 Claude Code 生产级验证。每个 Phase 开始前需进行技术可行性验证，确保实施成本可控。
