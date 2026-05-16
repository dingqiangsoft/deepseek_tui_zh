# DeepSeek TUI - AIClaw Project
# Auto switch to AIClaw project directory

$WORK_DIR = "F:\ai\tranprojects\AIClaw\aiclaw"

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

# Change to AIClaw project directory
if (Test-Path $WORK_DIR) {
    Set-Location $WORK_DIR
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "  DeepSeek TUI - AIClaw Project" -ForegroundColor Cyan
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Project: $WORK_DIR" -ForegroundColor Green
    Write-Host "Model: $env:DEEPSEEK_MODEL" -ForegroundColor Gray
    Write-Host ""
    
    # Start DeepSeek TUI with YOLO mode (auto-approve all tools)
    deepseek --provider openai --yolo
} else {
    Write-Host "========================================" -ForegroundColor Red
    Write-Host "  Error: Project directory not found!" -ForegroundColor Red
    Write-Host "========================================" -ForegroundColor Red
    Write-Host ""
    Write-Host "Path: $WORK_DIR" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Please check if the directory exists." -ForegroundColor Yellow
    Write-Host ""
    Pause
}
