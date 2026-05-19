# AICodeClaw MCP 自动配置脚本
# 自动检测 WorkBuddy 类型并创建正确的配置文件

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "  AICodeClaw MCP 自动配置" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

# MCP Server 路径
$mcpServerPath = "f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\index.js"

# 检查 MCP Server 是否存在
if (-not (Test-Path $mcpServerPath)) {
    Write-Host "❌ 错误：MCP Server 文件不存在" -ForegroundColor Red
    Write-Host "   路径: $mcpServerPath" -ForegroundColor Red
    exit 1
}

Write-Host "✅ MCP Server 文件存在" -ForegroundColor Green

# 配置文件内容
$mcpConfig = @{
    mcpServers = @{
        aicodclaw = @{
            command = "node"
            args = @($mcpServerPath)
            disabled = $false
        }
    }
} | ConvertTo-Json -Depth 4

# 检测并配置不同的 WorkBuddy 类型
$configured = $false

# 1. DeepSeek TUI
$deepseekConfigDir = "$env:USERPROFILE\.deepseek"
$deepseekConfigPath = "$deepseekConfigDir\mcp.json"

if (Test-Path $deepseekConfigDir) {
    Write-Host "`n[1/2] 配置 DeepSeek TUI..." -ForegroundColor Yellow
    
    try {
        if (-not (Test-Path $deepseekConfigDir)) {
            New-Item -ItemType Directory -Path $deepseekConfigDir -Force | Out-Null
        }
        
        $mcpConfig | Out-File -FilePath $deepseekConfigPath -Encoding UTF8
        Write-Host "  ✅ 已创建: $deepseekConfigPath" -ForegroundColor Green
        $configured = $true
    } catch {
        Write-Host "  ❌ 创建失败: $_" -ForegroundColor Red
    }
} else {
    Write-Host "`n[1/2] 跳过 DeepSeek TUI（目录不存在）" -ForegroundColor Gray
}

# 2. VS Code Roo-Cline
$vscodeConfigDir = "$env:APPDATA\Code\User\globalStorage\rooveterinaryinc.roo-cline\settings"
$vscodeConfigPath = "$vscodeConfigDir\mcp_settings.json"

if (Test-Path "$env:APPDATA\Code\User\globalStorage\rooveterinaryinc.roo-cline") {
    Write-Host "`n[2/2] 配置 VS Code Roo-Cline..." -ForegroundColor Yellow
    
    try {
        if (-not (Test-Path $vscodeConfigDir)) {
            New-Item -ItemType Directory -Path $vscodeConfigDir -Force | Out-Null
        }
        
        $mcpConfig | Out-File -FilePath $vscodeConfigPath -Encoding UTF8
        Write-Host "  ✅ 已创建: $vscodeConfigPath" -ForegroundColor Green
        $configured = $true
    } catch {
        Write-Host "  ❌ 创建失败: $_" -ForegroundColor Red
    }
} else {
    Write-Host "`n[2/2] 跳过 VS Code Roo-Cline（目录不存在）" -ForegroundColor Gray
}

# 总结
Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "  配置完成" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

if ($configured) {
    Write-Host "✅ MCP 配置已创建成功！" -ForegroundColor Green
    Write-Host "`n📋 下一步:" -ForegroundColor Yellow
    Write-Host "  1. 完全关闭 WorkBuddy" -ForegroundColor White
    Write-Host "  2. 重新启动 WorkBuddy" -ForegroundColor White
    Write-Host "  3. 检查是否能找到 8 个 aicodclaw_* 工具" -ForegroundColor White
    Write-Host "`n⚠️  重要：修改配置后必须重启 WorkBuddy！" -ForegroundColor Yellow
} else {
    Write-Host "❌ 未检测到任何 WorkBuddy" -ForegroundColor Red
    Write-Host "`n请先安装以下任一软件：" -ForegroundColor Yellow
    Write-Host "  - DeepSeek TUI" -ForegroundColor White
    Write-Host "  - VS Code + Roo-Cline 插件" -ForegroundColor White
}

Write-Host ""
