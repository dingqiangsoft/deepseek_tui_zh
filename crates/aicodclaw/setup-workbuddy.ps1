# WorkBuddy 对接 AICodeClaw 自动化配置脚本
# 用于腾讯云 WorkBuddy 黑客松比赛

Write-Host "`n🔌 WorkBuddy 对接 AICodeClaw 配置脚本" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════`n" -ForegroundColor Cyan

# ═══════════════════════════════════════════════════════════
# 步骤 1：检查前置条件
# ═══════════════════════════════════════════════════════════

Write-Host "[步骤 1/5] 检查前置条件" -ForegroundColor Yellow

# 检查 AICodeClaw 可执行文件
$AICodeClawPath = ".\target\release\deepseek.exe"
if (Test-Path $AICodeClawPath) {
    Write-Host "✅ 找到 AICodeClaw: $AICodeClawPath" -ForegroundColor Green
    $AICodeClawFullPath = (Get-Item $AICodeClawPath).FullName
} else {
    Write-Host "❌ 未找到 AICodeClaw 可执行文件" -ForegroundColor Red
    Write-Host "   请先运行: cargo build --release" -ForegroundColor Yellow
    exit 1
}

# 检查配置文件
$ConfigTemplate = ".\crates\aicodclaw\workbuddy-config.json"
if (Test-Path $ConfigTemplate) {
    Write-Host "✅ 找到配置模板: $ConfigTemplate" -ForegroundColor Green
} else {
    Write-Host "❌ 未找到配置模板" -ForegroundColor Red
    exit 1
}

# ═══════════════════════════════════════════════════════════
# 步骤 2：查找 WorkBuddy 配置目录
# ═══════════════════════════════════════════════════════════

Write-Host "`n[步骤 2/5] 查找 WorkBuddy 配置目录" -ForegroundColor Yellow

$PossiblePaths = @(
    "$env:APPDATA\WorkBuddy",
    "$env:USERPROFILE\.workbuddy",
    "$env:USERPROFILE\.config\workbuddy",
    "$env:LOCALAPPDATA\WorkBuddy"
)

$WorkBuddyConfigDir = $null
foreach ($path in $PossiblePaths) {
    if (Test-Path $path) {
        $WorkBuddyConfigDir = $path
        Write-Host "✅ 找到 WorkBuddy 配置目录: $path" -ForegroundColor Green
        break
    }
}

if (-Not $WorkBuddyConfigDir) {
    Write-Host "⚠️ 未找到 WorkBuddy 配置目录" -ForegroundColor Yellow
    Write-Host "   请手动指定 WorkBuddy 配置目录路径:" -ForegroundColor Yellow
    $WorkBuddyConfigDir = Read-Host "输入路径"
    
    if (-Not (Test-Path $WorkBuddyConfigDir)) {
        Write-Host "❌ 路径不存在，创建新目录..." -ForegroundColor Yellow
        New-Item -ItemType Directory -Path $WorkBuddyConfigDir -Force | Out-Null
    }
}

# ═══════════════════════════════════════════════════════════
# 步骤 3：生成配置文件
# ═══════════════════════════════════════════════════════════

Write-Host "`n[步骤 3/5] 生成 WorkBuddy 配置文件" -ForegroundColor Yellow

# 读取模板
$ConfigContent = Get-Content $ConfigTemplate -Raw

# 替换路径（使用绝对路径）
$ConfigContent = $ConfigContent -replace 'F:\\\\ai\\\\codes\\\\github\\\\deepseektuizh\\\\target\\\\release\\\\deepseek.exe', ($AICodeClawFullPath -replace '\\', '\\')

# 目标配置文件
$TargetConfigPath = "$WorkBuddyConfigDir\config.json"

# 备份原配置（如果存在）
if (Test-Path $TargetConfigPath) {
    $BackupPath = "$WorkBuddyConfigDir\config.json.backup.$(Get-Date -Format 'yyyyMMddHHmmss')"
    Copy-Item $TargetConfigPath $BackupPath
    Write-Host "✅ 已备份原配置: $BackupPath" -ForegroundColor Green
}

# 写入新配置
$ConfigContent | Out-File -FilePath $TargetConfigPath -Encoding UTF8
Write-Host "✅ 已生成配置文件: $TargetConfigPath" -ForegroundColor Green

# ═══════════════════════════════════════════════════════════
# 步骤 4：验证配置
# ═══════════════════════════════════════════════════════════

Write-Host "`n[步骤 4/5] 验证配置" -ForegroundColor Yellow

# 检查配置文件内容
$FinalConfig = Get-Content $TargetConfigPath -Raw
Write-Host "`n配置文件内容:" -ForegroundColor Cyan
Write-Host $FinalConfig -ForegroundColor White

# 测试 MCP Server（可选）
Write-Host "`n是否测试 MCP Server 连接？(y/n)" -ForegroundColor Yellow
$TestMCP = Read-Host

if ($TestMCP -eq 'y' -or $TestMCP -eq 'Y') {
    Write-Host "`n启动 MCP Server 测试（按 Ctrl+C 退出）..." -ForegroundColor Magenta
    & $AICodeClawPath mcp stdio
}

# ═══════════════════════════════════════════════════════════
# 步骤 5：完成提示
# ═══════════════════════════════════════════════════════════

Write-Host "`n[步骤 5/5] 配置完成" -ForegroundColor Yellow

Write-Host "`n✅ WorkBuddy 对接配置完成！" -ForegroundColor Green
Write-Host "═══════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""
Write-Host "下一步操作：" -ForegroundColor Yellow
Write-Host "1. 重启 WorkBuddy" -ForegroundColor White
Write-Host "2. 在 WorkBuddy 中检查 MCP Server 状态" -ForegroundColor White
Write-Host "3. 尝试调用 AICodeClaw 的工具" -ForegroundColor White
Write-Host ""
Write-Host "配置文件位置：" -ForegroundColor Yellow
Write-Host "  $TargetConfigPath" -ForegroundColor White
Write-Host ""
Write-Host "如果需要恢复原配置：" -ForegroundColor Yellow
Write-Host "  删除 $TargetConfigPath" -ForegroundColor White
Write-Host "  重命名备份文件为 config.json" -ForegroundColor White
Write-Host ""
Write-Host "═══════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "🎯 祝比赛成功！" -ForegroundColor Green
Write-Host "═══════════════════════════════════════════════`n" -ForegroundColor Cyan
