# /web 命令诊断脚本
# 用于排查 "Unknown command: /web" 问题

Write-Host "`n=== /web 命令诊断 ===" -ForegroundColor Cyan

# 1. 检查二进制文件是否存在
Write-Host "`n[1/5] 检查二进制文件..." -ForegroundColor Cyan
if (Test-Path "./target/release/deepseek.exe") {
    $file = Get-Item "./target/release/deepseek.exe"
    Write-Host "✓ 找到: $($file.FullName)" -ForegroundColor Green
    Write-Host "  最后修改: $($file.LastWriteTime)" -ForegroundColor Gray
} else {
    Write-Host "✗ 未找到二进制文件" -ForegroundColor Red
    Write-Host "  请运行: cargo build --release" -ForegroundColor Yellow
    exit 1
}

# 2. 检查编译时间是否太旧
$now = Get-Date
$age = $now - $file.LastWriteTime
if ($age.TotalMinutes -gt 60) {
    Write-Host "⚠ 编译时间较旧 ($([math]::Round($age.TotalMinutes, 0)) 分钟前)" -ForegroundColor Yellow
    $rebuild = Read-Host "是否重新编译？(y/n)"
    if ($rebuild -eq 'y') {
        Write-Host "`n重新编译中..." -ForegroundColor Cyan
        cargo build --release 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✓ 编译成功" -ForegroundColor Green
        } else {
            Write-Host "✗ 编译失败" -ForegroundColor Red
            exit 1
        }
    }
} else {
    Write-Host "✓ 编译时间是最新的" -ForegroundColor Green
}

# 3. 检查源代码中是否有 /web 命令
Write-Host "`n[2/5] 检查源代码..." -ForegroundColor Cyan
$webCmdFile = "crates/tui/src/commands/mod.rs"
if (Test-Path $webCmdFile) {
    $content = Get-Content $webCmdFile -Raw
    if ($content -match '"web"\s*\|\s*"wangluo"') {
        Write-Host "✓ 源代码中包含 /web 命令注册" -ForegroundColor Green
    } else {
        Write-Host "✗ 源代码中未找到 /web 命令注册" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "✗ 未找到命令注册文件" -ForegroundColor Red
    exit 1
}

# 4. 检查 web.rs 文件
Write-Host "`n[3/5] 检查 web.rs 文件..." -ForegroundColor Cyan
$webFile = "crates/tui/src/commands/web.rs"
if (Test-Path $webFile) {
    Write-Host "✓ web.rs 文件存在" -ForegroundColor Green
    $webContent = Get-Content $webFile -Raw
    if ($webContent -match "handle_web_command") {
        Write-Host "✓ handle_web_command 函数存在" -ForegroundColor Green
    } else {
        Write-Host "✗ handle_web_command 函数不存在" -ForegroundColor Red
    }
} else {
    Write-Host "✗ web.rs 文件不存在" -ForegroundColor Red
    exit 1
}

# 5. 检查依赖
Write-Host "`n[4/5] 检查 aiwebllm 依赖..." -ForegroundColor Cyan
$tuiCargo = "crates/tui/Cargo.toml"
if (Test-Path $tuiCargo) {
    $cargoContent = Get-Content $tuiCargo -Raw
    if ($cargoContent -match "deepseek-aiwebllm") {
        Write-Host "✓ aiwebllm 依赖已添加" -ForegroundColor Green
    } else {
        Write-Host "✗ aiwebllm 依赖未添加" -ForegroundColor Red
    }
}

# 6. 测试运行
Write-Host "`n[5/5] 测试运行..." -ForegroundColor Cyan
Write-Host "即将启动 TUI，请在其中输入 /help 查看命令列表" -ForegroundColor Yellow
Write-Host "按任意键继续..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

Write-Host "`n启动 TUI..." -ForegroundColor Cyan
./target/release/deepseek.exe
