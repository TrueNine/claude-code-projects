---
argument-hint: [locale_file_at_path]
allowed-tools: Read, Write, Edit, Glob, Grep, Bash
description: Optimize memory prompt files to improve readability for AI agents and achieve better working results
---



`optimize-prompt` is responsible for organizing user intent and optimizing the description of `AI Agent` `$1` memory prompts, while executing customized adjustments according to optional parameter `$2`. The process follows established rules to optimize the structure, formatting, and content simplification of memory prompts. If the user does not provide `$1`, the task will exit immediately. Please note: memory prompts to be optimized usually contain format errors, redundant descriptions, or incomplete draft information. Maintain traceability when making modifications.




## Optimization Rules



### Language Selection Rules

- **Priority processing of `**.locale.*` files**: When the filename contains `**.locale.*`, execute the process directly
- **Translate non-.locale. files**: If the user provides a file that is not `**.locale.*`, translate it into a Chinese `**.locale.*` file next to it, and optimize the translated `**.locale.*` file
- **Always maintain user awareness of prompts**: Through automatic translation and optimization mechanisms, ensure users can always understand and control prompt content
- When facing `**.locale.md` files, use American English logic to write British Chinese, keeping technical terms in their original English



### Brief Description at Document Beginning

- The first paragraph of a memory prompt file needs to use 2-5 sentences to outline the purpose of the entire document
- Keep the expression concise and direct, without rhetorical flourishes



### Title Structure Optimization

- Title nesting level must not exceed level 3
- `#` is only used for level 1 headings, `##` is only used for level 2 headings
- Remove redundant title nesting
- Ensure clear title hierarchy



### Content Expression Standards

- **Prohibit emoji usage**: Strictly prohibit any emoji in documents to maintain professionalism
- Use concise and clear written language
- Maintain document style consistency and professionalism
- Abandon flowery language and deliberate Chinese alignment formatting, directly present key information



### Terminology Conversion Rules

- Unify "大驼峰命名" (Big Camel Case naming) as `PascalCase`
- Unify "小驼峰命名" (Small Camel Case naming) as `camelCase`
- Unify "蛇形命名" (Snake naming) as `snake_case`
- Unify "烤串命名" (Kebab naming) as `kebab-case`



### Example Writing Standards

- **XML Container Requirements**
  - Use structured `XML` tags to wrap examples for easy parsing and reuse
  - Place examples in ` ```xml ... ``` ` code blocks for unified display format
  - Tag attributes must use English quotes, avoid mixing
  - Strictly reuse tag names and structures defined in `.ai/meta/example-schema.dtd`

- **Available Tags**
  - `<examples>`: Top-level container for grouping examples on the same topic, can be used directly as root node
  - `<example>`: General example, can be used as root node or embedded within `<examples>`
  - `<good-example>`: Positive example, can only appear within `<examples>`
  - `<bad-example>`: Negative example, can only appear within `<examples>`
  - `<thinking>`: Describes thinking process, can only be embedded within individual example nodes
  - `<tooling>`: Records tool commands involved in examples, must set `name="..."`, and is a self-closing tag

- **Attribute Conventions**
  - `description="..."`: Optional, briefly explains the intent of the example
  - `user-input="..."`: Optional, shows the user input corresponding to the example
  - `params:path="..."`: Optional, records file paths related to the example
  - `params:command="..."`: Optional, describes execution commands or scripts
  - Other `params:*` attributes follow `params:<key>="<value>"` syntax, must use English attribute names
  - Optional attributes should not be added just for format completeness, uninvolved information remains default
  - `description` content directly explains scenarios or risk points, does not repeat description of tag semantics like "correct example" "wrong example"

- **Atomic Principle**
  - Each example covers only one concept, does not split multiple topics
  - Prohibit inline comments in examples, explanatory information needs to be placed in structured content
  - `<tooling>` nodes need to include `name="<tool-name>"`, and supplement `params:*` as needed, because they are empty elements they must be self-closing
  - Keep only one continuous code segment or output fragment
  - Example content must be self-contained, understandable without additional context


```xml
<!DOCTYPE examples "/.ai/meta/example-schema.dtd">
<examples description="Correct and incorrect examples of handling Result">
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

- **Role Definition**: Clearly define the AI's identity and professional background
- **Task Description**: Clearly and specifically describe the tasks to be completed
- **Constraints**: Clearly define limitations and requirements
- **Output Format**: Specify the structure and format of output



### Attention Mechanism Optimization

- **Core Point Limit**: Each prompt should highlight at most 3 core points
- **Avoid Attention Dilution**: Overuse of emphasis formatting (bold, code blocks, etc.) will reduce effectiveness
- **Position Strategy**: Place the most critical information at the beginning and end



### Prompt Length Optimization

- **Simplification Principle**: Remove redundant descriptions, keep core information
- **Necessary Details**: Retain key technical details and constraint conditions
- **Readability**: Reasonable paragraph segmentation, avoid overly long paragraphs



### Prompt File Structure Requirements

- **YAML Front Matter**: The beginning of the file may contain a YAML configuration block, defining tool permissions and basic descriptions
- **Descriptive Text**: In addition to YAML configuration, should also include a text description explaining the purpose and functionality of the memory prompt, approximately 2-5 sentences
- **Structural Completeness**: Ensure prompt files have both configuration information and functional descriptions
- **Placeholder Cleanup**: Remove historically遗留 empty titles or invalid markers, maintain compact structure



### Format Optimization Techniques

- **Encoding Standards**: Use UTF-8 encoding to ensure compatibility
- **Indentation Standards**: Uniformly use 2-space indentation
- **Line Endings**: Use LF line endings (not CRLF)
- **Format Consistency**: Ensure the entire document format style is unified
- **Punctuation Standards**: Prohibit Chinese punctuation, uniformly use English punctuation
- Prohibit using `---` horizontal lines in the main text, except for YAML configuration blocks



### File Structure Representation Standards

- **Prohibit Tree Structure Diagrams**: Do not use ASCII art-style tree diagrams to represent file structure
- **Use Indentation Method**: File structures must be represented using simple indentation format
- **Clear and Concise**: Ensure the structure is clear and easy to read, avoid overly complex representation methods

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



### Constraint Clarification

- **Must Include**: Clearly list conditions that must be satisfied
- **Prohibited Items**: Clearly explain what cannot be done
- **Boundary Conditions**: Define the scope and limitations of processing



### Output Standardization

- **Format Specifications**: Specify specific output formats (tables, lists, code blocks, etc.)
- **Structural Requirements**: Clearly define the organizational structure of output
- **Example Explanations**: Provide examples of expected output



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