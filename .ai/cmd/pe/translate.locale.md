---
argument-hint: [ locale_markdown_file ] [ translation_description ]
allowed-tools: Read, Write, Glob, Grep, Bash
description: 将中文本地化记忆提示词文件翻译为英文记忆提示词文件，保持术语与质量标准一致
---

将中文本地化记忆提示词文件 #$1 (.locale.md) 翻译为英文记忆提示词文件，同时维持既定质量标准与术语一致性。

# 任务执行流程
## [STEP-1] **解析输出路径**
**优先匹配特殊路径**，并依据下表生成目标文件：

| 源文件路径                                            | 输出文件路径                                                                |
|--------------------------------------------------|-----------------------------------------------------------------------|
| [.ai/locale/](/.ai/locale) `**/*.locale.md`      | `**/*.md`                                                             |
| [.ai/locale/](/.ai/locale) `**/AGENTS.locale.md` | `<relative_path>/AGENTS.md`, `<relative_path>/CLAUDE.md`              |
| [.ai/locale/](/.ai/locale) `AGENTS.locale.md`    | `AGENTS.md`, `CLAUDE.md`                                              |
| [.ai/locale/](/.ai/locale) `README.locale.md`    | `README.md`                                                           |
| [.ai/cmd/](/.ai/cmd) `**/*.locale.md`            | `.claude/commands/**/*.md`, `.ai/out/.claude/commands/**/*.md`        |
| [.ai/sa/](.ai/sa) `**/*.locale.md`               | `.claude/subagents/**/*.md`, `.ai/out/.claude/subagents/**/*.md`      |
| [.ai/user/](.ai/user) `**/*.locale.md`           | `~/.claude/CLAUDE.md`, `~/.codex/AGENTS.md`, `.ai/out/global/**/*.md` |
| [.ai/meta/](.ai/meta) `**/*.locale.md`           | `.ai/meta/**/*.md`                                                    |

当未命中特殊路径映射时，套用通用规则：`filename.locale.extension -> filename.extension`。

其中 `<relative_path>` 表示源文件去除 `.ai/locale/` 前缀后的目录结构。
```xml
<Example description="">.ai/locale/templates/AGENTS.locale.md -> [templates/AGENTS.md, templates/CLAUDE.md]</Example>
```

## [STEP-2] **检查目标文件**
- 使用 `Glob(pattern: "<target_file>")` 判断目标文件是否已存在
- 使用 `Bash(command: "mkdir -p <target_directory>")` 创建所有需要的目标目录

## [STEP-3] **删除旧文件**
- 若目标文件已存在，则调用 `Bash(command: "rm <target_file>")` 清理，确保后续写入干净

## [STEP-4] **读取源文件**
- 调用 `Read($1)` 获取源文件内容

## [STEP-5] **执行翻译**
- 保留 Markdown 结构与格式
- 维持代码块内容不变，仅翻译其中的中文注释或说明

## [STEP-6] **写入目标文件**
- 创建新的目标文件并写入翻译结果
- 若存在多个目标路径，先写入第一个文件，再使用 `Bash(command: "cp -R <first_file> <target_file>")` 复制至剩余路径，避免偏差

## [STEP-7] **错误处理**
- 如 `Write` 调用失败，立即执行 `Bash(command: "rm <target_file>")` 清理
- 清理后重新从步骤 1 开始，拒绝局部修补

# 质量标准
- **术语一致**：逐条对照术语表，确保大小写和标点完全吻合
- **技术准确**：核对命令、参数、文件路径等技术细节，防止引入新含义
- **格式保持**：保留标题层级、列表缩进、表格和行内代码，禁止增删空行
- **空白忠实**：严格保留原有空格与空行，它们也是提示词的一部分
- **链接同步**：根据目标文档结构更新相对或绝对路径，保证锚点与文件名同步
- **代码完整**：除翻译注释外保持代码块原样，并确认语句缩进正确

```xml
<Examples description="文件路径转换">
  <Example>.ai/cmd/pe/translate.locale.md -> [.claude/commands/pe/translate.md, .ai/out/.claude/commands/pe/translate.md]</Example>
  <Example>.ai/cmd/pe/setup.locale.md -> [.claude/commands/pe/setup.md, .ai/out/.claude/commands/pe/setup.md]</Example>
  <Example>.ai/user/cc.locale.md -> [.ai/out/GLOBAL/cc.md, ~/.claude/CLAUDE.md, ~/.codex/AGENTS.md]</Example>
  <Example>.ai/user/USER_AGENTS.locale.md -> [.ai/out/GLOBAL/USER_AGENTS.md, ~/.claude/CLAUDE.md, ~/.codex/AGENTS.md]</Example>
  <Example>.ai/locale/AGENTS.locale.md -> [AGENTS.md, CLAUDE.md]</Example>
  <Example>.ai/locale/templates/AGENTS.locale.md -> [templates/AGENTS.md, templates/CLAUDE.md]</Example>
  <Example>.ai/locale/README.locale.md -> README.md</Example>
  <Example>.ai/locale/.ai/cmd/AGENTS.locale.md -> [.ai/cmd/AGENTS.md, .ai/cmd/CLAUDE.md]</Example>
  <Example>.ai/locale/meta/examples.locale.md -> .ai/meta/examples.md</Example>
  <Example>.ai/locale/meta/prompt.locale.md -> .ai/meta/prompt.md</Example>
  <Example>.ai/locale/meta/AGENTS.locale.md -> [.ai/meta/AGENTS.md, .ai/meta/CLAUDE.md]</Example>
</Examples>
```
