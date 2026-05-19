# 快速测试 aiwebllm 模块
# 用法: .\quick-test.ps1

Write-Host "`n=== aiwebllm 快速测试 ===" -ForegroundColor Cyan

# 检查是否设置了 API Key
$apiKey = $env:QIANWEN_API_KEY
if (-not $apiKey) {
    Write-Host "`n⚠ 未设置 QIANWEN_API_KEY 环境变量" -ForegroundColor Yellow
    Write-Host "请运行以下命令设置 API Key:" -ForegroundColor Yellow
    Write-Host '`$env:QIANWEN_API_KEY = "你的API密钥"' -ForegroundColor White
    Write-Host "`n获取 API Key: https://dashscope.console.aliyun.com/" -ForegroundColor Yellow
    
    $continue = Read-Host "`n是否继续运行网页版测试？(y/n)"
    if ($continue -ne 'y') {
        exit
    }
} else {
    Write-Host "`n✓ 检测到 QIANWEN_API_KEY" -ForegroundColor Green
}

# 进入目录
Push-Location "crates/aiwebllm"

# 测试 1: 编译检查
Write-Host "`n[1/3] 编译检查..." -ForegroundColor Cyan
cargo check 2>&1 | Out-Null
if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ 编译成功" -ForegroundColor Green
} else {
    Write-Host "✗ 编译失败" -ForegroundColor Red
    Pop-Location
    exit 1
}

# 测试 2: 单元测试
Write-Host "`n[2/3] 运行单元测试..." -ForegroundColor Cyan
cargo test --lib 2>&1 | Select-String "test result|running" | ForEach-Object { Write-Host $_ }

# 测试 3: 集成测试（如果有 API Key）
if ($apiKey) {
    Write-Host "`n[3/3] 运行 API 连接测试..." -ForegroundColor Cyan
    cargo run --example test_qianwen_web
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "`n✓ 所有测试通过！" -ForegroundColor Green
    } else {
        Write-Host "`n⚠ API 测试失败，请检查 API Key 和网络" -ForegroundColor Yellow
    }
} else {
    Write-Host "`n[3/3] 跳过 API 测试（未设置 API Key）" -ForegroundColor Yellow
}

Pop-Location

Write-Host "`n=== 测试完成 ===" -ForegroundColor Cyan
Write-Host "`n下一步:" -ForegroundColor White
Write-Host "1. 设置 API Key 后重新测试" -ForegroundColor Gray
Write-Host "2. 运行完整的 TUI 测试: cargo run --bin deepseek" -ForegroundColor Gray
Write-Host "3. 在 TUI 中使用 /web 命令测试 Web LLM" -ForegroundColor Gray
