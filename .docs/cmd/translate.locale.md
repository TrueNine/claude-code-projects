---
argument-hint: [ locale_markdown_file ] [ translation_description ]
allowed-tools: Read, Write, Glob, Grep, Bash
description: 将中文本地化记忆提示词文件翻译为英文记忆提示词文件，遵循一致的术语和质量标准
---

将中文本地化记忆提示词文件 #$1 (.locale.md) 翻译为英文记忆提示词文件，同时保持质量标准和术语一致性。

# 任务执行流程

1. **解析文件名**:
  - **特殊位置规则** (优先检查):
    - `.docs/cmd/**.locale.md` -> `.claude/commands/**.md`
    - `.docs/sa/**.locale.md` -> `.claude/agents/**.md`
    - `.docs/CLAUDE-cmd.locale.md` -> `.docs/cmd/CLAUDE.md`
    - `.docs/CLAUDE-sa.locale.md` -> `.docs/sa/CLAUDE.md`
    - `.docs/CLAUDE-user.locale.md` -> `.docs/user/CLAUDE.md`
    - `.docs/CLAUDE-project.locale.md` -> `.docs/project/CLAUDE.md`
    - `.docs/CLAUDE.locale.md` -> `.docs/CLAUDE.md`
  - **标准规则**: `filename.locale.extension` -> `filename.extension`

2. **检查目标文件**:
  - 使用 `Search(pattern: "$1")` 验证目标文件是否存在
  - 模式: 基于步骤 2 确定的目标路径

3. **删除现有文件**:
  - 如果目标文件存在，使用 Bash 工具删除
  - 命令: `Bash(rm filename.extension)` (Linux/Mac) 或 等价 (Windows) 命令

4. **读取源文件**: `Read($1)`

5. **执行翻译**:
  - 保留 Markdown 结构和格式
  - 应用词汇表中的一致术语
  - 保持代码块不变, 且翻译所有注释内容

6. **写入目标文件**:
  - 创建新的目标文件并写入翻译内容
  - 无需读取现有目标文件 (已在步骤 4 中删除)

7. **错误处理**:
  - 如果 `Write` 失败，立即删除目标文件
  - 使用 `Bash(rm target_file)` 执行删除
  - 重新开始流程，不尝试修复



## 质量标准

- **术语一致性**: 严格遵循词汇表
- **技术准确性**: 保持技术概念的精确性
- **格式保持**: 保留所有 Markdown 格式
- **链接处理**: 适当更新文档链接
- **代码完整性**: 保持代码示例不变

```xml
<Example description="文件名转换">
  - `translate.locale.md` → `translate.md`
  - `setup.locale.md` → `setup.md`
  - `config.locale.yaml` → `config.yaml`
</Example>
```

```xml
<Examples description="翻译示范">
  <GoodExample userInput="请参阅文档">
    See documentation
  </GoodExample>
  <GoodExample userInput="配置文件">
    Configuration file
  </GoodExample>
  <GoodExample userInput="命令行工具">
    Command-line tool
  </GoodExample>
</Examples>
```
