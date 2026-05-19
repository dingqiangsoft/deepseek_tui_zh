/**
 * AICodeClaw MCP Server 全面测试脚本
 * 测试所有工具的可用性
 */

const { spawn } = require('child_process');
const path = require('path');

const MCP_SERVER = path.join(__dirname, 'index.js');
let messageId = 0;

// 测试用例
const tests = [
  {
    name: 'aicodclaw_file_read',
    tool: 'aicodclaw_file_read',
    args: { path: path.join(__dirname, '..', 'test-autocode-task.md') },
    validate: (result) => {
      const text = result.content?.[0]?.text || '';
      return text.includes('测试任务') && text.length > 100;
    }
  },
  {
    name: 'aicodclaw_file_write',
    tool: 'aicodclaw_file_write',
    args: { 
      path: path.join(__dirname, 'test-output.txt'), 
      content: 'Test content - ' + new Date().toISOString() 
    },
    validate: (result) => {
      const text = result.content?.[0]?.text || '';
      return text.includes('已写入') || text.includes('written');
    }
  },
  {
    name: 'aicodclaw_shell',
    tool: 'aicodclaw_shell',
    args: { 
      command: 'echo "Shell test"', 
      cwd: __dirname 
    },
    validate: (result) => {
      // shell 工具可能返回多个 content 元素
      const allText = result.content?.map(c => c.text).join('\n') || '';
      return allText.includes('Shell test') || allText.includes('工作目录');
    }
  }
];

async function runTest(test) {
  console.log(`\n🧪 测试: ${test.name}`);
  
  return new Promise((resolve) => {
    const server = spawn('node', [MCP_SERVER], {
      stdio: ['pipe', 'pipe', 'pipe']
    });

    let output = '';
    let testPassed = false;

    server.stdout.on('data', (data) => {
      output += data.toString();
      
      // 检查初始化响应
      if (output.includes('"id":1') && !output.includes('initialized')) {
        // 发送工具调用
        const callRequest = {
          jsonrpc: '2.0',
          id: 2,
          method: 'tools/call',
          params: {
            name: test.tool,
            arguments: test.args
          }
        };
        server.stdin.write(JSON.stringify(callRequest) + '\n');
      }
      
      // 检查工具调用响应
      if (output.includes('"id":2')) {
        try {
          const lines = output.split('\n');
          const responseLine = lines.find(l => l.includes('"id":2'));
          if (responseLine) {
            const response = JSON.parse(responseLine);
            const result = response.result;
            
            if (result.isError) {
              console.log(`  ❌ 失败: ${result.content?.[0]?.text || 'Unknown error'}`);
              testPassed = false;
            } else if (test.validate(result)) {
              console.log(`  ✅ 通过`);
              testPassed = true;
            } else {
              console.log(`  ❌ 验证失败`);
              testPassed = false;
            }
          }
        } catch (e) {
          console.log(`  ❌ 解析错误: ${e.message}`);
          testPassed = false;
        }
        
        server.kill();
        resolve(testPassed);
      }
    });

    server.stderr.on('data', (data) => {
      // 忽略 stderr
    });

    server.on('error', (err) => {
      console.log(`  ❌ 进程错误: ${err.message}`);
      resolve(false);
    });

    // 发送初始化请求
    const initRequest = {
      jsonrpc: '2.0',
      id: 1,
      method: 'initialize',
      params: {
        protocolVersion: '2024-11-05',
        capabilities: {},
        clientInfo: {
          name: 'test-script',
          version: '1.0.0'
        }
      }
    };
    server.stdin.write(JSON.stringify(initRequest) + '\n');

    // 10 秒超时
    setTimeout(() => {
      console.log(`  ⏱️ 超时`);
      server.kill();
      resolve(false);
    }, 10000);
  });
}

async function main() {
  console.log('🚀 AICodeClaw MCP Server 全面测试');
  console.log('='.repeat(50));
  
  let passed = 0;
  let failed = 0;

  for (const test of tests) {
    const success = await runTest(test);
    if (success) {
      passed++;
    } else {
      failed++;
    }
  }

  console.log('\n' + '='.repeat(50));
  console.log(`📊 测试结果: ${passed} 通过, ${failed} 失败`);
  
  if (failed === 0) {
    console.log('✅ 所有测试通过！');
  } else {
    console.log(`❌ 有 ${failed} 个测试失败，需要修复`);
  }
  
  process.exit(failed > 0 ? 1 : 0);
}

main().catch(console.error);
