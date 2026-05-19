/**
 * AICodeClaw CI Server v2.0
 * 
 * 企业级 CI 全自动开发工具
 * 专门针对教育项目（课本/题库/语音/视频）优化
 * 支持：自动开发 → 自动构建 → 自动部署
 */

import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  readFileSync,
  writeFileSync,
  existsSync,
  statSync,
  mkdirSync,
  readdirSync,
  copyFileSync
} from "fs";
import { join, resolve, dirname } from "path";
import { exec } from "child_process";
import { promisify } from "util";
import { z } from "zod";

const execAsync = promisify(exec);
const isWindows = process.platform === "win32";

// ============================================
// 环境变量配置
// ============================================
const API_URL = process.env.LLM_API_URL || "http://192.168.2.5:1234/v1/chat/completions";
const API_KEY = process.env.LLM_API_KEY || "";
const DEFAULT_MODEL = process.env.LLM_MODEL || "qwen3.5-9b-deepseek-v4-flash@q6_k";

// ============================================
// 辅助函数：智能 JSON 提取
// ============================================
function extractJson(content) {
  if (!content) {
    throw new Error("AI 返回内容为空");
  }

  // 1. 尝试直接解析
  try {
    return JSON.parse(content);
  } catch (e) {
    // 继续尝试其他方法
  }

  // 2. 尝试提取 ```json 代码块
  const jsonMatch = content.match(/```json\s*([\s\S]*?)```/);
  if (jsonMatch) {
    try {
      return JSON.parse(jsonMatch[1].trim());
    } catch (e) {
      // 继续尝试
    }
  }

  // 3. 尝试提取任意 ``` 代码块
  const codeMatch = content.match(/```\s*([\s\S]*?)```/);
  if (codeMatch) {
    try {
      return JSON.parse(codeMatch[1].trim());
    } catch (e) {
      // 继续尝试
    }
  }

  // 4. 尝试清理 Markdown 标记后解析
  try {
    const cleaned = content
      .replace(/```json/g, '')
      .replace(/```/g, '')
      .trim();
    return JSON.parse(cleaned);
  } catch (e) {
    // 所有方法都失败
  }

  throw new Error("无法从 AI 响应中提取有效 JSON");
}

// ============================================
// 辅助函数：安全检查
// ============================================
function validateFilePath(filePath) {
  // 防止路径穿越攻击
  if (filePath.includes("..")) {
    throw new Error(`非法路径（包含 ..）: ${filePath}`);
  }
  
  // 防止绝对路径
  if (filePath.startsWith("/") || filePath.includes(":\\") && !filePath.startsWith("C:\\")) {
    throw new Error(`非法路径（绝对路径）: ${filePath}`);
  }
  
  return true;
}

// ============================================
// 辅助函数：代码清洗
// ============================================
function cleanCode(code) {
  if (!code) return "";
  
  // 移除开头的 ```xxx 标记
  code = code.replace(/^```[a-zA-Z]*\n?/, "");
  
  // 移除结尾的 ``` 标记
  code = code.replace(/```\s*$/, "");
  
  // 去除首尾空白
  code = code.trim();
  
  return code;
}

// ============================================
// 主服务
// ============================================
const server = new McpServer({
  name: "aicodclaw-ci",
  version: "2.0.0",
  description: "AICodeClaw CI 全自动开发工具"
});

// ============================================
// 工具 1：CI 一键全自动开发（核心！）
// ============================================
server.tool(
  "aicodclaw_ci_autodev",
  "企业级 CI 全自动开发：读取任务 → 分析 → 写代码 → 构建 → 部署",
  {
    taskPath: z.string().describe("任务文件路径（.md/.xlsx）"),
    workDir: z.string().describe("项目根目录"),
    deploy: z.boolean().default(false).describe("是否自动部署"),
    format: z.boolean().default(true).describe("是否自动格式化")
  },
  async ({ taskPath, workDir, deploy, format }) => {
    const startTime = Date.now();
    const logs = [];
    const errors = [];
    let successFiles = 0;

    const log = (msg) => {
      const t = new Date().toISOString();
      logs.push(`[${t}] CI → ${msg}`);
      console.error(`[CI] ${msg}`);
    };

    try {
      // 1. 验证目录
      const root = resolve(workDir);
      if (!existsSync(root)) {
        return { content: [{ type: "text", text: `❌ 目录不存在: ${root}` }], isError: true };
      }
      
      const stats = statSync(root);
      if (!stats.isDirectory()) {
        return { content: [{ type: "text", text: `❌ 路径不是目录: ${root}` }], isError: true };
      }
      
      log(`✅ 工作目录: ${root}`);

      // 2. 读取任务
      if (!existsSync(taskPath)) {
        return { content: [{ type: "text", text: `❌ 任务文件不存在: ${taskPath}` }], isError: true };
      }
      
      const taskContent = readFileSync(taskPath, "utf-8");
      log(`✅ 任务读取完成：${taskContent.length} 字符`);

      // 3. 分析项目
      log("🔍 开始分析项目结构...");
      try {
        const { stdout: tree } = await execAsync(
          isWindows ? `tree /F /A "${root}"` : `ls -la "${root}"`,
          { cwd: root, maxBuffer: 10 * 1024 * 1024 }
        );
        log("✅ 项目结构分析完成");
      } catch (err) {
        log(`⚠️ 项目结构分析失败: ${err.message}`);
      }

      // 4. 生成开发计划
      log("🤖 AI 生成开发计划...");
      
      const planResponse = await fetch(API_URL, {
        method: "POST",
        headers: { 
          "Content-Type": "application/json",
          ...(API_KEY && { "Authorization": `Bearer ${API_KEY}` })
        },
        body: JSON.stringify({
          model: DEFAULT_MODEL,
          messages: [
            { 
              role: "system", 
              content: "你是资深前端架构师，擅长 UniApp/Vue 项目开发。请输出严格JSON格式的开发计划。" 
            },
            { 
              role: "user", 
              content: `
项目目录：${root}
任务文档：
${taskContent.substring(0, 6000)}

请分析任务并输出 JSON 格式的开发计划：
{
  "project": "项目名称",
  "techStack": ["技术栈"],
  "files": [
    { 
      "path": "文件相对路径", 
      "prompt": "详细的代码生成指令，包含功能要求、技术规范、样式要求" 
    }
  ]
}

注意：
1. files 数组包含所有需要创建/修改的文件
2. 每个 prompt 要足够详细，包含完整的功能描述
3. 优先创建核心页面和组件
4. 只输出 JSON，不要其他内容
` 
            }
          ],
          max_tokens: 4096,
          temperature: 0.1
        })
      });

      if (!planResponse.ok) {
        throw new Error(`AI API 错误: ${planResponse.status} ${planResponse.statusText}`);
      }

      const planData = await planResponse.json();
      const content = planData.choices?.[0]?.message?.content;
      
      if (!content) {
        log(`❌ AI 返回内容为空`);
        log(`完整响应: ${JSON.stringify(planData).substring(0, 500)}`);
        throw new Error("AI 计划生成失败：空响应");
      }

      let planJson;
      try {
        planJson = extractJson(content);
      } catch (e) {
        log(`❌ JSON 解析失败: ${e.message}`);
        log(`原始内容预览: ${content.substring(0, 300)}`);
        throw new Error(`AI 返回的格式无法解析: ${e.message}`);
      }

      const files = planJson.files;
      if (!Array.isArray(files)) {
        throw new Error("计划中未找到有效的文件列表（files 数组）");
      }
      
      log(`✅ 计划生成完成：共 ${files.length} 个文件`);
      log(`📋 项目名称: ${planJson.project || '未命名'}`);

      // 5. CI 批量生成 + 写入文件
      log("🚀 开始 CI 批量开发...");
      
      for (let i = 0; i < files.length; i++) {
        const f = files[i];
        log(`[${i + 1}/${files.length}] 生成 → ${f.path}`);
        
        try {
          // 安全检查：验证文件路径
          validateFilePath(f.path);
          
          // 生成代码
          const codeRes = await fetch(API_URL, {
            method: "POST",
            headers: { 
              "Content-Type": "application/json",
              ...(API_KEY && { "Authorization": `Bearer ${API_KEY}` })
            },
            body: JSON.stringify({
              model: DEFAULT_MODEL,
              messages: [
                { 
                  role: "system", 
                  content: "你是专业的前端工程师，擅长 Vue 3 + UniApp 开发。请输出完整可运行的代码，不包含任何解释。" 
                },
                { 
                  role: "user", 
                  content: `
项目信息：
- 名称: ${planJson.project || '未命名'}
- 技术栈: ${planJson.techStack?.join(', ') || '未指定'}

文件路径: ${f.path}

开发要求:
${f.prompt}

技术规范:
- 使用 Vue 3 Composition API (<script setup>)
- TypeScript 优先
- 样式使用 SCSS + BEM 规范
- 包含加载状态、错误状态、空状态处理
- 组件命名使用 PascalCase
- 响应式布局（使用 rpx 单位）

请直接输出完整代码，不要解释。
` 
                }
              ],
              max_tokens: 6000,
              temperature: 0.2
            })
          });

          if (!codeRes.ok) {
            throw new Error(`AI API 错误: ${codeRes.status} ${codeRes.statusText}`);
          }

          const codeData = await codeRes.json();
          let code = codeData.choices?.[0]?.message?.content || "";

          if (!code) {
            throw new Error("AI 返回代码为空");
          }

          // 清洗代码
          code = cleanCode(code);

          if (!code || code.length < 10) {
            throw new Error("AI 生成的代码太短，可能失败");
          }

          // 写入文件
          const target = join(root, f.path);
          mkdirSync(dirname(target), { recursive: true });
          writeFileSync(target, code, "utf-8");
          
          const fileSize = (code.length / 1024).toFixed(1);
          log(`✅ 成功: ${f.path} (${fileSize}KB)`);
          successFiles++;

        } catch (e) {
          errors.push({ file: f.path, err: e.message });
          log(`❌ 失败: ${f.path} → ${e.message}`);
        }
      }

      // 6. 格式化代码
      if (format) {
        log("🎨 CI → 自动格式化代码...");
        try {
          await execAsync(
            isWindows 
              ? `npx prettier --write "**/*.{vue,js,ts,scss,css}"`
              : `npx prettier --write "**/*.{vue,js,ts,scss,css}"`,
            { cwd: root, timeout: 60000 }
          );
          log("✅ 格式化完成");
        } catch (err) {
          log(`⚠️ 格式化失败: ${err.message}`);
        }
      }

      // 7. 构建项目
      log("🔨 CI → 自动构建...");
      try {
        await execAsync("npm run build", { cwd: root, timeout: 120000 });
        log("✅ 构建成功");
      } catch (err) {
        log(`⚠️ 构建失败: ${err.message}`);
      }

      // 8. 部署（可选）
      if (deploy) {
        log("🚀 CI → 自动部署到 Edge Functions...");
        try {
          await execAsync("npx supabase functions deploy --no-verify-jwt", { 
            cwd: root, 
            timeout: 120000 
          });
          log("✅ 部署成功");
        } catch (err) {
          log(`⚠️ 部署失败: ${err.message}`);
        }
      }

      // 9. 生成构建报告
      const cost = ((Date.now() - startTime) / 1000).toFixed(1);
      const successRate = files.length > 0 ? Math.round(successFiles / files.length * 100) : 0;
      
      const report = `
# 🚀 AICodeClaw CI 构建完成

## 📊 基本信息
- **项目名称**: ${planJson.project || '未命名'}
- **技术栈**: ${planJson.techStack?.join(', ') || '未指定'}
- **工作目录**: ${root}
- **任务文件**: ${taskPath}
- **耗时**: ${cost} 秒

## ✅ 开发统计
- **总文件数**: ${files.length}
- **成功文件**: ✅ ${successFiles}
- **失败文件**: ❌ ${errors.length}
- **成功率**: ${successRate}%

## ❌ 错误详情
${errors.length === 0 ? "✅ 无错误" : errors.map((e, i) => `
### 错误 ${i + 1}
- **文件**: ${e.file}
- **错误**: ${e.err}
`).join('\n')}

## 📝 构建日志（最近 30 条）
${logs.slice(-30).join('\n')}

---
**生成时间**: ${new Date().toISOString()}
**AICodeClaw CI v2.0**
      `.trim();

      const reportPath = join(root, "CI-BUILD-REPORT.md");
      writeFileSync(reportPath, report, "utf-8");
      log(`📄 报告已生成: ${reportPath}`);

      return {
        content: [{ type: "text", text: report }]
      };

    } catch (err) {
      log(`❌ CI 崩溃: ${err.message}`);
      return {
        content: [{ 
          type: "text", 
          text: `❌ CI 系统崩溃\n\n错误: ${err.message}\n\n日志:\n${logs.join('\n')}` 
        }],
        isError: true
      };
    }
  }
);

// ============================================
// 工具 2：批量执行 YOLO
// ============================================
server.tool(
  "aicodclaw_ci_batch",
  "CI 批量执行文件操作、命令",
  {
    cwd: z.string().describe("工作目录"),
    operations: z.array(z.object({
      type: z.enum(["write", "read", "shell", "copy"]),
      path: z.string().describe("文件路径"),
      content: z.string().optional().describe("写入内容"),
      command: z.string().optional().describe("Shell 命令"),
      target: z.string().optional().describe("复制目标")
    })).describe("操作列表")
  },
  async ({ cwd, operations }) => {
    const results = [];
    const root = resolve(cwd);
    
    for (const op of operations) {
      try {
        const full = join(root, op.path);
        
        if (op.type === "write") {
          mkdirSync(dirname(full), { recursive: true });
          writeFileSync(full, op.content || '', "utf-8");
          results.push(`✅ write ${op.path}`);
        }
        
        if (op.type === "read") {
          if (!existsSync(full)) {
            results.push(`❌ read ${op.path} → 文件不存在`);
            continue;
          }
          const content = readFileSync(full, "utf-8");
          results.push(`📖 read ${op.path}: ${content.slice(0, 200)}${content.length > 200 ? '...' : ''}`);
        }
        
        if (op.type === "shell") {
          const adaptedCmd = isWindows 
            ? op.command?.replace(/&&/g, ';').replace(/\bcat\b/g, 'type').replace(/\bls\b/g, 'dir')
            : op.command;
          await execAsync(adaptedCmd, { cwd: root, timeout: 30000 });
          results.push(`🖥️ shell ${op.command}`);
        }
        
        if (op.type === "copy" && op.target) {
          if (!existsSync(full)) {
            results.push(`❌ copy ${op.path} → 源文件不存在`);
            continue;
          }
          copyFileSync(full, join(root, op.target));
          results.push(`📋 copy ${op.path} → ${op.target}`);
        }
        
      } catch (e) {
        results.push(`❌ ${op.type} ${op.path} → ${e.message}`);
      }
    }
    
    return {
      content: [{ 
        type: "text", 
        text: `🚀 CI 批量执行完成\n📁 工作目录: ${root}\n\n${results.join('\n')}` 
      }]
    };
  }
);

// ============================================
// 启动
// ============================================
async function start() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error("✅ AICodeClaw CI v2.0 已启动");
  console.error("📦 2个超级工具：");
  console.error("   aicodclaw_ci_autodev   一键全自动开发");
  console.error("   aicodclaw_ci_batch     批量文件操作");
}

start().catch(console.error);
