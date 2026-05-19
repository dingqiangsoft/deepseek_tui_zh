# DeepSeek TUI Debug Launcher
# This script redirects stderr to a file for debugging

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  DeepSeek TUI - DEBUG MODE" -ForegroundColor Cyan
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

Write-Host "[2/2] Starting in DEBUG mode..." -ForegroundColor Green
Write-Host "  Debug logs will be saved to: debug.log" -ForegroundColor Yellow
Write-Host "  Open another PowerShell and run:" -ForegroundColor Yellow
Write-Host "  Get-Content debug.log -Wait" -ForegroundColor Cyan
Write-Host ""

# Start and redirect stderr to file
.\target\release\deepseek.exe 2> debug.log

Write-Host ""
Write-Host "Program exited." -ForegroundColor Yellow
Write-Host "Check debug.log for full logs." -ForegroundColor Yellow
Pause
