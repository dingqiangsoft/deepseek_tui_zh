# DeepSeek TUI - Dark Theme Startup
# Start with dark theme for eye comfort

# Set environment variables
$env:DEEPSEEK_PROVIDER = "openai"
$env:DEEPSEEK_BASE_URL = "http://192.168.2.5:1234/v1"
$env:DEEPSEEK_MODEL = "qwen3.5-9b-deepseek-v4-flash@q6_k"
$env:DEEPSEEK_API_KEY = "not-needed"
$env:DEEPSEEK_ALLOW_INSECURE_HTTP = "1"

# Set PowerShell to dark theme colors
$host.UI.RawUI.BackgroundColor = 'Black'
$host.UI.RawUI.ForegroundColor = 'White'
$host.PrivateData.ErrorForegroundColor = 'Red'
$host.PrivateData.WarningForegroundColor = 'Yellow'
Clear-Host

# Maximize window
$shell = New-Object -ComObject Shell.Application
$shell.Windows() | Where-Object { $_.LocationURL -eq $null } | ForEach-Object { $_.Fullscreen = $true }

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  DeepSeek TUI - Dark Theme" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Starting with dark theme for eye comfort..." -ForegroundColor Yellow
Write-Host "Provider: $env:DEEPSEEK_PROVIDER" -ForegroundColor Gray
Write-Host "Model: $env:DEEPSEEK_MODEL" -ForegroundColor Gray
Write-Host ""

# Start DeepSeek TUI
deepseek --provider openai
