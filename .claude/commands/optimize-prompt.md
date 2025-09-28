---
allowed-tools: Read, Write, Edit, MultiEdit, Glob, Grep, Bash, TodoWrite, Task
description: Optimize Claude memory files to make them more readable for AI and achieve better work results
---

`optimize-prompt` is a task that optimizes Claude.ai `CLAUDE.md` memory prompt files. Parameters: `$1` (required): path to the prompt file to optimize, `$2` (optional): user's specific requirements or optimization direction.
This task performs structural optimization, format standardization, and content simplification of memory prompts according to established rules.

## Optimization Rules

### Language Selection Rules
- **Optimize Chinese files only**: Optimize only when filename contains `.locale.`
- **English files not optimized**: All non-`.locale.` files are translation generated, exit directly without processing

### Title Structure Optimization
- Title nesting level must not exceed level 3
- Remove redundant title nesting
- Ensure clear title hierarchy

### Content Expression Standards
- **Prohibit emojis**: Strictly prohibit any emojis in documentation to maintain professionalism
- Use concise and clear written expression
- Maintain documentation style consistency and professionalism

### Example Writing Standards

**XML Tag System**
Use structured XML tags to wrap examples, optimizing AI parsing efficiency and reading experience:

**Tag type definitions**:
- `<Example>` - General example, showing standard usage
- `<Examples>` - Example collection container, including multiple related examples
- `<GoodExample>` - Best practice example, showing recommended approach
- `<BadExample>` - Negative example, showing practices to avoid

**Description attribute specifications**:
- All example tags support `description=""` attribute to explain the example's purpose
- Descriptions should be concise and clear, highlighting the example's core value and learning points

**Dialogue mechanism specifications**:
- `user:` - User input content
- `claude:` - Claude response output content
- Support independent `claude:` to identify pure output scenarios

**Formatting constraints**:
- `<GoodExample>` and `<BadExample>` tags can only be used within `<Examples>` containers
- All XML tags and their content maintain zero indentation format
- Must keep one blank line separator between tags and above content, ensuring clear document structure
- **Code example format**: All code examples must be wrapped in language-identified code blocks, such as:
  ```rust
  // Rust code
  ```

**Content simplification principles**:
- Examples should be short and effective, highlighting key points
- Avoid lengthy implementation details, focus on demonstrating concepts
- Code examples should not exceed 20 lines, text examples should not exceed 5 lines
- Each example should demonstrate only one key concept or comparison point

**BadExample optimization limitations**:
- Do not format optimize content within `<BadExample>` tags during optimization
- Keep BadExample content as-is unless it doesn't match the intended meaning
- The purpose of BadExample is to show wrong practices, including wrong formats, punctuation, indentation, etc.

### Core Structural Elements
- **Role definition**: Clearly define AI's identity and professional background
- **Task description**: Clearly and specifically describe the task to complete
- **Constraint conditions**: Clearly define restrictions and requirements
- **Output format**: Specify the structure and format of output

### Attention Mechanism Optimization
- **Key point limit**: Each prompt should highlight at most 3 key points
- **Avoid attention dilution**: Overusing emphasis formats (bold, code blocks, etc.) reduces effectiveness
- **Position strategy**: Place the most critical information at the beginning and end

### Prompt Length Optimization
- **Simplification principle**: Remove redundant descriptions, keep core information
- **Necessary details**: Retain key technical details and constraint conditions
- **Readability**: Reasonable paragraphing, avoid overly long paragraphs

### Prompt File Structure Requirements
- **YAML front matter**: File beginning may contain YAML configuration block, defining tool permissions and basic descriptions
- **Descriptive text**: Besides YAML configuration, should also include text description explaining the prompt's purpose and functionality
- **Structural completeness**: Ensure prompt files have both configuration information and functional descriptions

### Format Optimization Techniques
- **Encoding standards**: Use UTF-8 encoding to ensure compatibility
- **Indentation standards**: Uniformly use 2-space indentation
- **Line endings**: Use LF line endings (not CRLF)
- **Format consistency**: Ensure consistent document format style throughout
- **Punctuation standards**: Prohibit Chinese punctuation, uniformly use English punctuation

### File Structure Representation Standards
- **Prohibit tree structure diagrams**: Do not use ASCII art-style tree diagrams to represent file structure
- **Use indentation method**: File structure must be represented using simple indentation format
- **Clear and concise**: Ensure structure is clear and readable, avoid overly complex representation methods

<Examples>
<GoodExample description="Correct file structure representation (using indentation)">
```text
docs/
  - `prompts/` - Prompt templates
    - `user/` - Global user prompts
    - `project/` - Project-level prompts
    - `slashcommands/` - Slash command prompts
  - `qa/` - Q&A documentation
  - `references/` - Technical reference documentation
  - `other/` - Other documentation (build, Git, database, etc.)
```
</GoodExample>
<BadExample description="Incorrect file structure representation (using tree diagram)">
docs/
├── prompts/             # Prompt templates
│   ├── user/            # Global user prompts
│   ├── project/         # Project-level prompts
│   └── slashcommands/   # Slash command prompts
├── qa/                  # Q&A documentation
├── references/          # Technical reference documentation
└── other/               # Other documentation (build, Git, database, etc.)
</BadExample>
</Examples>

### Clarity Optimization
- **Avoid ambiguity**: Use precise vocabulary, avoid vague expressions
- **Be specific**: Transform abstract concepts into specific requirements
- **Executability**: Ensure instructions can be accurately understood and executed

### Constraint Condition Clarification
- **Must include**: Clearly list conditions that must be satisfied
- **Prohibited items**: Clearly explain what cannot be done
- **Boundary conditions**: Define the scope and limitations of processing

### Output Standardization
- **Format specifications**: Specify specific output formats (tables, lists, code blocks, etc.)
- **Structure requirements**: Clearly define the organizational structure of output
- **Example explanations**: Provide examples of expected output

### Punctuation Usage Examples

<Examples>
<GoodExample description="Correct use of English punctuation">
# Role: Code Review Assistant

You are an expert code reviewer with 10+ years of experience. Your task is to:
1. Analyze code quality and identify potential issues
2. Provide actionable feedback for improvements
3. Ensure code follows best practices and security guidelines

Focus on readability, maintainability, and performance aspects.
</GoodExample>
<BadExample description="Incorrect use of Chinese punctuation">
# Role: 代码审查助手

你是一位拥有10年以上经验的专家代码审查员。你的任务是:
1. 分析代码质量并识别潜在问题
2. 提供可操作的改进建议
3. 确保代码遵循最佳实践和安全准则

重点关注可读性、可维护性和性能方面。
</BadExample>
</Examples>

### Code Format Examples

<Examples>
<GoodExample description="Correct 2-space indentation format">
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ProcessedItem {
  pub id: String,
  pub name: String,
  pub value: f64,
}

pub fn process_data(data: &[HashMap<String, String>]) -> HashMap<String, Vec<ProcessedItem>> {
  let mut result = HashMap::new();

  if data.is_empty() {
    result.insert("status".to_string(), vec![]);
    result.insert("count".to_string(), vec![]);
    return result;
  }

  let mut processed = Vec::new();
  for item in data {
    if let Some(active) = item.get("active") {
      if active == "true" {
        if let (Some(id), Some(name), Some(value_str)) =
            (&item.get("id"), &item.get("name"), &item.get("value")) {
          if let Ok(value) = value_str.parse::<f64>() {
            processed.push(ProcessedItem {
              id: id.clone(),
              name: name.trim().to_string(),
              value,
            });
          }
        }
      }
    }
  }

  result.insert("status".to_string(), vec![]);
  result.insert("count".to_string(), vec![]);
  result
}
</GoodExample>
<BadExample description="Incorrect indentation and format">
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ProcessedItem {
    pub id: String,
    pub name: String,
    pub value: f64,
}

pub fn process_data(data: &[HashMap<String, String>]) -> HashMap<String, Vec<ProcessedItem>> {
    let mut result = HashMap::new();

    if data.is_empty() {
        result.insert("status".to_string(), vec![]);
        result.insert("count".to_string(), vec![]);
        return result;
    }

    let mut processed = Vec::new();
    for item in data {
        if let Some(active) = item.get("active") {
            if active == "true" {
                if let (Some(id), Some(name), Some(value_str)) =
                    (&item.get("id"), &item.get("name"), &item.get("value")) {
                    if let Ok(value) = value_str.parse::<f64>() {
                        processed.push(ProcessedItem {
                            id: id.clone(),
                            name: name.trim().to_string(),
                            value,
                        });
                    }
                }
            }
        }
    }

    result.insert("status".to_string(), vec![]);
    result.insert("count".to_string(), vec![]);
    result
}
</BadExample>
</Examples>

### Prompt Structure Examples

<Examples>
<GoodExample description="Clear and concise prompt structure">
# Code Generation Assistant

Generate clean, efficient, and well-documented code based on requirements.

## Key Guidelines
- Use meaningful variable and function names
- Include type hints for better code clarity
- Write docstrings for all public functions
- Follow the project's established patterns

## Output Format
```rust
/// Function implementation with proper documentation
pub fn function_name(param: ParamType) -> ReturnType {
  /// Brief description of the function.
  ///
  /// # Arguments
  /// * `param` - Description of the parameter
  ///
  /// # Returns
  /// Description of the return value
  ///
  /// # Examples
  /// ```
  /// let result = function_name(input_value);
  /// assert_eq!(result, expected_value);
  /// ```
  // Implementation here
}
```
</GoodExample>
<BadExample description="Redundant and complex prompt structure">
## 🤖 AI Code Generation Assistant v2.0.1 ###

### 📋 MISSION STATEMENT:
You are an advanced AI-powered code generation system designed to create high-quality, production-ready code solutions for enterprise applications.

### 🔧 TECHNICAL REQUIREMENTS:
- **Variable Naming Convention**: MUST utilize meaningful, self-documenting variable names that clearly express intent
- **Type Safety**: ALL function parameters MUST include comprehensive type annotations using the latest typing features
- **Documentation Standards**: EVERY public function REQUIRES extensive docstring documentation following Google/NumPy conventions
- **Pattern Consistency**: MUST rigidly adhere to existing architectural patterns without deviation

### 📤 COMPLEX OUTPUT SPECIFICATION:
The generated code should follow this exact structure:

```rust
// -*- coding: utf-8 -*-
//! Enterprise-grade function implementation with comprehensive documentation.
//! This module represents a critical business logic component.

pub fn elaborate_function_name_with_verbose_description(
    parameter_name: ParameterType,
) -> Result<ReturnTypeType, Box<dyn std::error::Error>> {
  /// This function performs the specified operation with high reliability.
  ///
  /// # Arguments
  /// * `parameter_name` - A detailed explanation of what this parameter represents,
  ///                     including its expected format, constraints, and usage patterns
  ///
  /// # Returns
  /// * `Result<ReturnTypeType, Box<dyn std::error::Error>>` - A comprehensive description of
  ///   the return value, including all possible return states, error conditions,
  ///   and data structure details
  ///
  /// # Errors
  /// * `ValueError` - Detailed explanation of when this error might occur
  /// * `TypeError` - Comprehensive list of scenarios leading to type errors
  ///
  /// # Examples
  /// ```
  /// match elaborate_function_name_with_verbose_description(input) {
  ///     Ok(result) => println!("Operation succeeded: {:?}", result),
  ///     Err(e) => eprintln!("Operation failed: {}", e),
  /// }
  /// ```
  // Implementation placeholder with extensive comments
  unimplemented!()
}
```
</BadExample>
</Examples>

---

## Documentation Type Description

### Characteristics and Positioning of Different Document Types

- **docs/prompts/slashcommands/**.locale.md are shortcut command files, they focus on task efficiency optimization
- **docs/prompts/user/**.locale.md are global memory files, they are usually more abstract
- **docs/prompts/project/**.locale.md are project-specific templates, though abstract but more characteristic of each project
- **docs/CLAUDE-**.locale.md are memory prompts for docs/, they help users continuously refine prompts to achieve better results