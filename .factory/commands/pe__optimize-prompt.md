---
argument-hint: [locale_file_at_path]
allowed-tools: Read, Write, Edit, Glob, Grep, Bash
description: Optimize memory prompt files to improve readability for AI agents and achieve better working results
---



`optimize-prompt` is responsible for organizing user intent and optimizing the description of `AI Agent` `$1` memory prompts, while executing custom adjustments based on optional parameter `$2`. The process follows established rules to perform structural optimization, format standardization, and content refinement of memory prompts. If the user does not provide `$1`, the task will exit immediately. Please note: memory prompts to be optimized typically contain format errors, redundant descriptions, or incomplete draft information. Maintain traceability when making modifications.




## Optimization Rules



### Language Selection Rules

- **Prioritize processing `**.locale.*` files**: When filename contains `**.locale.*`, execute process directly
- **Translate non-.locale. files**: If user provides non-`**.locale.*` files, translate a Chinese `**.locale.*` file next to it, then optimize the translated `**.locale.*` file
- **Maintain user awareness of prompts**: Ensure users can always understand and control prompt content through automatic translation and optimization mechanisms
- When handling `**.locale.md` files, use American English logic to write British Chinese, keeping English terminology unchanged



### Brief Description at Document Beginning

- First paragraph of memory prompt file needs to summarize the entire purpose in 2-5 sentences
- Keep expression concise and direct, avoid rhetorical embellishments



### Title Structure Optimization

- Title nesting level must not exceed 3 levels
- `#` only for level 1 titles, `##` only for level 2 titles
- Remove redundant title nesting
- Ensure clear title hierarchy



### Content Expression Standards

- **Prohibit emoji usage**: Strictly prohibit any emoji in documents to maintain professionalism
- Use concise and clear written expression
- Maintain document style consistency and professionalism
- Abandon flowery language and deliberate Chinese alignment formatting, present key information directly



### Terminology Conversion Rules

- Write "大驼峰命名" as `PascalCase`
- Write "小驼峰命名" as `camelCase`
- Write "蛇形命名" as `snake_case`
- Write "烤串命名" as `kebab-case`



### Example Writing Standards

- **XML Container Requirements**
  - Use structured `XML` tags to wrap examples for easy parsing and reuse
  - Place examples in ` ```xml ... ``` ` code blocks for unified display format
  - Tag attributes must use English quotes, avoid mixing
  - Strictly reuse tag names and structures defined in `.ai/meta/example-schema.dtd`

- **Available Tags**
  - `<examples>`: Top-level container for grouping examples of the same theme, can be used as root node directly
  - `<example>`: General example, can be root node or embedded within `<examples>`
  - `<good-example>`: Positive example, can only appear within `<examples>`
  - `<bad-example>`: Negative example, can only appear within `<examples>`
  - `<thinking>`: Describes thinking process, can only be embedded within individual example nodes
  - `<tooling>`: Records tool commands involved in examples, must set `name="..."`, and is self-closing tag

- **Attribute Conventions**
  - `description="..."`: Optional, briefly explains example intent
  - `user-input="..."`: Optional, shows user input corresponding to example
  - `params:path="..."`: Optional, records file paths related to example
  - `params:command="..."`: Optional, describes execution commands or scripts
  - Other `params:*` attributes follow `params:<key>="<value>"` syntax, must use English attribute names
  - Optional attributes should not be added just for formatting completeness, keep default for unrelated information
  - `description` content directly explains scenarios or risk points, avoid repeating "correct example" "bad example" and other tag semantics

- **Atomic Principle**
  - Each example covers only one concept, do not split multiple themes
  - Prohibit inline comments in examples, explanatory information must be placed in structured content
  - `<tooling>` nodes need to include `name="<tool-name>"` and supplement `params:*` as needed, self-closing because empty elements
  - Keep only one continuous code segment or output fragment
  - Example content must be self-contained, understandable without additional context


```xml
<!DOCTYPE examples "/.ai/meta/example-schema.dtd">
<examples description="Correct and incorrect examples for handling Result">
  <good-example>
    <thinking>Directly return parse_data result, let upper layer handle error context</thinking>
    fn process_data(data: &str) -> Result<ProcessedData, Error> {
      parse_data(data)
    }
  </good-example>
  <bad-example description="Missing error propagation">
    <thinking>Ignore error details, only return default value</thinking>
    <tooling name="Search" params:pattern="process_data"/>
    fn process_data(data: &str) -> Result<ProcessedData, Error> {
      parse_data(data).unwrap_or_default()
    }
  </bad-example>
</examples>
```



### Core Structural Elements

- **Role Definition**: Clearly define AI's identity and professional background
- **Task Description**: Clearly and specifically explain the task to be completed
- **Constraint Conditions**: Clearly define limitations and requirements
- **Output Format**: Specify the structure and format of output



### Attention Mechanism Optimization

- **Core Point Limit**: Each prompt highlights at most 3 core points
- **Avoid Attention Dilution**: Overuse of emphasis formats (bold, code blocks, etc.) reduces effectiveness
- **Position Strategy**: Place most critical information at beginning and end



### Prompt Length Optimization

- **Simplification Principle**: Remove redundant descriptions, keep core information
- **Necessary Details**: Retain key technical details and constraint conditions
- **Readability**: Reasonable paragraph division, avoid overly long paragraphs



### Prompt File Structure Requirements

- **YAML Front Configuration**: File beginning may contain YAML configuration block defining tool permissions and basic description
- **Descriptive Text**: Besides YAML configuration, should include text description explaining memory prompt's purpose and functionality, approximately 2-5 sentences
- **Structural Completeness**: Ensure prompt files have both configuration information and functionality description
- **Placeholder Cleanup**: Remove historical empty titles or invalid markers, keep structure compact



### Format Optimization Techniques

- **Encoding Standards**: Use UTF-8 encoding to ensure compatibility
- **Indentation Standards**: Use 2-space indentation consistently
- **Line Endings**: Use LF line endings (not CRLF)
- **Format Consistency**: Ensure entire document format style is unified
- **Punctuation Standards**: Prohibit Chinese punctuation, use English punctuation consistently
- Prohibit using `---` horizontal lines in main text, except YAML configuration blocks



### File Structure Representation Standards

- **Prohibit Tree Structure Diagrams**: Do not use ASCII art style tree diagrams to represent file structure
- **Use Indentation Method**: File structure must use simple indentation format representation
- **Clear and Concise**: Ensure structure is clear and readable, avoid overly complex representation methods

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example>
    - [.docs/](/.docs)
      - [prompts/](/.docs/prompts) - Prompt templates
      - [user/](/.docs/user) - Global user prompts
      - [project/](/.docs/project) - Project-level prompts
      - [slashcommands/](/.docs/slashcommands) - Slash command prompts
      - [qa/](/.docs/qa) - Q&A documentation
      - [references/](/.docs/references) - Technical reference documentation
      - [other/](/.docs/other) - Other documentation (build, Git, database, etc.)
  </good-example>

  <bad-example description="Using tree structure diagram">
    .docs/
    ├── prompts/ # Prompt templates
    │ ├── user/ # Global user prompts
    │ ├── project/ # Project-level prompts
    │ └── slashcommands/ # Slash command prompts
    ├── qa/ # Q&A documentation
    ├── references/ # Technical reference documentation
    └── other/ # Other documentation (build, Git, database, etc.)
  </bad-example>
</examples>
```



### Clarity Optimization

- **Avoid Ambiguity**: Use precise vocabulary, avoid vague expressions
- **Concretization**: Convert abstract concepts into specific requirements
- **Executability**: Ensure instructions can be accurately understood and executed



### Constraint Condition Clarification

- **Must Include**: Clearly list conditions that must be satisfied
- **Prohibited Items**: Clearly explain what cannot be done
- **Boundary Conditions**: Define processing scope and limitations



### Output Standardization

- **Format Specifications**: Specify specific output formats (tables, lists, code blocks, etc.)
- **Structural Requirements**: Clearly define output organization structure
- **Example Illustrations**: Provide examples of expected output



### Punctuation Usage Examples

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="Using English punctuation">
    # Role: Code Review Assistant

    You are an expert code reviewer with 10+ years of experience. Your task is to:
    1. Analyze code quality and identify potential issues
    2. Provide actionable feedback for improvements
    3. Ensure code follows best practices and security guidelines

    Focus on readability, maintainability, and performance aspects.
  </good-example>
  <bad-example description="Using Chinese punctuation">
    # Role: 代码审查助手

    你是一位拥有10年以上经验的专家代码审查员。你的任务是:
    1. 分析代码质量并识别潜在问题
    2. 提供可操作的改进建议
    3. 确保代码遵循最佳实践和安全准则

    重点关注可读性、可维护性和性能方面。
  </bad-example>
</examples>
```