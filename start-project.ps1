# DeepSeek TUI - Quick Start with Custom Directory
# Change WORK_DIR to your project path

$WORK_DIR = "F:\ai\codes\github\DeepSeek-TUI-main\DeepSeek-TUI-main"  # <-- 修改这里

# Set environment variables
$env:DEEPSEEK_PROVIDER = "openai"
$env:DEEPSEEK_BASE_URL = "http://192.168.2.5:1234/v1"
$env:DEEPSEEK_MODEL = "qwen3.5-9b-deepseek-v4-flash@q6_k"
$env:DEEPSEEK_API_KEY = "not-needed"
$env:DEEPSEEK_ALLOW_INSECURE_HTTP = "1"

# Set PowerShell to dark theme
$host.UI.RawUI.BackgroundColor = 'Black'
$host.UI.RawUI.ForegroundColor = 'White'
Clear-Host

# Change to project directory
Set-Location $WORK_DIR

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  DeepSeek TUI - Dark Theme" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Project: $WORK_DIR" -ForegroundColor Yellow
Write-Host "Model: $env:DEEPSEEK_MODEL" -ForegroundColor Gray
Write-Host ""

# Start DeepSeek TUI
deepseek --provider openai
