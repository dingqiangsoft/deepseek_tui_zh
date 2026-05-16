# DeepSeek TUI Startup Script
# Use Local LLM Service (Qwen3.5-9b-DeepSeek-V4-Flash)

# Configure Local LLM Service
$env:DEEPSEEK_PROVIDER = "openai"
$env:DEEPSEEK_BASE_URL = "http://192.168.2.5:1234/v1"
$env:DEEPSEEK_MODEL = "qwen3.5-9b-deepseek-v4-flash@q6_k"
$env:DEEPSEEK_API_KEY = "not-needed"  # Local service usually doesn't need API Key
$env:DEEPSEEK_ALLOW_INSECURE_HTTP = "1"  # Allow non-localhost HTTP connections

# 显示配置信息
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  DeepSeek TUI - 本地LLM启动" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Provider:    $env:DEEPSEEK_PROVIDER" -ForegroundColor Yellow
Write-Host "Base URL:    $env:DEEPSEEK_BASE_URL" -ForegroundColor Yellow
Write-Host "Model:       $env:DEEPSEEK_MODEL" -ForegroundColor Yellow
Write-Host "API Key:     $env:DEEPSEEK_API_KEY" -ForegroundColor Yellow
Write-Host ""
Write-Host "正在启动 DeepSeek TUI..." -ForegroundColor Green
Write-Host ""

# 启动DeepSeek TUI
# 注意：直接使用 deepseek 命令会继承上面设置的所有环境变量
deepseek
