# Cookie 配置指南

## 如何获取 Web LLM 的 Cookie

### 通义千问 Cookie 获取

1. **登录通义千问**
   - 打开浏览器访问：https://www.qianwen.com
   - 使用账号登录

2. **打开开发者工具**
   - 按 `F12` 或右键点击"检查"
   - 切换到"网络"(Network)标签

3. **捕获请求**
   - 刷新页面
   - 在网络请求列表中找到任意一个请求
   - 点击该请求

4. **复制 Cookie**
   - 在请求头(Headers)中找到 `Cookie` 字段
   - 复制完整的 Cookie 值

5. **保存为 JSON 文件**
   - 创建文件：`~/.deepseek/cookies/qianwen.json`
   - 格式如下：

```json
{
  "cookies": [
    {
      "name": "session_id",
      "value": "你的session值",
      "domain": ".qianwen.com",
      "path": "/",
      "secure": true,
      "httpOnly": true
    },
    {
      "name": "csrf_token",
      "value": "你的token值",
      "domain": ".qianwen.com",
      "path": "/"
    }
  ],
  "last_updated": "2026-05-16T17:00:00Z"
}
```

### 豆包 Cookie 获取

1. **登录豆包**
   - 打开浏览器访问：https://www.doubao.com
   - 使用账号登录

2. **打开开发者工具**
   - 按 `F12` 或右键点击"检查"
   - 切换到"网络"(Network)标签

3. **捕获请求**
   - 刷新页面
   - 找到任意一个请求
   - 点击该请求

4. **复制 Cookie**
   - 在请求头(Headers)中找到 `Cookie` 字段
   - 复制完整的 Cookie 值

5. **保存为 JSON 文件**
   - 创建文件：`~/.deepseek/cookies/doubao.json`
   - 格式与通义千问相同

---

## 使用浏览器扩展获取 Cookie（推荐）

### Chrome/Edge

1. 安装扩展：**EditThisCookie** 或 **Cookie-Editor**
2. 登录 Web LLM 网站
3. 点击扩展图标
4. 点击"导出"按钮
5. 将导出的 JSON 保存到对应文件

### Firefox

1. 安装扩展：**Cookies Manager+**
2. 登录 Web LLM 网站
3. 打开扩展
4. 导出 Cookie
5. 保存到对应文件

---

## Cookie 存储路径

```
~/.deepseek/cookies/
├── qianwen.json    # 通义千问 Cookie
└── doubao.json     # 豆包 Cookie
```

### Windows 路径示例
```
C:\Users\你的用户名\.deepseek\cookies\qianwen.json
C:\Users\你的用户名\.deepseek\cookies\doubao.json
```

### Linux/macOS 路径示例
```
~/.deepseek/cookies/qianwen.json
~/.deepseek/cookies/doubao.json
```

---

## Cookie 更新频率

- **通义千问**：Cookie 有效期约 7-30 天，过期后需要重新获取
- **豆包**：Cookie 有效期约 7-15 天，过期后需要重新获取

**提示**：当收到认证失败错误时，通常需要更新 Cookie。

---

## 安全注意事项

⚠️ **重要**：
- Cookie 文件包含敏感信息，请勿分享给他人
- 不要将 Cookie 文件提交到 Git 仓库
- 定期检查 Cookie 文件权限（建议设置为仅所有者可读）

```bash
# Linux/macOS 设置文件权限
chmod 600 ~/.deepseek/cookies/*.json

# Windows 右键文件 → 属性 → 安全 → 编辑权限
```

---

## 测试 Cookie 是否有效

使用以下命令测试：

```bash
# 通义千问
/web qianwen "你好，请回复测试成功"

# 豆包
/web doubao "你好，请回复测试成功"
```

如果收到正常回复，说明 Cookie 有效。
