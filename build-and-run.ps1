# DeepSeek TUI 中文版 - 一键编译安装并启动
# 此脚本会自动编译安装最新代码并启动程序

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  DeepSeek TUI 中文版 - 一键编译安装" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 第1步：检查是否在项目目录
$projectPath = "F:\ai\codes\github\deepseektuizh"
$currentPath = (Get-Location).Path

if ($currentPath -ne $projectPath) {
    Write-Host "切换到项目目录..." -ForegroundColor Yellow
    Set-Location $projectPath
}

Write-Host "[1/4] 检查 Rust 环境..." -ForegroundColor Green
$cargoVersion = cargo --version
if ($LASTEXITCODE -ne 0) {
    Write-Host "错误：未找到 cargo，请先安装 Rust" -ForegroundColor Red
    pause
    exit 1
}
Write-Host "  ✓ $cargoVersion" -ForegroundColor Green
Write-Host ""

# 第2步：清理旧编译（可选，确保使用最新代码）
Write-Host "[2/4] 清理旧的编译缓存..." -ForegroundColor Yellow
cargo clean -p deepseek-tui 2>&1 | Out-Null
Write-Host "  ✓ 清理完成" -ForegroundColor Green
Write-Host ""

# 第3步：编译安装 TUI
Write-Host "[3/4] 编译安装 DeepSeek TUI..." -ForegroundColor Yellow
Write-Host "  这可能需要几分钟，请耐心等待..." -ForegroundColor Gray
Write-Host ""

cargo install --path crates/tui --locked --force

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "错误：编译失败！" -ForegroundColor Red
    pause
    exit 1
}

Write-Host ""
Write-Host "  ✓ 编译安装成功！" -ForegroundColor Green
Write-Host ""

# 验证安装
$tuiPath = "$env:USERPROFILE\.cargo\bin\deepseek-tui.exe"
if (Test-Path $tuiPath) {
    $lastWrite = (Get-Item $tuiPath).LastWriteTime
    Write-Host "  安装位置: $tuiPath" -ForegroundColor Gray
    Write-Host "  编译时间: $lastWrite" -ForegroundColor Gray
}
Write-Host ""

# 第4步：启动程序
Write-Host "[4/4] 启动 DeepSeek TUI..." -ForegroundColor Green
Write-Host ""
Write-Host "提示：" -ForegroundColor Yellow
Write-Host "  - 启动后输入 /config locale zh-Hans 切换为中文" -ForegroundColor Yellow
Write-Host "  - 按 Ctrl+C 可以随时退出" -ForegroundColor Yellow
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  正在启动程序..." -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 设置环境变量（支持局域网LLM）
$env:DEEPSEEK_ALLOW_INSECURE_HTTP = '1'

# 启动程序
deepseek
