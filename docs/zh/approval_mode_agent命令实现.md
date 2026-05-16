# `/config approval_mode agent` 命令实现说明

> **实现日期**: 2026-05-16  
> **功能版本**: v1.0  
> **适用版本**: v0.8.37+

---

## 🎯 功能说明

新增了 `/config approval_mode agent` 命令，该命令可以**同时切换 TUI 模式和批准模式**，解决用户在 Plan 模式下无法通过命令启用权限的问题。

---

## 📋 新增命令

### 命令格式

```bash
/config approval_mode <mode_name>
```

### 支持的参数

| 参数 | 切换到的 TUI 模式 | 设置的 approval_mode | 说明 |
|------|------------------|---------------------|------|
| `agent` | Agent | suggest | 开发模式（推荐） |
| `plan` | Plan | never | 只读模式 |
| `yolo` | YOLO | auto | 自由模式 |

---

## 🎮 使用示例

### 示例 1: 切换到 Agent 模式

```bash
/config approval_mode agent
```

**效果**：
- ✅ TUI 模式：Plan → Agent
- ✅ approval_mode：→ suggest
- ✅ exec_shell：可用（需确认）
- ✅ write_file：可用
- ✅ agent_open：可用（需确认）

**输出**：
```
Switched to agent mode (approval_mode = SUGGEST)
```

---

### 示例 2: 切换到 YOLO 模式

```bash
/config approval_mode yolo
```

**效果**：
- ✅ TUI 模式：→ YOLO
- ✅ approval_mode：→ auto
- ✅ 所有工具：自动执行

**输出**：
```
Switched to yolo mode (approval_mode = AUTO)
```

---

### 示例 3: 切换到 Plan 模式

```bash
/config approval_mode plan
```

**效果**：
- ✅ TUI 模式：→ Plan
- ✅ approval_mode：→ never
- ✅ 所有写入工具：禁用

**输出**：
```
Switched to plan mode (approval_mode = NEVER)
```

---

## 🔧 技术实现

### 代码位置

**文件**: `crates/tui/src/commands/config.rs`

**函数**: `set_config_value()`

### 实现逻辑

```rust
"approval_mode" | "approval" => {
    let value_lower = value.trim().to_ascii_lowercase();
    
    // 检查是否是模式名称（agent/plan/yolo）
    if matches!(value_lower.as_str(), "agent" | "plan" | "yolo") {
        // 1. 解析目标模式
        let target_mode = match value_lower.as_str() {
            "agent" => AppMode::Agent,
            "plan" => AppMode::Plan,
            "yolo" => AppMode::Yolo,
            _ => unreachable!(),
        };
        
        // 2. 切换模式
        app.set_mode(target_mode);
        
        // 3. 设置对应的 approval_mode
        let approval = match target_mode {
            AppMode::Yolo => ApprovalMode::Auto,
            AppMode::Agent => ApprovalMode::Suggest,
            AppMode::Plan => ApprovalMode::Never,
        };
        app.approval_mode = approval;
        
        // 4. 返回成功消息
        return CommandResult::with_message_and_action(
            format!("Switched to {} mode (approval_mode = {})", 
                    target_mode.as_setting(), 
                    approval.label()),
            AppAction::UpdateCompaction(app.compaction_config()),
        );
    }
    
    // 标准的 approval_mode 值处理（auto/suggest/never）
    // ...
}
```

---

## 📊 命令对比

### 原有命令（仍然可用）

```bash
# 只设置 approval_mode，不切换模式
/config approval_mode auto      # 自动批准
/config approval_mode suggest   # 建议批准
/config approval_mode never     # 从不执行
```

### 新增命令

```bash
# 同时切换模式和设置 approval_mode
/config approval_mode agent     # 切换到 Agent + suggest
/config approval_mode yolo      # 切换到 YOLO + auto
/config approval_mode plan      # 切换到 Plan + never
```

---

## 🎯 使用场景

### 场景 1: AI 在 Plan 模式下无法执行工具

**问题**：
```
Failed to authorize tool execution: Tool 'exec_shell' is unavailable in Plan mode
```

**解决**：
```bash
/config approval_mode agent
```

**效果**：立即切换到 Agent 模式，AI 可以执行工具了！

---

### 场景 2: 快速切换到开发模式

**之前**（需要两步）：
```bash
/mode agent
/config approval_mode auto
```

**现在**（只需一步）：
```bash
/config approval_mode agent
```

---

### 场景 3: 快速切换到 YOLO 模式

**之前**：
```bash
/mode yolo
```

**现在**（统一命令格式）：
```bash
/config approval_mode yolo
```

---

## ✅ 测试用例

### 测试 1: approval_mode agent 切换

```rust
#[test]
fn test_set_approval_mode_switches_tui_mode() {
    let mut app = create_test_app();
    app.set_mode(AppMode::Plan); // 先设置为 Plan
    
    let result = set_config(&mut app, Some("approval_mode agent"));
    assert!(result.message.is_some());
    let msg = result.message.unwrap();
    assert!(msg.contains("Switched to agent mode"));
    assert_eq!(app.mode, AppMode::Agent);
    assert_eq!(app.approval_mode, ApprovalMode::Suggest);
}
```

### 测试 2: approval_mode yolo 切换

```rust
let result = set_config(&mut app, Some("approval_mode yolo"));
assert_eq!(app.mode, AppMode::Yolo);
assert_eq!(app.approval_mode, ApprovalMode::Auto);
```

### 测试 3: approval_mode plan 切换

```rust
let result = set_config(&mut app, Some("approval_mode plan"));
assert_eq!(app.mode, AppMode::Plan);
assert_eq!(app.approval_mode, ApprovalMode::Never);
```

---

## 📝 错误处理

### 无效值

```bash
/config approval_mode invalid
```

**输出**：
```
Invalid approval_mode. Use: auto, suggest/on-request/untrusted, never/deny, or agent/plan/yolo to switch modes
```

---

## 🔗 相关文档

- [权限配置指南](zh/权限配置指南.md) - 完整权限配置说明
- [模式和命令指南](zh/模式和命令指南.md) - 模式切换详细说明
- [QUICK_COMMANDS.md](QUICK_COMMANDS.md) - 快速命令参考卡
- [MODES.md](MODES.md) - 英文模式文档

---

## 🎉 总结

这个新功能解决了**最常见的问题**：用户在 Plan 模式下无法通过命令启用权限。

**一句话**：
```bash
/config approval_mode agent  # 一键启用所有权限！
```

---

> **实现者**: AI Assistant  
> **审核状态**: 待审核  
> **兼容性**: 向后兼容，不影响现有命令
