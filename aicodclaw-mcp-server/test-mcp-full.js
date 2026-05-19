// MCP Server 完整功能测试
// 模拟 WorkBuddy 的调用流程

import { spawn } from 'child_process';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

console.log('\n========================================');
console.log('  AICodeClaw MCP Server - 功能测试');
console.log('========================================\n');

// 启动 MCP Server
const server = spawn('node', [path.join(__dirname, 'index.js')], {
  stdio: ['pipe', 'pipe', 'pipe']
});

let messageId = 0;
const pendingRequests = new Map();
let testResults = [];

// 监听 STDERR（日志）
server.stderr.on('data', (data) => {
  const msg = data.toString().trim();
  if (msg) {
    console.log('📋 [Server Log]', msg);
  }
});

// 监听 STDOUT（响应）
server.stdout.on('data', (data) => {
  const lines = data.toString().trim().split('\n');
  
  lines.forEach(line => {
    if (!line) return;
    
    try {
      const response = JSON.parse(line);
      const id = response.id;
      
      if (pendingRequests.has(id)) {
        const request = pendingRequests.get(id);
        pendingRequests.delete(id);
        
        const isSuccess = !response.error;
        testResults.push({
          test: request.testName,
          success: isSuccess,
          result: response.result,
          error: response.error
        });
        
        if (isSuccess) {
          console.log(`✅ [${request.testName}] 成功`);
        } else {
          console.log(`❌ [${request.testName}] 失败:`, response.error?.message);
        }
      }
    } catch (e) {
      // Ignore parse errors
    }
  });
});

// 发送请求函数
function sendRequest(method, params, testName) {
  messageId++;
  const request = {
    jsonrpc: '2.0',
    id: messageId,
    method: method,
    params: params
  };
  
  pendingRequests.set(messageId, { testName, ...request });
  server.stdin.write(JSON.stringify(request) + '\n');
  console.log(`→ [${testName}] 发送 ${method}`);
  
  return messageId;
}

// 等待函数
function wait(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

// 测试流程
async function runTests() {
  console.log('\n🚀 开始测试...\n');
  
  // 测试 1: 初始化
  sendRequest('initialize', {
    protocolVersion: '2024-11-05',
    capabilities: {},
    clientInfo: { name: 'workbuddy-test', version: '1.0.0' }
  }, '1. 初始化连接');
  
  await wait(500);
  
  // 测试 2: 获取工具列表
  sendRequest('tools/list', {}, '2. 获取工具列表');
  
  await wait(500);
  
  // 测试 3: 读取文件
  sendRequest('tools/call', {
    name: 'aicodclaw_file_read',
    arguments: {
      path: path.join(__dirname, 'package.json')
    }
  }, '3. 读取 package.json');
  
  await wait(500);
  
  // 测试 4: 搜索代码
  sendRequest('tools/call', {
    name: 'aicodclaw_search',
    arguments: {
      path: __dirname,
      pattern: 'aicodclaw_file_read'
    }
  }, '4. 搜索代码');
  
  await wait(500);
  
  // 测试 5: 执行 Shell 命令
  sendRequest('tools/call', {
    name: 'aicodclaw_shell',
    arguments: {
      command: 'node --version'
    }
  }, '5. 执行 Shell 命令');
  
  await wait(500);
  
  // 等待所有响应
  await wait(2000);
  
  // 打印测试报告
  console.log('\n========================================');
  console.log('  测试报告');
  console.log('========================================\n');
  
  const passed = testResults.filter(r => r.success).length;
  const failed = testResults.filter(r => !r.success).length;
  
  testResults.forEach((result, index) => {
    const icon = result.success ? '✅' : '❌';
    console.log(`${index + 1}. ${icon} ${result.test}`);
    if (result.error) {
      console.log(`   错误: ${result.error.message}`);
    }
  });
  
  console.log('\n----------------------------------------');
  console.log(`总计: ${testResults.length} 个测试`);
  console.log(`通过: ${passed} ✅`);
  console.log(`失败: ${failed} ❌`);
  console.log('----------------------------------------\n');
  
  if (failed > 0) {
    console.log('⚠️  部分测试失败，MCP Server 可能有问题！\n');
  } else {
    console.log('🎉 所有测试通过！MCP Server 工作正常！\n');
  }
  
  // 关闭服务器
  server.kill();
  process.exit(failed > 0 ? 1 : 0);
}

// 运行测试
runTests().catch(err => {
  console.error('测试异常:', err);
  server.kill();
  process.exit(1);
});
