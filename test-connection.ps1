# DeepSeek TUI - 快速测试脚本
# 测试本地LLM服务连接

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  DeepSeek TUI - 连接测试" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 测试本地LLM服务
$baseUrl = "http://192.168.2.5:1234"
Write-Host "正在测试: $baseUrl" -ForegroundColor Yellow

try {
    $response = Invoke-WebRequest -Uri "$baseUrl/v1/models" -Method GET -TimeoutSec 5 -ErrorAction Stop
    Write-Host "✓ 连接成功!" -ForegroundColor Green
    Write-Host ""
    Write-Host "响应内容:" -ForegroundColor Cyan
    $response.Content
} catch {
    Write-Host "✗ 连接失败!" -ForegroundColor Red
    Write-Host "错误: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "请检查:" -ForegroundColor Yellow
    Write-Host "  1. 本地LLM服务是否已启动" -ForegroundColor Yellow
    Write-Host "  2. 地址 http://192.168.2.5:1234 是否正确" -ForegroundColor Yellow
    Write-Host "  3. 防火墙是否允许访问" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "按任意键退出..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
