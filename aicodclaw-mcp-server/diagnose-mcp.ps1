# AICodeClaw MCP 连接诊断脚本

Write-Host "`n=== AICodeClaw MCP 连接诊断 ===" -ForegroundColor Cyan

# 1. 检查 Node.js
Write-Host "`n[1/5] 检查 Node.js..." -ForegroundColor Yellow
try {
    $nodeVersion = node --version
    Write-Host "  ✅ Node.js 已安装: $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "  ❌ Node.js 未安装" -ForegroundColor Red
    Write-Host "  请安装 Node.js: https://nodejs.org/" -ForegroundColor Red
    exit 1
}

# 2. 检查 MCP Server 文件
Write-Host "`n[2/5] 检查 MCP Server 文件..." -ForegroundColor Yellow
$mcpServerPath = "f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\index.js"
if (Test-Path $mcpServerPath) {
    Write-Host "  ✅ MCP Server 文件存在: $mcpServerPath" -ForegroundColor Green
    $fileSize = (Get-Item $mcpServerPath).Length
    Write-Host "  📄 文件大小: $([math]::Round($fileSize/1024, 2)) KB" -ForegroundColor Gray
} else {
    Write-Host "  ❌ MCP Server 文件不存在: $mcpServerPath" -ForegroundColor Red
    exit 1
}

# 3. 检查依赖
Write-Host "`n[3/5] 检查依赖包..." -ForegroundColor Yellow
$packageJsonPath = "f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\package.json"
if (Test-Path $packageJsonPath) {
    $packageJson = Get-Content $packageJsonPath | ConvertFrom-Json
    Write-Host "  ✅ package.json 存在" -ForegroundColor Green
    
    $nodeModulesPath = "f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\node_modules"
    if (Test-Path $nodeModulesPath) {
        Write-Host "  ✅ node_modules 存在" -ForegroundColor Green
    } else {
        Write-Host "  ⚠️  node_modules 不存在，需要运行 npm install" -ForegroundColor Yellow
        Write-Host "  执行: cd f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server; npm install" -ForegroundColor Yellow
    }
} else {
    Write-Host "  ❌ package.json 不存在" -ForegroundColor Red
}

# 4. 测试 MCP Server 启动
Write-Host "`n[4/5] 测试 MCP Server 启动..." -ForegroundColor Yellow
try {
    $process = Start-Process -FilePath "node" -ArgumentList @("`"f:\ai\codes\github\deepseektuizh\aicodclaw-mcp-server\index.js`"") -PassThru -NoNewWindow
    Start-Sleep -Seconds 2
    
    if ($process.HasExited) {
        Write-Host "  ❌ MCP Server 启动失败" -ForegroundColor Red
        Write-Host "  退出代码: $($process.ExitCode)" -ForegroundColor Red
    } else {
        Write-Host "  ✅ MCP Server 启动成功 (PID: $($process.Id))" -ForegroundColor Green
        Stop-Process -Id $process.Id -Force
    }
} catch {
    Write-Host "  ❌ MCP Server 启动异常: $_" -ForegroundColor Red
}

# 5. 检查常见配置位置
Write-Host "`n[5/5] 检查 MCP 配置文件..." -ForegroundColor Yellow

$configPaths = @(
    "$env:USERPROFILE\.deepseek\mcp.json",
    "$env:APPDATA\Code\User\globalStorage\rooveterinaryinc.roo-cline\settings\mcp_settings.json",
    "$env:LOCALAPPDATA\Code\User\globalStorage\rooveterinaryinc.roo-cline\settings\mcp_settings.json"
)

$configFound = $false
foreach ($configPath in $configPaths) {
    if (Test-Path $configPath) {
        Write-Host "  ✅ 找到配置文件: $configPath" -ForegroundColor Green
        $configFound = $true
        
        # 显示配置内容
        try {
            $config = Get-Content $configPath | ConvertFrom-Json
            if ($config.mcpServers) {
                Write-Host "  📋 MCP 服务器列表:" -ForegroundColor Gray
                foreach ($server in $config.mcpServers.PSObject.Properties) {
                    Write-Host "    - $($server.Name)" -ForegroundColor Gray
                    if ($server.Value.command) {
                        Write-Host "      命令: $($server.Value.command)" -ForegroundColor Gray
                    }
                    if ($server.Value.args) {
                        Write-Host "      参数: $($server.Value.args -join ' ')" -ForegroundColor Gray
                    }
                    if ($server.Value.disabled -eq $true) {
                        Write-Host "      状态: ❌ 已禁用" -ForegroundColor Red
                    } else {
                        Write-Host "      状态: ✅ 已启用" -ForegroundColor Green
                    }
                }
            }
        } catch {
            Write-Host "  ⚠️  配置文件解析失败: $_" -ForegroundColor Yellow
        }
    }
}

if (-not $configFound) {
    Write-Host "  ⚠️  未找到 MCP 配置文件" -ForegroundColor Yellow
    Write-Host "  请创建配置文件，参考以下示例:" -ForegroundColor Yellow
}

# 诊断总结
Write-Host "`n=== 诊断总结 ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "如果 MCP 连接失败，请检查:" -ForegroundColor Yellow
Write-Host "  1. WorkBuddy 是否已重启（修改 index.js 后必须重启）" -ForegroundColor White
Write-Host "  2. mcp.json 配置路径是否正确" -ForegroundColor White
Write-Host "  3. 服务器是否被禁用（disabled: true）" -ForegroundColor White
Write-Host "  4. Node.js 路径是否在系统 PATH 中" -ForegroundColor White
Write-Host ""
Write-Host "标准配置示例:" -ForegroundColor Green
Write-Host @"
{
  "mcpServers": {
    "aicodclaw": {
      "command": "node",
      "args": [
        "f:\\ai\\codes\\github\\deepseektuizh\\aicodclaw-mcp-server\\index.js"
      ],
      "disabled": false
    }
  }
}
"@ -ForegroundColor Gray
Write-Host ""
