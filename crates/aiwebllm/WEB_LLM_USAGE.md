# Web LLM 集成 - 使用说明

## ✅ 已完成的功能

### 1. 统一的 Web LLM 客户端接口

文件: `crates/aiwebllm/src/web_llm_client.rs`

支持的平台:
- **千问** (Qianwen): `https://www.qianwen.com/`
- **豆包** (Doubao): `https://www.doubao.com/`

### 2. TUI 命令集成

命令: `/web <平台> "问题"`

示例:
```
/web qianwen "推荐一个AI训练数据工具"
/web doubao "解释量子计算"
/web 千问 "什么是机器学习"
/web 豆包 "如何学习Rust"
```

别名: `/wangluo`

## 📋 使用步骤

### 方法 1: 使用独立测试程序（当前可用）

#### 千问测试
```bash
cd crates/aiwebllm/qianwen-test
cargo run
```

#### 豆包测试
```bash
cd crates/aiwebllm/doubao-test
cargo run
```

### 方法 2: 使用 TUI 命令（需要修复 aiwebllm 编译错误后）

1. 启动 TUI
2. 输入命令: `/web qianwen "你的问题"`
3. 等待浏览器自动操作
4. 查看回复

## 🔧 当前状态

### ✅ 已完成
- [x] 千问网页自动化测试（成功获取 339 字符回复）
- [x] 豆包网页自动化测试（成功获取 1823 字符回复）
- [x] 统一的 WebLlmClient 接口
- [x] TUI 命令注册（/web）
- [x] 多语言支持（中/英/日）
- [x] 平台枚举和配置

### ⚠️ 待完成
- [ ] 修复 aiwebllm 模块的编译错误（session.rs 等旧代码）
- [ ] TUI 命令完整集成（等待 aiwebllm 编译通过）
- [ ] 异步回复机制（将浏览器获取的回复发送回 TUI 界面）

## 📊 测试结果

| 平台 | URL | 回复长度 | 状态 |
|------|-----|---------|------|
| 千问 | https://www.qianwen.com/ | 339 字符 | ✅ 成功 |
| 豆包 | https://www.doubao.com/ | 1823 字符 | ✅ 成功 |

## 🎯 下一步

1. **修复 aiwebllm 编译错误** - session.rs 中的类型不匹配和依赖问题
2. **完成 TUI 集成** - 使 `/web` 命令可以在 TUI 中直接使用
3. **优化用户体验** - 添加进度提示、超时处理等

## 💡 技术架构

```
用户输入: /web qianwen "问题"
    ↓
TUI 命令处理器 (commands/web.rs)
    ↓
WebLlmClient (web_llm_client.rs)
    ↓
headless_chrome (浏览器自动化)
    ↓
千问/豆包网页
    ↓
JavaScript DOM 提取
    ↓
返回回复给用户
```

## 📝 注意事项

1. **浏览器窗口**: 当前使用 `headless: false`，会显示浏览器窗口，方便调试
2. **登录状态**: 使用浏览器的 Cookie，首次使用需要手动登录
3. **超时设置**: 默认 60 秒超时
4. **依赖**: 需要系统安装 Chrome/Chromium 浏览器
