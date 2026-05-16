# DeepSeek TUI - Quick Start
# Direct startup with Local LLM

# Set environment variables
$env:DEEPSEEK_PROVIDER = "openai"
$env:DEEPSEEK_BASE_URL = "http://192.168.2.5:1234/v1"
$env:DEEPSEEK_MODEL = "qwen3.5-9b-deepseek-v4-flash@q6_k"
$env:DEEPSEEK_API_KEY = "not-needed"
$env:DEEPSEEK_ALLOW_INSECURE_HTTP = "1"  # 允许局域网HTTP连接

# Start DeepSeek TUI
# 直接使用 deepseek 命令会继承上面设置的所有环境变量
deepseek
