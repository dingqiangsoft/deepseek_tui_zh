# DeepSeek TUI 启动脚本（带日志保存）
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  DeepSeek TUI 中文版 - 启动（带日志）" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 设置环境变量
$env:DEEPSEEK_PROVIDER = "openai"
$env:DEEPSEEK_BASE_URL = "http://192.168.2.5:1234/v1"
$env:DEEPSEEK_MODEL = "qwen3.5-9b-deepseek-v4-flash@q6_k"
$env:DEEPSEEK_API_KEY = "not-needed"
$env:DEEPSEEK_ALLOW_INSECURE_HTTP = "1"

Write-Host "[1/2] 设置环境变量..." -ForegroundColor Green
Write-Host "  DEEPSEEK_BASE_URL = $env:DEEPSEEK_BASE_URL"
Write-Host "  DEEPSEEK_ALLOW_INSECURE_HTTP = $env:DEEPSEEK_ALLOW_INSECURE_HTTP"
Write-Host ""

Write-Host "[2/2] 启动程序（日志保存到 web_debug.log）..." -ForegroundColor Green
Write-Host ""

# 启动并保存日志
.\target\release\deepseek.exe 2>&1 | Tee-Object -FilePath web_debug.log

Write-Host ""
Write-Host "程序已退出。日志已保存到 web_debug.log" -ForegroundColor Yellow
Write-Host "按任意键关闭此窗口..."
Pause
