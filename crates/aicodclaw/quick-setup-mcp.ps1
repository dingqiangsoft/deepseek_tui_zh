# 一键配置 WorkBuddy MCP
# 将 AICodeClaw 添加到 WorkBuddy 的 MCP 配置中

Write-Host "`n🔌 一键配置 WorkBuddy MCP" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════`n" -ForegroundColor Cyan

# 配置文件路径
$WorkBuddyMcpConfig = "C:\Users\Administrator\.workbuddy\.mcp.json"
$NewConfig = ".\crates\aicodclaw\mcp-config-complete.json"

# 检查文件
if (-Not (Test-Path $WorkBuddyMcpConfig)) {
    Write-Host "❌ 未找到 WorkBuddy MCP 配置文件" -ForegroundColor Red
    exit 1
}

if (-Not (Test-Path $NewConfig)) {
    Write-Host "❌ 未找到新配置文件" -ForegroundColor Red
    exit 1
}

# 备份原配置
$BackupPath = "C:\Users\Administrator\.workbuddy\.mcp.json.backup.$(Get-Date -Format 'yyyyMMddHHmmss')"
Copy-Item $WorkBuddyMcpConfig $BackupPath
Write-Host "✅ 已备份原配置: $BackupPath" -ForegroundColor Green

# 显示原配置
Write-Host "`n原配置内容:" -ForegroundColor Yellow
Get-Content $WorkBuddyMcpConfig | Write-Host -ForegroundColor Gray

# 显示新配置
Write-Host "`n新配置内容:" -ForegroundColor Yellow
Get-Content $NewConfig | Write-Host -ForegroundColor White

# 确认是否应用
Write-Host "`n是否应用新配置？(y/n)" -ForegroundColor Cyan
$Confirm = Read-Host

if ($Confirm -ne 'y' -and $Confirm -ne 'Y') {
    Write-Host "❌ 已取消配置" -ForegroundColor Yellow
    exit 0
}

# 应用新配置
Copy-Item $NewConfig $WorkBuddyMcpConfig -Force
Write-Host "`n✅ 配置已应用！" -ForegroundColor Green

# 验证
Write-Host "`n验证配置:" -ForegroundColor Cyan
Get-Content $WorkBuddyMcpConfig | Write-Host -ForegroundColor White

Write-Host "`n═══════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "下一步：" -ForegroundColor Yellow
Write-Host "1. 重启 WorkBuddy" -ForegroundColor White
Write-Host "2. 检查 MCP Server 列表中是否有 aicodclaw" -ForegroundColor White
Write-Host "3. 测试工具调用" -ForegroundColor White
Write-Host ""
Write-Host "如需恢复原配置：" -ForegroundColor Yellow
Write-Host "复制 $BackupPath 到 $WorkBuddyMcpConfig" -ForegroundColor White
Write-Host "═══════════════════════════════════════════════`n" -ForegroundColor Cyan
