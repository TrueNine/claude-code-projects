---
argument-hint: [locale_file_at_path]
allowed-tools: Read, Write, Edit, Glob, Grep, Bash
description: Optimize Claude Code memory prompt files to make them more readable and achieve better results
---

`optimize-prompt` is a task that optimizes Claude.ai `$1` memory prompt files. Parameters: `$1` (required): path to the prompt file to optimize, `$2` (optional): user's specific requirements or optimization direction.

This task performs structural optimization, format standardization, and content simplification of memory prompts according to established rules. For non-`**.locale.*` files, it will first translate them into Chinese `**.locale.*` files and then optimize them, ensuring users can always understand and control the prompt content.

## Optimization Rules

### Language Selection Rules
- **Prioritize processing `**.locale.*` files**: When the filename contains `**.locale.*`, execute the process directly
- **Translate non-.locale. files**: If the user provides a non-`**.locale.*` file, translate it into a Chinese `**.locale.*` file alongside it, then optimize the translated `**.locale.*` file
- **Maintain user's prompt perception capability**: Through automatic translation and optimization mechanisms, ensure users can always understand and control prompt content

### Brief Description at Document Beginning
- The first paragraph of a memory prompt file should contain a 2-5 sentence description of the entire document content
- Be concise and direct, avoid artistic expression

### Heading Structure Optimization
- Heading nesting levels must not exceed level 3
- Remove redundant heading nesting
- Ensure clear heading hierarchy

### Content Expression Standards
- **Prohibit emoji usage**: Documents strictly prohibit any emoji (emoji) to maintain professionalism
- Use concise and clear written expression
- Maintain document style consistency and professionalism

### Example Writing Standards

**XML Tag System**
- Use structured `XML` tags to wrap examples for easy parsing and reuse
- Place entire example in ` ```xml … ``` ` code block to maintain consistent format
- Add appropriate attributes to tags, such as `description="…"`, `userInput="…"`

**Tag Type Definitions**
- `<Example>`: General example, used to show standard practices
- `<GoodExample>`: Positive example, can only appear in `<Examples>`
- `<BadExample>`: Negative example, can only appear in `<Examples>`
- `<Examples>`: Example collection container, used to combine `<GoodExample>` and `<BadExample>`
- `<Thinking>`: Describes thinking process, can only appear in `<Example>`, `<GoodExample>`, `<BadExample>`

**Attribute Usage Standards**
- `description="…"`: Optional attribute, used to supplement example description, can only be used in `<Example>`, `<GoodExample>`, `<BadExample>`
- `userInput="…"`: Optional attribute, used to show user input in examples, can only be used in `<Example>`, `<GoodExample>`, `<BadExample>`

**Example Atomicity Principle**
- Each example must be atomic, it is strongly not recommended to write comments within examples, nor to include multiple code segments or discontinuous output in one example.
- **Single concept**: Each example shows only one clear technical concept or practice
- **No comments**: Examples do not contain any explanatory comments
- **Single code segment**: Each example contains only one continuous code segment
- **Independent and complete**: The example itself should be complete, not dependent on additional explanation

```xml
<Examples>
  <GoodExample>
    fn process_data(data: &str) -> Result<ProcessedData, Error> {
      parse_data(data)
    }
  </GoodExample>

  <BadExample>
    // Using Result type to handle errors
    fn process_data(data: &str) -> Result<ProcessedData, Error> {
      parse_data(data)
    }

    // Can also use Option to handle optional values - violates single concept principle
    fn get_optional_value() -> Option<String> {
      Some("value".to_string())
    }
  </BadExample>
</Examples>
```

### Core Structural Elements
- **Role definition**: Clearly define AI's identity and professional background
- **Task description**: Clearly and specifically describe the task to be completed
- **Constraint conditions**: Clearly define limitations and requirements
- **Output format**: Specify the structure and format of output

### Attention Mechanism Optimization
- **Key point limitation**: Each prompt should highlight at most 3 key points
- **Avoid attention dilution**: Overuse of emphasis formatting (bold, code blocks, etc.) reduces effectiveness
- **Position strategy**: Place the most critical information at the beginning and end

### Prompt Length Optimization
- **Conciseness principle**: Remove redundant descriptions, retain core information
- **Necessary details**: Retain key technical details and constraint conditions
- **Readability**: Reasonable paragraph division, avoid overly long paragraphs

### Prompt File Structure Requirements
- **YAML front configuration**: File may start with a YAML configuration block, defining tool permissions and basic description
- **Descriptive text**: Besides YAML configuration, should include a text description explaining the purpose and function of the memory prompt, approximately 2-5 sentences
- **Structural completeness**: Ensure prompt files have both configuration information and functional description
- ******:

### Format Optimization Techniques
- **Encoding standards**: Use UTF-8 encoding to ensure compatibility
- **Indentation standards**: Use 2 spaces for indentation uniformly
- **Line endings**: Use LF line endings (not CRLF)
- **Format consistency**: Ensure consistent formatting style throughout the document
- **Punctuation standards**: Prohibit Chinese punctuation, use English punctuation uniformly

### File Structure Representation Standards
- **Prohibit tree structure diagrams**: Do not use ASCII art-style tree diagrams to represent file structure
- **Use indentation method**: File structure must use simple indentation format for representation
- **Clear and concise**: Ensure structure is clear and readable, avoid overly complex representation methods

```xml

<Examples>
  <GoodExample>
    .docs/
    - `prompts/` - Prompt templates
    - `user/` - Global user prompts
    - `project/` - Project-level prompts
    - `slashcommands/` - Slash command prompts
    - `qa/` - Q&A documentation
    - `references/` - Technical reference documentation
    - `other/` - Other documentation (build, Git, database, etc.)
  </GoodExample>

  <BadExample description="Using tree structure diagram">
    .docs/
    ├── prompts/ # Prompt templates
    │ ├── user/ # Global user prompts
    │ ├── project/ # Project-level prompts
    │ └── slashcommands/ # Slash command prompts
    ├── qa/ # Q&A documentation
    ├── references/ # Technical reference documentation
    └── other/ # Other documentation (build, Git, database, etc.)
  </BadExample>
</Examples>
```

### Clarity Optimization
- **Avoid ambiguity**: Use precise vocabulary, avoid vague expressions
- **Concretization**: Convert abstract concepts into specific requirements
- **Executability**: Ensure instructions can be accurately understood and executed

### Constraint Condition Clarification
- **Must include**: Clearly list conditions that must be met
- **Prohibited items**: Clearly explain what cannot be done
- **Boundary conditions**: Define processing scope and limitations

### Output Standardization
- **Format specifications**: Specify specific output formats (tables, lists, code blocks, etc.)
- **Structure requirements**: Clearly define output organizational structure
- **Example descriptions**: Provide examples of expected output

### Punctuation Usage Examples

```xml

<Examples>
  <GoodExample description="Using English punctuation">
    # Role: Code Review Assistant

    You are an expert code reviewer with 10+ years of experience. Your task is to:
    1. Analyze code quality and identify potential issues
    2. Provide actionable feedback for improvements
    3. Ensure code follows best practices and security guidelines

    Focus on readability, maintainability, and performance aspects.
  </GoodExample>
  <BadExample description="Using Chinese punctuation">
    # Role: 代码审查助手

    你是一位拥有10年以上经验的专家代码审查员。你的任务是:
    1. 分析代码质量并识别潜在问题
    2. 提供可操作的改进建议
    3. 确保代码遵循最佳实践和安全准则

    重点关注可读性、可维护性和性能方面。
  </BadExample>
</Examples>

```

## Document Type Description

- `.docs/cmd/**.locale.md` - These are quick command files, focusing on task efficiency optimization
- `.docs/user/**.locale.md` - These are global memory files, usually more abstract
- `.docs/project/**.locale.md` - These are project-specific templates, abstract but with project characteristics
- `.docs/ss/**.locale.md` - These are "Claude Code sub-agents", professional and specialized in specific domains
- `.docs/CLAUDE-**.locale.md` - These are memory prompts for .docs/, helping users continuously improve prompts for better results