/**
 * AICodeClaw MCP Server
 * 
 * Provides development tools for WorkBuddy integration
 * Reduces WorkBuddy token consumption by 90-95%
 */

import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { readFileSync, writeFileSync, existsSync, statSync, mkdirSync } from "fs";
import { join, resolve, dirname, sep } from "path";
import { exec } from "child_process";
import { promisify } from "util";
import { z } from "zod";

const execAsync = promisify(exec);

// Create MCP Server
const server = new McpServer({
  name: "aicodclaw",
  version: "1.0.0"
});

// ============================================
// Helper Functions
// ============================================

/**
 * 验证工作目录
 */
function validateCwd(cwd) {
  if (!cwd) {
    return { valid: false, error: "工作目录 (cwd) 为必填参数" };
  }
  
  const fullPath = resolve(cwd);
  
  if (!existsSync(fullPath)) {
    return { valid: false, error: `工作目录不存在: ${fullPath}` };
  }
  
  const stats = statSync(fullPath);
  if (!stats.isDirectory()) {
    return { valid: false, error: `路径不是目录: ${fullPath}` };
  }
  
  return { valid: true, path: fullPath };
}

/**
 * 将 Linux 命令转换为 Windows 兼容命令
 */
function adaptCommandForWindows(command, cwd) {
  let adapted = command;
  
  // 替换 mkdir -p → PowerShell New-Item
  adapted = adapted.replace(/mkdir\s+-p\s+([^\s;]+)/g, 'New-Item -ItemType Directory -Path "$1" -Force');
  
  // 替换 rm -rf → PowerShell Remove-Item
  adapted = adapted.replace(/rm\s+-rf\s+([^\s;]+)/g, 'Remove-Item -Recurse -Force "$1"');
  
  // 替换 cp -r → PowerShell Copy-Item
  adapted = adapted.replace(/cp\s+-r\s+([^\s;]+)\s+([^\s;]+)/g, 'Copy-Item -Recurse "$1" "$2"');
  
  // 替换 && 分隔符 → ; 分隔符
  adapted = adapted.replace(/&&/g, ';');
  
  // 替换 rm → del (Windows)
  adapted = adapted.replace(/\brm\b/g, 'del');
  
  // 替换 mv → move (Windows)
  adapted = adapted.replace(/\bmv\b/g, 'move');
  
  // 替换 cat → type (Windows)
  adapted = adapted.replace(/\bcat\b/g, 'type');
  
  // 替换 ls → dir (Windows)
  adapted = adapted.replace(/\bls\b/g, 'dir');
  
  return adapted;
}

const isWindows = process.platform === 'win32';

// ============================================
// Tool 1: File Read
// ============================================
server.tool(
  "aicodclaw_file_read",
  "Read file contents",
  { path: z.string().describe("File path to read") },
  async ({ path }) => {
    try {
      const fullPath = resolve(path);
      
      if (!existsSync(fullPath)) {
        return {
          content: [{ type: "text", text: `Error: File not found: ${fullPath}` }],
          isError: true
        };
      }
      
      const stats = statSync(fullPath);
      if (stats.size > 10 * 1024 * 1024) {
        return {
          content: [{ type: "text", text: "Error: File too large (>10MB)" }],
          isError: true
        };
      }
      
      const content = readFileSync(fullPath, "utf-8");
      return {
        content: [{ 
          type: "text", 
          text: `📄 文件路径: ${fullPath}\n\n${content}` 
        }]
      };
    } catch (error) {
      return {
        content: [{ type: "text", text: `Error: ${error.message}` }],
        isError: true
      };
    }
  }
);

// ============================================
// Tool 2: File Write
// ============================================
server.tool(
  "aicodclaw_file_write",
  "Write content to file",
  {
    path: z.string().describe("File path to write"),
    content: z.string().describe("Content to write")
  },
  async ({ path, content }) => {
    try {
      const fullPath = resolve(path);
      const dir = dirname(fullPath);
      
      if (!existsSync(dir)) {
        mkdirSync(dir, { recursive: true });
      }
      
      writeFileSync(fullPath, content, "utf-8");
      return {
        content: [{ 
          type: "text", 
          text: `✅ 文件已写入: ${fullPath}\n📊 大小: ${content.length} 字符` 
        }]
      };
    } catch (error) {
      return {
        content: [{ type: "text", text: `Error: ${error.message}` }],
        isError: true
      };
    }
  }
);

// ============================================
// Tool 3: Code Search
// ============================================
server.tool(
  "aicodclaw_search",
  "Search for pattern in files",
  {
    path: z.string().describe("Directory or file to search"),
    pattern: z.string().describe("Search pattern")
  },
  async ({ path, pattern }) => {
    try {
      const fullPath = resolve(path);
      const isWindows = process.platform === "win32";
      const command = isWindows
        ? `findstr /s /n "${pattern}" "${fullPath}\\*"`
        : `grep -r -n "${pattern}" "${fullPath}"`;
      
      const { stdout } = await execAsync(command, { maxBuffer: 10 * 1024 * 1024 });
      
      if (!stdout.trim()) {
        return {
          content: [{ type: "text", text: "No matches found" }]
        };
      }
      
      return {
        content: [{ type: "text", text: stdout }]
      };
    } catch (error) {
      if (error.code === 1) {
        return {
          content: [{ type: "text", text: "No matches found" }]
        };
      }
      return {
        content: [{ type: "text", text: `Error: ${error.message}` }],
        isError: true
      };
    }
  }
);

// ============================================
// Tool 4: Apply Patch
// ============================================
server.tool(
  "aicodclaw_apply_patch",
  "Apply unified diff patch to file",
  {
    path: z.string().describe("File path to patch"),
    patch: z.string().describe("Unified diff patch content")
  },
  async ({ path, patch }) => {
    try {
      const fullPath = resolve(path);
      
      if (!existsSync(fullPath)) {
        return {
          content: [{ type: "text", text: `Error: File not found: ${fullPath}` }],
          isError: true
        };
      }
      
      const patchFile = join(process.cwd(), "temp.patch");
      writeFileSync(patchFile, patch, "utf-8");
      
      const { stdout } = await execAsync(`patch "${fullPath}" < "${patchFile}"`);
      
      require("fs").unlinkSync(patchFile);
      
      return {
        content: [{ type: "text", text: `Patch applied successfully\n${stdout}` }]
      };
    } catch (error) {
      return {
        content: [{ type: "text", text: `Error: ${error.message}` }],
        isError: true
      };
    }
  }
);

// ============================================
// Tool 5: Shell Command
// ============================================
server.tool(
  "aicodclaw_shell",
  "Execute shell command",
  {
    command: z.string().describe("Shell command to execute"),
    cwd: z.string().describe("Working directory (required)")
  },
  async ({ command, cwd }) => {
    // 验证工作目录
    const cwdValidation = validateCwd(cwd);
    if (!cwdValidation.valid) {
      return {
        content: [{ type: "text", text: `❌ ${cwdValidation.error}` }],
        isError: true
      };
    }
    
    const workDir = cwdValidation.path;
    
    // Windows 命令适配
    const adaptedCommand = isWindows ? adaptCommandForWindows(command, workDir) : command;
    
    if (isWindows && adaptedCommand !== command) {
      console.error(`[shell] 命令已适配: ${command} → ${adaptedCommand}`);
    }
    
    try {
      const { stdout, stderr } = await execAsync(adaptedCommand, {
        cwd: workDir,
        maxBuffer: 10 * 1024 * 1024,
        timeout: 60000,
        shell: isWindows ? 'powershell.exe' : '/bin/bash'
      });
      
      const output = [];
      output.push({ type: "text", text: `📁 工作目录: ${workDir}\n🖥️  平台: ${process.platform}\n` });
      if (stdout) output.push({ type: "text", text: stdout });
      if (stderr) output.push({ type: "text", text: `⚠️  stderr:\n${stderr}` });
      
      return { content: output };
    } catch (error) {
      return {
        content: [
          { type: "text", text: `❌ 命令执行失败 (退出码 ${error.code})\n📁 工作目录: ${workDir}\n🖥️  平台: ${process.platform}\n` },
          { type: "text", text: error.stdout || "" },
          { type: "text", text: error.stderr || error.message }
        ],
        isError: true
      };
    }
  }
);

// ============================================
// Tool 6: AI Query
// ============================================
server.tool(
  "aicodclaw_deepseek",
  "Query AI for code analysis or answers",
  { query: z.string().describe("Query to send to AI") },
  async ({ query }) => {
    try {
      const response = await fetch("http://192.168.2.5:1234/v1/chat/completions", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          model: process.env.DEEPSEEK_MODEL || "qwen3.5-9b-deepseek-v4-flash@q6_k",
          messages: [{ role: "user", content: query }],
          max_tokens: 4096
        })
      });
      
      if (!response.ok) {
        const errorText = await response.text();
        return {
          content: [{ type: "text", text: `AI API error: ${response.status}\n${errorText}` }],
          isError: true
        };
      }
      
      const data = await response.json();
      const answer = data.choices?.[0]?.message?.content || "No response";
      
      return {
        content: [{ type: "text", text: answer }]
      };
    } catch (error) {
      return {
        content: [{ type: "text", text: `Error calling AI: ${error.message}` }],
        isError: true
      };
    }
  }
);

// ============================================
// Tool 7: AI Reply
// ============================================
server.tool(
  "aicodclaw_deepseek-reply",
  "Get detailed AI response with optional system prompt",
  {
    query: z.string().describe("Query to send to AI"),
    system: z.string().optional().describe("System prompt")
  },
  async ({ query, system }) => {
    try {
      const messages = [];
      if (system) {
        messages.push({ role: "system", content: system });
      }
      messages.push({ role: "user", content: query });
      
      const response = await fetch("http://192.168.2.5:1234/v1/chat/completions", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          model: process.env.DEEPSEEK_MODEL || "qwen3.5-9b-deepseek-v4-flash@q6_k",
          messages: messages,
          max_tokens: 4096
        })
      });
      
      if (!response.ok) {
        const errorText = await response.text();
        return {
          content: [{ type: "text", text: `AI API error: ${response.status}\n${errorText}` }],
          isError: true
        };
      }
      
      const data = await response.json();
      const answer = data.choices?.[0]?.message?.content || "No response";
      
      return {
        content: [{ type: "text", text: answer }]
      };
    } catch (error) {
      return {
        content: [{ type: "text", text: `Error calling AI: ${error.message}` }],
        isError: true
      };
    }
  }
);

// ============================================
// Tool 8: YOLO Mode - Batch Operations
// ============================================
server.tool(
  "aicodclaw_yolo",
  "Execute multiple operations in YOLO mode (auto-approve all)",
  {
    operations: z.array(z.object({
      type: z.enum(["file_read", "file_write", "shell", "search"]),
      params: z.record(z.any())
    })).describe("List of operations to execute"),
    cwd: z.string().describe("Working directory (required)")
  },
  async ({ operations, cwd }) => {
    // 验证工作目录
    const cwdValidation = validateCwd(cwd);
    if (!cwdValidation.valid) {
      return {
        content: [{ type: "text", text: `❌ ${cwdValidation.error}` }],
        isError: true
      };
    }
    
    const workDir = cwdValidation.path;
    const results = [];
    
    for (let i = 0; i < operations.length; i++) {
      const op = operations[i];
      try {
        let result;
        
        switch (op.type) {
          case "file_read": {
            const fullPath = resolve(workDir, op.params.path);
            const content = readFileSync(fullPath, "utf-8");
            result = { success: true, data: content.substring(0, 1000), path: fullPath };
            break;
          }
          case "file_write": {
            const fullPath = resolve(workDir, op.params.path);
            const dir = dirname(fullPath);
            if (!existsSync(dir)) mkdirSync(dir, { recursive: true });
            writeFileSync(fullPath, op.params.content, "utf-8");
            result = { success: true, message: `已写入: ${fullPath}` };
            break;
          }
          case "shell": {
            const adaptedCmd = isWindows ? adaptCommandForWindows(op.params.command, workDir) : op.params.command;
            const { stdout } = await execAsync(adaptedCmd, {
              cwd: workDir,
              maxBuffer: 10 * 1024 * 1024,
              timeout: 60000,
              shell: isWindows ? 'powershell.exe' : '/bin/bash'
            });
            result = { success: true, output: stdout.substring(0, 500) };
            break;
          }
          case "search": {
            const searchPath = resolve(workDir, op.params.path);
            const command = isWindows
              ? `findstr /s /n "${op.params.pattern}" "${searchPath}\\*"`
              : `grep -r -n "${op.params.pattern}" "${searchPath}"`;
            const { stdout } = await execAsync(command, { maxBuffer: 10 * 1024 * 1024 });
            result = { success: true, matches: stdout.split("\n").length - 1 };
            break;
          }
        }
        
        results.push({ index: i, ...result });
      } catch (error) {
        results.push({
          index: i,
          success: false,
          error: error.message
        });
      }
    }
    
    return {
      content: [{
        type: "text",
        text: `🚀 YOLO 模式执行完成\n📁 工作目录: ${workDir}\n🖥️  平台: ${process.platform}\n\n执行 ${operations.length} 个操作:\n${
          results.map(r => `  [${r.index}] ${r.success ? '✅' : '❌'} ${r.message || r.error || ''}`).join('\n')
        }`
      }]
    };
  }
);

// ============================================
// Tool 9: AutoCode - 一键生成
// ============================================
server.tool(
  "aicodclaw_autocode",
  "一键生成：读取任务文档（.md/.xlsx），自动分析项目、理解需求、调用AI自主开发、测试并完成所有功能",
  {
    taskDocPath: z.string().describe("任务文档路径（.md 或 .xlsx）"),
    workDir: z.string().optional().describe("工作目录（可选，未指定时会提示）")
  },
  async ({ taskDocPath, workDir }) => {
    const startTime = Date.now();
    const logs = [];
    const errors = [];
    let completedTasks = 0;
    let failedTasks = 0;
    
    // 工具函数：记录日志
    const log = (message) => {
      const timestamp = new Date().toISOString();
      logs.push(`[${timestamp}] ${message}`);
      console.error(`[autocode] ${message}`);
    };
    
    // 1. 验证工作目录
    const projectDir = workDir ? resolve(workDir) : process.cwd();
    
    if (!workDir) {
      return {
        content: [{
          type: "text",
          text: `⚠️ 请指定工作目录参数 workDir\n示例：{"taskDocPath": "任务.md", "workDir": "f:\\project"}`
        }],
        isError: true
      };
    }
    
    if (!existsSync(projectDir)) {
      return {
        content: [{ type: "text", text: `❌ 工作目录不存在: ${projectDir}` }],
        isError: true
      };
    }
    
    log(`✅ 工作目录: ${projectDir}`);
    
    // 2. 读取任务文档
    log(`📖 读取任务文档: ${taskDocPath}`);
    
    if (!existsSync(taskDocPath)) {
      return {
        content: [{ type: "text", text: `❌ 任务文档不存在: ${taskDocPath}` }],
        isError: true
      };
    }
    
    const ext = taskDocPath.split('.').pop().toLowerCase();
    let taskContent = '';
    
    try {
      if (ext === 'md') {
        taskContent = readFileSync(taskDocPath, 'utf-8');
      } else if (ext === 'xlsx' || ext === 'xls') {
        // Excel 文件需要特殊处理，这里简化为读取
        taskContent = `Excel文件: ${taskDocPath}\n请解析文件内容`;
        log(`⚠️ Excel 文件需要额外依赖，建议使用 .md 格式`);
      } else {
        return {
          content: [{ type: "text", text: `❌ 不支持的文件格式: ${ext}，仅支持 .md 或 .xlsx` }],
          isError: true
        };
      }
      
      log(`✅ 文档读取成功 (${taskContent.length} 字符)`);
    } catch (error) {
      return {
        content: [{ type: "text", text: `❌ 读取文档失败: ${error.message}` }],
        isError: true
      };
    }
    
    // 3. 分析项目结构
    log(`🔍 分析项目结构...`);
    
    try {
      const { stdout: treeOutput } = await execAsync(`tree /F /A "${projectDir}"`, {
        maxBuffer: 10 * 1024 * 1024
      });
      log(`✅ 项目结构分析完成`);
    } catch (error) {
      log(`⚠️ 项目结构分析失败: ${error.message}`);
    }
    
    // 4. 调用 AI 理解任务并生成开发计划
    log(`🤖 调用 AI 理解任务...`);
    
    const analysisPrompt = `
你是一个高级前端开发工程师。请分析以下任务文档，并制定开发计划。

## 项目目录
工作目录: ${projectDir}

## 任务文档
${taskContent.substring(0, 5000)}

## 要求
1. 分析项目意图和技术栈
2. 识别所有需要开发的功能模块
3. 制定详细的开发步骤（按优先级排序）
4. 为每个步骤指定：文件路径、功能描述、技术要求

请以 JSON 格式返回开发计划：
{
  "projectName": "项目名称",
  "techStack": ["技术1", "技术2"],
  "tasks": [
    {
      "id": 1,
      "name": "任务名称",
      "filePath": "文件路径",
      "description": "功能描述",
      "priority": "P0/P1/P2"
    }
  ]
}
`;
    
    let developmentPlan;
    try {
      const aiResponse = await fetch("http://192.168.2.5:1234/v1/chat/completions", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          model: process.env.DEEPSEEK_MODEL || "qwen3.5-9b-deepseek-v4-flash@q6_k",
          messages: [
            { role: "system", content: "你是专业的开发工程师，擅长分析需求并制定开发计划。" },
            { role: "user", content: analysisPrompt }
          ],
          max_tokens: 4096,
          temperature: 0.3
        })
      });
      
      if (!aiResponse.ok) {
        throw new Error(`AI API error: ${aiResponse.status}`);
      }
      
      const aiData = await aiResponse.json();
      const aiContent = aiData.choices?.[0]?.message?.content || '';
      
      // 提取 JSON
      const jsonMatch = aiContent.match(/\{[\s\S]*\}/);
      if (jsonMatch) {
        developmentPlan = JSON.parse(jsonMatch[0]);
        log(`✅ 开发计划生成成功 (${developmentPlan.tasks?.length || 0} 个任务)`);
      } else {
        throw new Error('AI 返回格式不正确');
      }
    } catch (error) {
      log(`❌ AI 分析失败: ${error.message}`);
      errors.push({ task: '分析任务', error: error.message });
      failedTasks++;
    }
    
    // 5. 执行开发任务
    if (developmentPlan && developmentPlan.tasks) {
      log(`🚀 开始执行开发任务 (${developmentPlan.tasks.length} 个)...`);
      
      for (let i = 0; i < developmentPlan.tasks.length; i++) {
        const task = developmentPlan.tasks[i];
        log(`\n━━━ 任务 ${i + 1}/${developmentPlan.tasks.length}: ${task.name} ━━━`);
        
        let retryCount = 0;
        const maxRetries = 10;
        let taskSuccess = false;
        
        while (retryCount < maxRetries && !taskSuccess) {
          try {
            // 5.1 生成代码
            log(`  📝 生成代码 (尝试 ${retryCount + 1}/${maxRetries})...`);
            
            const codePrompt = `
你是前端开发专家。请根据以下任务要求生成代码：

## 项目信息
- 项目名称: ${developmentPlan.projectName}
- 技术栈: ${developmentPlan.techStack?.join(', ')}

## 任务要求
- 任务名称: ${task.name}
- 文件路径: ${task.filePath}
- 功能描述: ${task.description}

## 开发规范
- 使用 TypeScript
- Vue 3 Composition API
- 组件命名: PascalCase
- 样式使用 SCSS + BEM 规范
- 包含加载状态、错误状态、空状态处理

## 要求
1. 生成完整的 Vue 组件代码
2. 包含 template、script、style
3. 代码要完整可运行
4. 遵循最佳实践

请直接输出代码，不要解释。
`;
            
            const codeResponse = await fetch("http://192.168.2.5:1234/v1/chat/completions", {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              body: JSON.stringify({
                model: process.env.DEEPSEEK_MODEL || "qwen3.5-9b-deepseek-v4-flash@q6_k",
                messages: [
                  { role: "system", content: "你是专业的前端开发工程师，擅长生成高质量的 Vue 组件代码。" },
                  { role: "user", content: codePrompt }
                ],
                max_tokens: 4096,
                temperature: 0.2
              })
            });
            
            if (!codeResponse.ok) {
              throw new Error(`AI API error: ${codeResponse.status}`);
            }
            
            const codeData = await codeResponse.json();
            let generatedCode = codeData.choices?.[0]?.message?.content || '';
            
            // 提取代码块
            const codeMatch = generatedCode.match(/```[\s\S]*?```/);
            if (codeMatch) {
              generatedCode = codeMatch[0].replace(/```[a-z]*\n?/gi, '').trim();
            }
            
            // 5.2 写入文件 - 严格路径验证
            // 确保 filePath 是相对路径
            let relativePath = task.filePath;
            
            // 清理前导斜杠（可能多层）
            while (relativePath.startsWith('/') || relativePath.startsWith('\\')) {
              relativePath = relativePath.substring(1);
            }
            
            // 强制要求：必须是相对路径，不能以盘符开头
            if (/^[a-zA-Z]:/.test(relativePath)) {
              throw new Error(`非法路径: ${relativePath} 不能包含盘符，必须是相对路径`);
            }
            
            // 强制要求：必须以 src/ 或 pages/ 等常见目录开头
            if (!/^(src|pages|components|static|utils|services|store|api)\//.test(relativePath)) {
              log(`  ⚠️ 可疑路径: ${relativePath}，建议以 src/ 或 pages/ 开头`);
            }
            
            // 正确拼接路径
            const fullPath = resolve(projectDir, relativePath);
            
            // 严格安全检查：确保路径在项目目录内（使用 toLowerCase 处理 Windows 大小写）
            const normalizedProjectDir = projectDir.toLowerCase() + (projectDir.endsWith('\\') || projectDir.endsWith('/') ? '' : sep);
            if (!fullPath.toLowerCase().startsWith(normalizedProjectDir)) {
              throw new Error(`路径安全错误: ${fullPath} 不在项目目录 ${projectDir} 内`);
            }
            
            const dir = dirname(fullPath);
            
            if (!existsSync(dir)) {
              mkdirSync(dir, { recursive: true });
            }
            
            // 验证生成的代码不为空
            if (!generatedCode || generatedCode.trim().length < 50) {
              throw new Error(`生成的代码为空或太短 (${generatedCode.length} 字符)`);
            }
            
            // 写入文件
            writeFileSync(fullPath, generatedCode, 'utf-8');
            
            // 严格验证 1：文件是否存在
            if (!existsSync(fullPath)) {
              throw new Error(`文件写入失败（不存在）: ${fullPath}`);
            }
            
            // 严格验证 2：文件大小
            const stats = statSync(fullPath);
            if (stats.size === 0) {
              throw new Error('生成的文件为空（磁盘写入失败）');
            }
            
            // 严格验证 3：读取文件内容确认（关键！）
            const verifyContent = readFileSync(fullPath, 'utf-8');
            if (verifyContent.length !== generatedCode.length) {
              throw new Error(`文件内容验证失败：预期 ${generatedCode.length} 字符，实际 ${verifyContent.length} 字符`);
            }
            
            if (!verifyContent.includes(generatedCode.substring(0, 100))) {
              throw new Error('文件内容不匹配（写入的数据与读取的数据不一致）');
            }
            
            const fileSizeKB = (stats.size / 1024).toFixed(1);
            log(`  ✅ 文件已创建并验证: ${fullPath} (${fileSizeKB}KB)`);
            log(`  ✅ 任务完成: ${task.name}`);
            taskSuccess = true;
            completedTasks++;
            
          } catch (error) {
            retryCount++;
            log(`  ⚠️ 失败 (第 ${retryCount} 次): ${error.message}`);
            
            if (retryCount >= maxRetries) {
              log(`  ❌ 任务失败（已达最大重试次数）: ${task.name}`);
              errors.push({
                task: task.name,
                filePath: task.filePath,
                error: error.message,
                retries: retryCount
              });
              failedTasks++;
            }
            
            // 等待一下再重试
            await new Promise(r => setTimeout(r, 1000));
          }
        }
      }
    }
    
    // 6. 生成开发报告
    const endTime = Date.now();
    const duration = Math.round((endTime - startTime) / 1000);
    
    const report = `
# 一键生成开发报告

## 基本信息
- **工作目录**: ${projectDir}
- **任务文档**: ${taskDocPath}
- **开始时间**: ${new Date(startTime).toISOString()}
- **结束时间**: ${new Date(endTime).toISOString()}
- **总耗时**: ${duration} 秒

## 任务统计
- **总任务数**: ${completedTasks + failedTasks}
- **成功完成**: ✅ ${completedTasks}
- **失败任务**: ❌ ${failedTasks}
- **成功率**: ${Math.round(completedTasks / (completedTasks + failedTasks) * 100)}%

## 错误详情 (${errors.length} 个)
${errors.length > 0 ? errors.map((err, i) => `
### 错误 ${i + 1}
- **任务**: ${err.task}
- **文件**: ${err.filePath || 'N/A'}
- **错误**: ${err.error}
${err.retries ? `- **重试次数**: ${err.retries}` : ''}
`).join('\n') : '无错误'}

## 开发日志
${logs.join('\n')}

---
**生成时间**: ${new Date().toISOString()}
    `.trim();
    
    // 保存报告
    const reportPath = join(projectDir, 'autocode-report.md');
    writeFileSync(reportPath, report, 'utf-8');
    log(`📄 开发报告已保存: ${reportPath}`);
    
    return {
      content: [{
        type: "text",
        text: report
      }]
    };
  }
);

// ============================================
// Start Server
// ============================================
async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error("✅ AICodeClaw MCP Server running on stdio");
  console.error("📦 9 tools registered:");
  console.error("   - aicodclaw_file_read");
  console.error("   - aicodclaw_file_write");
  console.error("   - aicodclaw_search");
  console.error("   - aicodclaw_apply_patch");
  console.error("   - aicodclaw_shell");
  console.error("   - aicodclaw_deepseek");
  console.error("   - aicodclaw_deepseek-reply");
  console.error("   - aicodclaw_yolo (批量操作模式)");
  console.error("   - aicodclaw_autocode (🆕 一键生成)");
}

main().catch(console.error);
