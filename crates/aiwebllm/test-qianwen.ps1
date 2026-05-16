# 测试千问 API 连接
# 用法: .\test-qianwen.ps1 -ApiKey "your-api-key"

param(
    [Parameter(Mandatory=$true)]
    [string]$ApiKey
)

Write-Host "🧪 测试千问 Web API 连接..." -ForegroundColor Cyan
Write-Host ""

# 构建请求体
$body = @{
    model = "qwen-turbo"
    input = @{
        messages = @(
            @{
                role = "user"
                content = "你好，这是一个测试。请简短回复测试成功。"
            }
        )
    }
    parameters = @{
        result_format = "message"
    }
} | ConvertTo-Json -Depth 10

# 设置请求头
$headers = @{
    "Authorization" = "Bearer $ApiKey"
    "Content-Type" = "application/json"
    "X-DashScope-Version" = "2024-10-15"
}

Write-Host "📤 发送请求到千问 API..." -ForegroundColor Yellow
Write-Host "   模型: qwen-turbo"
Write-Host "   端点: https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation"
Write-Host ""

try {
    # 发送请求
    $response = Invoke-RestMethod -Uri "https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation" `
                                  -Method Post `
                                  -Headers $headers `
                                  -Body $body `
                                  -ContentType "application/json"

    Write-Host "📥 收到响应！" -ForegroundColor Green
    Write-Host ""
    Write-Host "✅ 请求成功！" -ForegroundColor Green
    Write-Host ""
    Write-Host "完整响应:" -ForegroundColor Cyan
    $response | ConvertTo-Json -Depth 10 | Write-Host
    
    # 尝试提取回复内容
    if ($response.output -and $response.output.text) {
        Write-Host ""
        Write-Host "🎯 回复内容:" -ForegroundColor Magenta
        Write-Host $response.output.text -ForegroundColor White
    }

} catch {
    Write-Host "❌ 请求失败！" -ForegroundColor Red
    Write-Host ""
    Write-Host "错误信息:" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Yellow
    Write-Host ""
    
    if ($_.ErrorDetails) {
        Write-Host "详细错误:" -ForegroundColor Red
        Write-Host $_.ErrorDetails.Message -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "测试完成！" -ForegroundColor Cyan
