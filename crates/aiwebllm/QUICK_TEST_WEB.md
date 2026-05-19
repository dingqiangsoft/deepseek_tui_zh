# 🚀 快速测试 /web 命令

## ✅ 编译状态

项目已经成功编译！`/web` 命令已经注册到 TUI 中。

---

## 📝 如何使用 /web 命令

### 步骤 1: 启动 TUI

```powershell
# 使用已编译的版本
./target/release/deepseek

# 或者从源码运行
cargo run --bin deepseek
```

### 步骤 2: 在 TUI 中输入命令

进入 TUI 界面后，在底部的命令输入框中输入：

```
/web qianwen "推荐一个ai训练开源工具"
```

或者

```
/web doubao "解释一下什么是人工智能"
```

### 步骤 3: 查看帮助

在 TUI 中输入：

```
/web --help
```

或者查看所有命令：

```
/help
```

你应该能看到 `/web` 命令在列表中。

---

## 🎯 命令格式

### 基本用法

```
/web <平台> "问题"
```

### 支持的平台

- **qianwen** (千问): 通义千问
- **doubao** (豆包): 字节跳动豆包

### 示例

```
# 查询千问
/web qianwen "推荐一个AI训练数据处理工具"

# 查询豆包
/web doubao "解释量子计算的基本原理"

# 使用中文平台名
/web 千问 "什么是机器学习"
```

---

## 🔧 故障排除

### 问题 1: "Unknown command: /web"

**原因**: TUI 没有使用最新编译的版本

**解决**:
```powershell
# 重新编译
cargo build --release

# 确保使用正确的二进制文件
./target/release/deepseek
```

### 问题 2: 命令没有响应

**原因**: aiwebllm 模块可能没有正确初始化

**解决**:
1. 检查配置文件是否存在
2. 查看日志输出

```powershell
# 使用 debug 模式运行
RUST_LOG=debug ./target/release/deepseek
```

### 问题 3: 需要配置 Cookie 或 API Key

**千问 API 方式**:
```powershell
$env:QIANWEN_API_KEY = "你的API密钥"
./target/release/deepseek
```

**千问网页方式**:
- 确保已登录 https://www.qianwen.com/
- Cookie 会自动提取

**豆包网页方式**:
- 需要先登录 https://www.doubao.com/
- 手动配置 Cookie 到配置文件

---

## 📊 测试清单

- [ ] TUI 成功启动
- [ ] `/help` 命令显示 `/web` 在列表中
- [ ] `/web qianwen "测试"` 成功发送请求
- [ ] 收到 AI 回复
- [ ] 回复内容正确显示

---

## 💡 提示

1. **首次使用**: 建议先用千问 API 测试（更稳定）
2. **网页模式**: 需要确保浏览器已登录对应平台
3. **查看详细日志**: 使用 `RUST_LOG=debug` 环境变量
4. **命令别名**: `/web` 也可以使用 `/wangluo`

---

## 📚 相关文档

- [TEST_COMMANDS.md](./crates/aiwebllm/TEST_COMMANDS.md) - 完整测试命令参考
- [TEST_GUIDE.md](./crates/aiwebllm/TEST_GUIDE.md) - 测试指南
- [WEB_LLM_USAGE.md](./crates/aiwebllm/WEB_LLM_USAGE.md) - Web LLM 使用指南

---

**准备好测试了吗？** 运行 `./target/release/deepseek` 开始吧！🎉
