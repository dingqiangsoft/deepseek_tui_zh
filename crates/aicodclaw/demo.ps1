# AICodeClaw 自动化演示脚本 (PowerShell)
# 用于录制比赛演示视频

param(
    [string]$DemoMode = "full",  # full, quick, mcp
    [string]$Model = "auto",
    [string]$Platform = "qianwen"
)

# ═══════════════════════════════════════════════════════════
# AICodeClaw 自动化演示脚本
# 用于腾讯云 WorkBuddy 黑客松比赛
# ═══════════════════════════════════════════════════════════

Write-Host "`n🦞 AICodeClaw - 自动化演示脚本" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════`n" -ForegroundColor Cyan

# 检查 AICodeClaw 是否存在
$AICodeClawPath = ".\target\release\deepseek.exe"
if (-Not (Test-Path $AICodeClawPath)) {
    Write-Host "❌ 错误：未找到 AICodeClaw 可执行文件" -ForegroundColor Red
    Write-Host "   请先运行: cargo build --release" -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ 找到 AICodeClaw: $AICodeClawPath" -ForegroundColor Green

# ═══════════════════════════════════════════════════════════
# 演示模式 1：完整演示 (full)
# ═══════════════════════════════════════════════════════════
function Start-FullDemo {
    Write-Host "`n📹 完整演示模式" -ForegroundColor Cyan
    Write-Host "───────────────────────────────────────────`n" -ForegroundColor Cyan

    # 步骤 1：显示版本信息
    Write-Host "[步骤 1/5] 显示版本信息" -ForegroundColor Yellow
    Write-Host "> AICodeClaw --version`n" -ForegroundColor Gray
    & $AICodeClawPath --version
    Start-Sleep -Seconds 2

    # 步骤 2：显示 MCP 配置
    Write-Host "`n[步骤 2/5] 显示 MCP 配置" -ForegroundColor Yellow
    Write-Host "> 配置文件: crates/aicodclaw/mcp-config.toml`n" -ForegroundColor Gray
    Get-Content "crates/aicodclaw/mcp-config.toml" | Select-Object -First 15
    Start-Sleep -Seconds 2

    # 步骤 3：测试 Web LLM 查询
    Write-Host "`n[步骤 3/5] 测试 Web LLM 查询" -ForegroundColor Yellow
    Write-Host "> /web $Platform `"推荐一个适合本地部署的 OCR 工具`"`n" -ForegroundColor Gray
    Write-Host "   (此步骤需要手动在 TUI 中输入命令)" -ForegroundColor Magenta
    Write-Host "   启动 TUI: .\target\release\deepseek" -ForegroundColor Magenta
    Start-Sleep -Seconds 3

    # 步骤 4：展示 Sub-agent 能力
    Write-Host "`n[步骤 4/5] 展示 Sub-agent 能力" -ForegroundColor Yellow
    Write-Host "> /agent_open `"审查 crates/aiwebllm 的代码质量`"`n" -ForegroundColor Gray
    Write-Host "   (此步骤需要手动在 TUI 中输入命令)" -ForegroundColor Magenta
    Start-Sleep -Seconds 3

    # 步骤 5：显示项目结构
    Write-Host "`n[步骤 5/5] 显示项目结构" -ForegroundColor Yellow
    Write-Host "> tree crates /F`n" -ForegroundColor Gray
    tree crates /F | Select-Object -First 30
    Start-Sleep -Seconds 2

    Write-Host "`n✅ 完整演示完成！" -ForegroundColor Green
    Write-Host "   请在 TUI 中手动测试 /web 和 /agent_open 命令" -ForegroundColor Yellow
}

# ═══════════════════════════════════════════════════════════
# 演示模式 2：快速演示 (quick)
# ═══════════════════════════════════════════════════════════
function Start-QuickDemo {
    Write-Host "`n⚡ 快速演示模式" -ForegroundColor Cyan
    Write-Host "───────────────────────────────────────────`n" -ForegroundColor Cyan

    # 快速显示关键信息
    Write-Host "🦞 AICodeClaw 核心信息:`n" -ForegroundColor Cyan
    Write-Host "  定位：开源的企业 AI 私域引擎" -ForegroundColor White
    Write-Host "        零泄露的 AI 软件工厂`n" -ForegroundColor White
    
    Write-Host "核心优势：" -ForegroundColor Yellow
    Write-Host "  ✓ 开源免费 - 代码透明可审计" -ForegroundColor Green
    Write-Host "  ✓ 私域部署 - 数据永不离场" -ForegroundColor Green
    Write-Host "  ✓ MCP 协议 - 无缝接入 WorkBuddy" -ForegroundColor Green
    Write-Host "  ✓ 研发效率提升 300%`n" -ForegroundColor Green

    Write-Host "适用场景：" -ForegroundColor Yellow
    Write-Host "  🏦 金融机构 - 代码审查" -ForegroundColor White
    Write-Host "  🏛️ 政企单位 - 智能运维" -ForegroundColor White
    Write-Host "  🏢 科技公司 - 研发辅助`n" -ForegroundColor White

    Write-Host "✅ 快速演示完成！" -ForegroundColor Green
}

# ═══════════════════════════════════════════════════════════
# 演示模式 3：MCP 演示 (mcp)
# ═══════════════════════════════════════════════════════════
function Start-McpDemo {
    Write-Host "`n🔌 MCP 协议演示模式" -ForegroundColor Cyan
    Write-Host "───────────────────────────────────────────`n" -ForegroundColor Cyan

    Write-Host "[步骤 1/3] 显示 MCP 配置文件" -ForegroundColor Yellow
    Write-Host "文件位置: crates/aicodclaw/mcp-config.toml`n" -ForegroundColor Gray
    
    if (Test-Path "crates/aicodclaw/mcp-config.toml") {
        Get-Content "crates/aicodclaw/mcp-config.toml"
    } else {
        Write-Host "❌ 配置文件不存在" -ForegroundColor Red
    }

    Write-Host "`n[步骤 2/3] 测试 MCP Server" -ForegroundColor Yellow
    Write-Host "> .\target\release\deepseek mcp stdio`n" -ForegroundColor Gray
    Write-Host "   (此步骤会启动 MCP Server，按 Ctrl+C 退出)" -ForegroundColor Magenta
    Start-Sleep -Seconds 3

    Write-Host "`n[步骤 3/3] WorkBuddy 配置示例" -ForegroundColor Yellow
    Write-Host @"

{
  "mcpServers": {
    "aicodclaw": {
      "command": "aicodclaw",
      "args": ["mcp", "stdio"],
      "env": {
        "DEEPSEEK_PROVIDER": "openai",
        "DEEPSEEK_BASE_URL": "http://localhost:11434/v1",
        "DEEPSEEK_MODEL": "qwen3.5-9b-deepseek-v4-flash@q6_k"
      }
    }
  }
}

"@ -ForegroundColor White

    Write-Host "`n✅ MCP 演示完成！" -ForegroundColor Green
}

# ═══════════════════════════════════════════════════════════
# 主程序
# ═══════════════════════════════════════════════════════════

switch ($DemoMode.ToLower()) {
    "full"   { Start-FullDemo }
    "quick"  { Start-QuickDemo }
    "mcp"    { Start-McpDemo }
    default  {
        Write-Host "`n❌ 未知的演示模式: $DemoMode" -ForegroundColor Red
        Write-Host "   可用模式: full, quick, mcp`n" -ForegroundColor Yellow
        exit 1
    }
}

Write-Host "`n═══════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "🎯 演示完成！祝比赛成功！" -ForegroundColor Green
Write-Host "═══════════════════════════════════════════════`n" -ForegroundColor Cyan
