@echo off
chcp 65001 >nul
echo ========================================
echo   DeepSeek TUI 中文版 - 快速启动
echo ========================================
echo.

cd /d F:\ai\codes\github\deepseektuizh

echo [1/2] 设置环境变量...
set DEEPSEEK_PROVIDER=openai
set DEEPSEEK_BASE_URL=http://192.168.2.5:1234/v1
set DEEPSEEK_MODEL=qwen3.5-9b-deepseek-v4-flash@q6_k
set DEEPSEEK_API_KEY=not-needed
set DEEPSEEK_ALLOW_INSECURE_HTTP=1

echo [2/2] 启动程序...
echo.
echo 提示: 启动后输入 /config locale zh-Hans 切换中文
echo 提示: 按 Ctrl+C 两次可以退出程序
echo.

deepseek.exe

echo.
echo 程序已退出。按任意键关闭此窗口...
pause >nul
