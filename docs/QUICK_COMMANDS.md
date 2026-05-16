# DeepSeek TUI 快速命令参考卡

> 打印此页贴在桌边！📌

---

## 🎮 模式切换

### 快捷键
```
Tab         → 切换模式 (Plan → Agent → YOLO)
Shift+Tab   → 切换推理强度 (off → high → max)
```

### Slash 命令
```bash
/mode plan      # 计划模式（只读）
/mode agent     # 代理模式（默认）
/mode yolo      # 自由模式（自动批准）

/mode 1         # = Plan
/mode 2         # = Agent
/mode 3         # = YOLO
```

---

## 🔒 批准模式

```bash
# 查看当前配置
/config

# 设置批准模式
/config approval_mode auto      # 自动批准所有工具
/config approval_mode suggest   # 建议批准（默认）
/config approval_mode never     # 阻止需要批准的工具

# 别名（等价命令）
/config approval_mode on-request    # = suggest
/config approval_mode deny          # = never
```

---

## 🌐 语言设置

```bash
/config locale zh-Hans      # 简体中文
/config locale en           # English
/config locale ja           # 日本語
```

---

## 🎨 主题设置

```bash
/config theme dark          # 暗色主题
/config theme light         # 亮色主题
/config theme auto          # 跟随系统
```

---

## 🔧 常用命令

```bash
# 会话管理
/sessions                   # 查看会话列表
/resume <id>                # 恢复会话

# 上下文管理
/compact                    # 压缩上下文（推荐 60% 时使用）

# 帮助
/help                       # 查看帮助
F1 或 Ctrl-/                # 快捷键帮助

# 其他
/trust                      # 切换信任模式
/queue                      # 查看队列
```

---

## 🎯 快速配置场景

### 场景 1: 日常开发（推荐新手）
```bash
/mode agent
/config approval_mode suggest
```

### 场景 2: 高效开发
```bash
/mode agent
/config approval_mode auto
```

### 场景 3: 个人项目
```bash
/mode yolo
# 自动启用所有权限
```

### 场景 4: 只读调查
```bash
/mode plan
# 自动禁用所有写入工具
```

---

## ⌨️ 编辑器快捷键

```bash
# 基础编辑
Enter           → 发送消息
Alt-Enter       → 插入换行
Ctrl-U          → 删除到行首
Ctrl-W          → 删除前一个词
Ctrl-A          → 跳到行首
Ctrl-E          → 跳到行尾

# 历史
↑ / ↓           → 切换历史消息
Ctrl-P / Ctrl-N → 切换历史（替代）

# 补全
Tab             → 补全命令/@mention
Ctrl-K          → 命令面板

# 其他
Ctrl-O          → 查看详情
Ctrl-R          → 恢复会话
Ctrl-L          → 清屏
Ctrl-S          → 暂存草稿
```

---

## ⚠️ 重要提示

1. **YOLO 模式危险**：自动执行所有操作！
2. **使用 `/config`**：`/set` 命令已废弃
3. **定期 `/compact`**：防止上下文溢出
4. **Esc 是取消**：不是模式切换

---

## 📚 完整文档

- [模式和命令指南](zh/模式和命令指南.md)
- [快捷键说明](KEYBINDINGS.md)
- [配置参考](CONFIGURATION.md)

---

**最后更新**: 2026-05-16 | **版本**: v0.8.37+
