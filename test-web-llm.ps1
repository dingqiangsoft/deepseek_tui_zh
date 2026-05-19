Write-Host "=== 启动 DeepSeek TUI (直接运行 deepseek-tui) ===" -ForegroundColor Green
Write-Host "请确保：" -ForegroundColor Yellow
Write-Host "  1. 浏览器已登录千问" -ForegroundColor Yellow
Write-Host "  2. 在 TUI 中输入：/web qianwen 你好，请介绍一下你自己" -ForegroundColor Yellow
Write-Host ""

& .\target\release\deepseek-tui.exe
