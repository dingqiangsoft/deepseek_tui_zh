# DeepSeek TUI - Startup Menu
# Multiple startup options

function Show-Menu {
    Clear-Host
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "  DeepSeek TUI - Startup Menu" -ForegroundColor Cyan
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Please select startup mode:" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  [1] Local LLM (Qwen3.5-9b)" -ForegroundColor Green
    Write-Host "      URL: http://192.168.2.5:1234" -ForegroundColor Gray
    Write-Host "      Free, works offline" -ForegroundColor Gray
    Write-Host ""
    Write-Host "  [2] DeepSeek Cloud API" -ForegroundColor Green
    Write-Host "      Best performance, requires API Key" -ForegroundColor Gray
    Write-Host ""
    Write-Host "  [3] Test Local LLM Connection" -ForegroundColor Green
    Write-Host "      Check if service is available" -ForegroundColor Gray
    Write-Host ""
    Write-Host "  [4] Check System Status" -ForegroundColor Green
    Write-Host "      Run deepseek doctor" -ForegroundColor Gray
    Write-Host ""
    Write-Host "  [0] Exit" -ForegroundColor Red
    Write-Host ""
}

function Start-LocalLLM {
    Write-Host "`nStarting Local LLM mode..." -ForegroundColor Green
    $env:DEEPSEEK_PROVIDER = "openai"
    $env:DEEPSEEK_BASE_URL = "http://192.168.2.5:1234/v1"
    $env:DEEPSEEK_MODEL = "qwen3.5-9b-deepseek-v4-flash@q6_k"
    $env:DEEPSEEK_API_KEY = "not-needed"
    $env:DEEPSEEK_ALLOW_INSECURE_HTTP = "1"  # 允许局域网HTTP连接
    
    Write-Host "Provider: $env:DEEPSEEK_PROVIDER" -ForegroundColor Yellow
    Write-Host "Model: $env:DEEPSEEK_MODEL" -ForegroundColor Yellow
    Write-Host "Base URL: $env:DEEPSEEK_BASE_URL" -ForegroundColor Yellow
    Write-Host ""
    
    deepseek
}

function Start-CloudAPI {
    Write-Host "`nStarting DeepSeek Cloud API mode..." -ForegroundColor Green
    $env:DEEPSEEK_PROVIDER = "deepseek"
    $env:DEEPSEEK_BASE_URL = "https://api.deepseek.com/beta"
    $env:DEEPSEEK_MODEL = "deepseek-v4-pro"
    
    Write-Host "Provider: $env:DEEPSEEK_PROVIDER" -ForegroundColor Yellow
    Write-Host "Model: $env:DEEPSEEK_MODEL" -ForegroundColor Yellow
    Write-Host "Base URL: $env:DEEPSEEK_BASE_URL" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Note: Will prompt for API Key if not set" -ForegroundColor Cyan
    Write-Host ""
    
    deepseek --model auto
}

function Test-Connection {
    Write-Host "`nTesting Local LLM Connection..." -ForegroundColor Green
    $baseUrl = "http://192.168.2.5:1234"
    
    try {
        $response = Invoke-WebRequest -Uri "$baseUrl/v1/models" -Method GET -TimeoutSec 5 -ErrorAction Stop
        Write-Host "[SUCCESS] Connection established!" -ForegroundColor Green
        Write-Host "Service URL: $baseUrl" -ForegroundColor Yellow
    } catch {
        Write-Host "[FAILED] Connection failed!" -ForegroundColor Red
        Write-Host "Error: $_" -ForegroundColor Red
        Write-Host ""
        Write-Host "Please check:" -ForegroundColor Yellow
        Write-Host "  1. Local LLM service is running" -ForegroundColor Yellow
        Write-Host "  2. URL is correct" -ForegroundColor Yellow
        Write-Host "  3. Network/firewall settings" -ForegroundColor Yellow
    }
    
    Write-Host "`nPress any key to return to menu..." -ForegroundColor Gray
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
}

function Check-System {
    Write-Host "`nChecking System Status..." -ForegroundColor Green
    deepseek doctor
    Write-Host "`nPress any key to return to menu..." -ForegroundColor Gray
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
}

# Main loop
do {
    Show-Menu
    $choice = Read-Host "Enter option (0-4)"
    
    switch ($choice) {
        "1" { Start-LocalLLM }
        "2" { Start-CloudAPI }
        "3" { Test-Connection }
        "4" { Check-System }
        "0" { 
            Write-Host "`nGoodbye!" -ForegroundColor Green
            exit 
        }
        default { 
            Write-Host "`nInvalid option, please try again" -ForegroundColor Red
            Start-Sleep -Seconds 1
        }
    }
} while ($choice -ne "0")
