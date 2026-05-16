# DeepSeek TUI 启动脚本
# 使用DeepSeek官方API

# 配置DeepSeek官方API
$env:DEEPSEEK_PROVIDER = "deepseek"
$env:DEEPSEEK_BASE_URL = "https://api.deepseek.com/beta"
$env:DEEPSEEK_MODEL = "deepseek-v4-pro"
$env:DEEPSEEK_REASONING_EFFORT = "max"

# 显示配置信息
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  DeepSeek TUI - 官方API启动" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Provider:    $env:DEEPSEEK_PROVIDER" -ForegroundColor Yellow
Write-Host "Base URL:    $env:DEEPSEEK_BASE_URL" -ForegroundColor Yellow
Write-Host "Model:       $env:DEEPSEEK_MODEL" -ForegroundColor Yellow
Write-Host "推理深度:    $env:DEEPSEEK_REASONING_EFFORT" -ForegroundColor Yellow
Write-Host ""
Write-Host "注意: 需要预先设置 DEEPSEEK_API_KEY 环境变量" -ForegroundColor Red
Write-Host "或在首次运行时通过交互界面设置" -ForegroundColor Red
Write-Host ""
Write-Host "正在启动 DeepSeek TUI..." -ForegroundColor Green
Write-Host ""

# 启动DeepSeek TUI
deepseek --model auto
