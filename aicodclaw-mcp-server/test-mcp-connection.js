// MCP Server 完整连接测试
// 模拟 WorkBuddy 的完整调用流程

import { spawn } from 'child_process';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const server = spawn('node', ['index.js'], {
  stdio: ['pipe', 'pipe', 'pipe']
});

let messageId = 0;
let testResults = [];

// 监听输出
server.stdout.on('data', (data) => {
  const lines = data.toString().trim().split('\n');
  lines.forEach(line => {
    try {
      const response = JSON.parse(line);
      handleResponse(response);
    } catch (e) {
      // 忽略非 JSON 输出
    }
  });
});

server.stderr.on('data', (data) => {
  console.log('📡 Server:', data.toString().trim());
});

// 发送请求
function sendRequest(method, params, testName) {
  messageId++;
  const request = {
    jsonrpc: '2.0',
    id: messageId,
    method: method,
    params: params
  };
  
  server.stdin.write(JSON.stringify(request) + '\n');
  console.log(`→ 发送: ${method} [${testName}]`);
  
  return new Promise((resolve) => {
    const timeout = setTimeout(() => {
      console.log(`❌ 超时: ${testName}`);
      testResults.push({ test: testName, status: 'FAIL', reason: '超时' });
      resolve(false);
    }, 3000);
    
    const handler = (response) => {
      if (response.id === messageId) {
        clearTimeout(timeout);
        server.stdout.removeListener('data', dataHandler);
        
        if (response.error) {
          console.log(`❌ 失败: ${testName}`);
          console.log(`   错误: ${response.error.message}`);
          testResults.push({ test: testName, status: 'FAIL', reason: response.error.message });
          resolve(false);
        } else {
          console.log(`✅ 成功: ${testName}`);
          testResults.push({ test: testName, status: 'PASS' });
          resolve(true);
        }
      }
    };
    
    const dataHandler = (data) => {
      const lines = data.toString().trim().split('\n');
      lines.forEach(line => {
        try {
          const response = JSON.parse(line);
          handler(response);
        } catch (e) {}
      });
    };
    
    server.stdout.on('data', dataHandler);
  });
}

// 处理响应
function handleResponse(response) {
  // 已在上层处理
}

// 执行测试
async function runTests() {
  console.log('\n========================================');
  console.log('  MCP Server 完整连接测试');
  console.log('========================================\n');
  
  // 等待服务器启动
  await new Promise(resolve => setTimeout(resolve, 500));
  
  // 测试 1: initialize
  console.log('【测试 1/4】初始化连接...');
  await sendRequest('initialize', {
    protocolVersion: '2024-11-05',
    capabilities: {},
    clientInfo: { name: 'workbuddy-test', version: '1.0' }
  }, '初始化');
  
  // 发送 initialized 通知
  messageId++;
  server.stdin.write(JSON.stringify({
    jsonrpc: '2.0',
    method: 'notifications/initialized',
    params: {}
  }) + '\n');
  
  await new Promise(resolve => setTimeout(resolve, 200));
  
  // 测试 2: tools/list
  console.log('\n【测试 2/4】获取工具列表...');
  const toolsListSuccess = await sendRequest('tools/list', {}, '工具列表');
  
  if (toolsListSuccess) {
    console.log('   应该返回 8 个工具');
  }
  
  // 测试 3: 实际调用工具 (aicodclaw_shell)
  console.log('\n【测试 3/4】调用工具 (aicodclaw_shell)...');
  const callSuccess = await sendRequest('tools/call', {
    name: 'aicodclaw_shell',
    arguments: {
      command: 'echo "MCP test working"',
      timeout: 5000
    }
  }, '工具调用');
  
  // 测试 4: 调用文件读取工具
  console.log('\n【测试 4/4】调用工具 (aicodclaw_file_read)...');
  await sendRequest('tools/call', {
    name: 'aicodclaw_file_read',
    arguments: {
      path: 'f:\\ai\\codes\\github\\deepseektuizh\\README.md',
      maxLines: 10
    }
  }, '文件读取');
  
  // 输出结果
  console.log('\n========================================');
  console.log('  测试结果');
  console.log('========================================\n');
  
  const passed = testResults.filter(r => r.status === 'PASS').length;
  const total = testResults.length;
  
  testResults.forEach((result, i) => {
    const icon = result.status === 'PASS' ? '✅' : '❌';
    const reason = result.reason ? ` - ${result.reason}` : '';
    console.log(`${icon} 测试 ${i + 1}/${total}: ${result.test}${reason}`);
  });
  
  console.log(`\n📊 通过率: ${passed}/${total} (${Math.round(passed/total*100)}%)`);
  
  if (passed === total) {
    console.log('\n✅ MCP Server 完全正常！');
    console.log('   问题可能在 WorkBuddy 配置或连接方式');
  } else {
    console.log('\n❌ MCP Server 存在问题');
    console.log('   需要修复后才能被 WorkBuddy 调用');
  }
  
  console.log('\n========================================\n');
  
  // 关闭服务器
  server.kill();
  process.exit(0);
}

runTests().catch(err => {
  console.error('测试失败:', err);
  server.kill();
  process.exit(1);
});
