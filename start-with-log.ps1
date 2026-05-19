# DeepSeek TUI Launcher (with log)
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  DeepSeek TUI - Start with Log" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Set environment variables
$env:DEEPSEEK_PROVIDER = "openai"
$env:DEEPSEEK_BASE_URL = "http://192.168.2.5:1234/v1"
$env:DEEPSEEK_MODEL = "qwen3.5-9b-deepseek-v4-flash@q6_k"
$env:DEEPSEEK_API_KEY = "not-needed"
$env:DEEPSEEK_ALLOW_INSECURE_HTTP = "1"

Write-Host "[1/2] Setting environment variables..." -ForegroundColor Green
Write-Host "  DEEPSEEK_BASE_URL = $env:DEEPSEEK_BASE_URL"
Write-Host "  DEEPSEEK_ALLOW_INSECURE_HTTP = $env:DEEPSEEK_ALLOW_INSECURE_HTTP"
Write-Host ""

Write-Host "[2/2] Starting program..." -ForegroundColor Green
Write-Host "  Note: Logs are NOT saved to file (TUI requires direct terminal)" -ForegroundColor Yellow
Write-Host "  Debug logs will show in this window" -ForegroundColor Yellow
Write-Host ""

# Start directly (no pipe, TUI needs direct terminal)
.\target\release\deepseek.exe

Write-Host ""
Write-Host "Program exited." -ForegroundColor Yellow
Write-Host "Press any key to close..."
Pause
