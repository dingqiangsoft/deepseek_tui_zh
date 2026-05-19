/**
 * AICodeClaw Autocode 后台运行脚本
 * 
 * 解决 MCP 60 秒超时问题
 * 在后台独立运行，不中断、不超时
 * 完成后自动生成报告
 */

const { exec, spawn } = require('child_process');
const path = require('path');
const fs = require('fs');
const taskPath = path.resolve(process.argv[2] || '前端任务分配计划.md');
const workDir = path.resolve(process.argv[3] || '.');
const reportPath = path.join(workDir, 'autocode-background-report.md');
const logPath = path.join(workDir, 'autocode-background.log');

// ============================================
// 日志函数（同步写入，确保不丢失）
// ============================================
function log(message) {
  const timestamp = new Date().toISOString();
  const logLine = `[${timestamp}] ${message}`;
  console.log(logLine);
  
  // 使用 appendFileSync 同步写入，确保日志不丢失
  try {
    fs.appendFileSync(logPath, logLine + '\n');
  } catch (err) {
    console.error('日志写入失败:', err.message);
  }
}

// ============================================
// 主函数
// ============================================
async function main() {
  log('🚀 后台 Autocode 已启动');
  log(`📁 工作目录: ${workDir}`);
  log(`📄 任务文件: ${taskPath}`);

  const startTime = Date.now();

  try {
    // 验证文件存在
    if (!fs.existsSync(taskPath)) {
      throw new Error(`任务文件不存在: ${taskPath}`);
    }

    if (!fs.existsSync(workDir)) {
      throw new Error(`工作目录不存在: ${workDir}`);
    }

    log('✅ 文件验证通过');

    // 读取任务文档
    const taskContent = fs.readFileSync(taskPath, 'utf-8');
    log(`📖 任务文档读取完成 (${taskContent.length} 字符)`);

    // 构建 MCP 工具调用命令
    // 使用 spawn 而不是 exec，完全独立进程
    const mcpServer = path.join(__dirname, 'index.js');
    
    log('🔧 启动 MCP Server 进程...');
    
    // 创建子进程运行 MCP Server
    const child = spawn('node', [mcpServer], {
      cwd: workDir,
      stdio: ['pipe', 'pipe', 'pipe'],
      detached: true, // 独立进程组
      env: { ...process.env }
    });

    log(`✅ MCP Server 进程已启动 (PID: ${child.pid})`);

    // 准备初始化 JSON-RPC 请求
    const initializeRequest = JSON.stringify({
      jsonrpc: '2.0',
      id: 1,
      method: 'initialize',
      params: {
        protocolVersion: '2024-11-05',
        capabilities: {},
        clientInfo: {
          name: 'autocode-background',
          version: '1.0.0'
        }
      }
    }) + '\n';

    // 准备 tools/call 请求
    const callRequest = JSON.stringify({
      jsonrpc: '2.0',
      id: 2,
      method: 'tools/call',
      params: {
        name: 'aicodclaw_autocode',
        arguments: {
          taskDocPath: taskPath,
          workDir: workDir
        }
      }
    }) + '\n';

    // 发送初始化请求
    child.stdin.write(initializeRequest);
    log('📤 已发送 initialize 请求');

    // 等待初始化响应
    let initResponse = '';
    await new Promise((resolve) => {
      const onData = (data) => {
        initResponse += data.toString();
        if (initResponse.includes('"id":1')) {
          child.stdout.off('data', onData);
          resolve();
        }
      };
      child.stdout.on('data', onData);
      
      // 5 秒超时
      setTimeout(resolve, 5000);
    });

    log('✅ MCP Server 初始化完成');

    // 发送 autocode 工具调用
    child.stdin.write(callRequest);
    log('📤 已发送 autocode 工具调用');
    log('⏳ 开始自动开发（这可能需要几分钟）...');

    // 持续读取输出
    let finalResponse = '';
    child.stdout.on('data', (data) => {
      finalResponse += data.toString();
      
      // 检测是否完成
      if (finalResponse.includes('"id":2')) {
        log('✅ autocode 工具调用完成');
        
        // 提取结果
        const match = finalResponse.match(/\{"jsonrpc":"2.0","id":2,[\s\S]*\}/);
        if (match) {
          try {
            const result = JSON.parse(match[0]);
            const text = result.result?.content?.[0]?.text || '';
            
            log('📊 开发完成！');
            log(text);
            
            // 生成报告
            const report = `
# 🚀 AICodeClaw 后台自动开发完成

## 基本信息
- **任务文件**: ${taskPath}
- **工作目录**: ${workDir}
- **开始时间**: ${new Date(startTime).toISOString()}
- **完成时间**: ${new Date().toISOString()}
- **耗时**: ${((Date.now() - startTime) / 1000).toFixed(1)} 秒

## 执行结果

${text}

---
**后台运行脚本 v1.0**
            `.trim();

            fs.writeFileSync(reportPath, report, 'utf-8');
            log(`📄 报告已生成: ${reportPath}`);
          } catch (e) {
            log(`⚠️ 解析结果失败: ${e.message}`);
          }
        }
        
        // 清理进程
        child.kill();
        log('✅ 后台任务完成，进程已清理');
        process.exit(0);
      }
    });

    // 错误处理
    child.stderr.on('data', (data) => {
      log(`⚠️ stderr: ${data.toString()}`);
    });

    child.on('error', (err) => {
      log(`❌ 进程错误: ${err.message}`);
      process.exit(1);
    });

    child.on('exit', (code) => {
      log(`✅ 进程退出 (code: ${code})`);
      process.exit(code);
    });

    // 保持进程运行
    // 这个脚本会在后台一直运行，直到 autocode 完成

  } catch (error) {
    log(`❌ 错误: ${error.message}`);
    
    // 生成错误报告
    const errorReport = `
# ❌ AICodeClaw 后台自动开发失败

## 错误信息
- **错误**: ${error.message}
- **时间**: ${new Date().toISOString()}
- **任务文件**: ${taskPath}
- **工作目录**: ${workDir}

## 堆栈
${error.stack}

---
**后台运行脚本 v1.0**
    `.trim();

    fs.writeFileSync(reportPath, errorReport, 'utf-8');
    log(`📄 错误报告已生成: ${reportPath}`);
    
    process.exit(1);
  }
}

// 启动
main();
