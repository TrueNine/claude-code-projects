---
argument-hint: [locale_file_at_path]
allowed-tools: Read, Write, Edit, Glob, Grep, Bash
description: Optimize the memory prompt file so that it becomes more readable for "AI Agent" and achieves better results
---


`optimize-prompt` organizes the user intent and refines the description inside the `AI Agent` `$1` memory prompt while applying optional parameter `$2` for custom adjustments.
The pipeline follows established rules to restructure memory prompts, normalize formatting, and condense content.
IF the user does not provide `$1`, the task exits immediately.
Please note that memory prompts awaiting optimization usually contain formatting mistakes, redundant descriptions, or unfinished drafts, so modifications MUST stay traceable.


## Optimization Rules


### Language Selection Rules

- **Prioritize `**.locale.*` files**: Process immediately WHEN the file name contains `**.locale.*`.
- **Translate non `.locale.` files**: IF the user passes a file without `**.locale.*`, translate a Chinese `**.locale.*` sibling first, THEN optimize that translated `**.locale.*` file.
- **Maintain user awareness of prompts**: Use automatic translation and optimization so the user can always understand and control the prompt content.
- **Handle `**.locale.md` files**: Write refined content with American English logic and keep technical terms in English.


### Brief Introduction at the Beginning of the Document

- The first paragraph of a memory prompt file MUST summarize the document purpose in 2-5 sentences.
- Keep the statements direct and concise without rhetorical flourishes.


### Heading Structure Optimization

- Heading depth MUST NOT exceed three levels.
- Use `#` only for level-one headings and `##` only for level-two headings.
- Remove redundant nested headings.
- Ensure the heading hierarchy stays clear.


### Content Expression Guidelines

- **NEVER use emojis**: The document MUST remain professional.
- Use concise and precise written language.
- Keep the document style consistent and professional.
- Avoid elaborate wording or forced alignment; present the key information directly.


### Terminology Conversion Rules

- Replace “PascalCase” descriptions with `PascalCase`.
- Replace “camelCase” descriptions with `camelCase`.
- Replace “snake_case” descriptions with `snake_case`.
- Replace “kebab-case” descriptions with `kebab-case`.


### Example Construction Guidelines

- **XML container requirements**
  - Wrap examples with structured `XML` tags for easier parsing and reuse.
  - Place examples inside a ````xml ... ```` code block to keep formatting consistent.
  - All tag attributes MUST use double quotes.
  - Reuse tag names and structures defined by `__ai/meta/example-schema.dtd`.

- **Available tags**
  - `<examples>`: Top-level container for grouping examples within the same topic; MAY be the root node.
  - `<example>`: Generic example; MAY serve as a root node or as a child of `<examples>`.
  - `<good-example>`: Positive example; ONLY allowed inside `<examples>`.
  - `<bad-example>`: Negative example; ONLY allowed inside `<examples>`.
  - `<thinking>`: Describes the thought process; MUST nest inside a single example node.
  - `<tooling>`: Records tool commands used in the example; MUST set `name="..."` and be self-closing.

- **Attribute conventions**
  - `description="..."`: Optional, briefly states the example’s purpose.
  - `user-input="..."`: Optional, records the user input associated with the example.
  - `params:path="..."`: Optional, records related file paths.
  - `params:command="..."`: Optional, describes executed commands or scripts.
  - Other `params:*` attributes MUST follow the `params:<key>="<value>"` syntax with English attribute names.
  - Do not add optional attributes just for symmetry; omit unused information.
  - `description` content SHOULD directly describe the scenario without repeating tag semantics like “correct example.”

- **Atomicity principle**
  - Each example MUST focus on one concept without mixing topics.
  - Examples MUST NOT contain inline comments; move explanations into structured content.
  - `<tooling>` nodes MUST include `name="<tool-name>"`; use additional `params:*` attributes if needed. Because the node is empty, it MUST be self-closing.
  - Keep only one continuous code or output segment.
  - Example content MUST be self-contained and understandable without extra context.


````xml
<!DOCTYPE examples "/__ai/meta/example-schema.dtd">
<examples description="Result handling do and don't examples">
  <good-example>
    <thinking>Return the parse_data result directly so the caller handles the error context.</thinking>
    fn process_data(data: &str) -> Result<ProcessedData, Error> {
      parse_data(data)
    }
  </good-example>
  <bad-example description="Missing error propagation">
    <thinking>Hide the error details and only return a default value.</thinking>
    <tooling name="Search" params:pattern="process_data"/>
    fn process_data(data: &str) -> Result<ProcessedData, Error> {
      parse_data(data).unwrap_or_default()
    }
  </bad-example>
</examples>
````


### Core Structural Elements

- **Role definition**: Clarify the AI identity and professional background.
- **Task description**: Describe the task clearly and specifically.
- **Constraints**: Explain limitations and requirements explicitly.
- **Output format**: Specify the structure and format of the output.


### Attention Optimization

- **Focus limit**: Highlight no more than three core points per prompt.
- **Avoid attention dilution**: Overusing emphasis (bold, code blocks, etc.) reduces effectiveness.
- **Placement strategy**: Put the most critical information at the beginning and the end.


### Prompt Length Optimization

- **Streamline principle**: Remove redundant descriptions and keep core information.
- **Necessary details**: Preserve key technical details and constraints.
- **Readability**: Break content into manageable paragraphs to avoid overly long blocks.


### Memory Prompt File Structure Requirements

- **YAML front matter**: The file MAY start with a YAML block defining tool permissions and basic descriptions.
- **Descriptive text**: Beyond YAML, include a 2-5 sentence paragraph explaining the purpose and functionality of the memory prompt.
- **Structural integrity**: Ensure the file contains both configuration and descriptive sections.
- **Placeholder cleanup**: Remove legacy empty headings or invalid markers to keep the structure tight.


### Formatting Techniques

- **Encoding**: Use UTF-8 encoding for compatibility.
- **Indentation**: Use two spaces for indentation.
- **Line endings**: Use LF line endings (not CRLF).
- **Consistency**: Maintain a unified formatting style across the document.
- **Punctuation**: Use English punctuation only.
- Do NOT use `---` horizontal rules in the body; YAML front matter is the ONLY exception.


### File Structure Representation Guidelines

- **Prohibit tree diagrams**: Do NOT use ASCII tree diagrams to represent file structures.
- **Use indentation**: Represent file structures with simple indentation.
- **Clarity**: Keep the representation clear and easy to read without unnecessary complexity.


````xml
<!DOCTYPE examples SYSTEM "/__ai/meta/example-schema.dtd">
<examples>
  <good-example>
    - [.docs/](/.docs/)
      - [prompts/](/.docs/prompts/) - Prompt templates
      - [user/](/.docs/user/) - Global user prompts
      - [project/](/.docs/project/) - Project-level prompts
      - [slashcommands/](/.docs/slashcommands/) - Slash-command prompts
      - [qa/](/.docs/qa/) - Q&A documents
      - [references/](/.docs/references/) - Technical reference documents
      - [other/](/.docs/other/) - Other documents (build, Git, databases, etc.)
  </good-example>

  <bad-example description="Uses a tree diagram">
    .docs/
    ├── prompts/ # Prompt templates
    │ ├── user/ # Global user prompts
    │ ├── project/ # Project-level prompts
    │ └── slashcommands/ # Slash-command prompts
    ├── qa/ # Q&A documents
    ├── references/ # Technical reference documents
    └── other/ # Other documents (build, Git, databases, etc.)
  </bad-example>
</examples>
````


### Clarity Optimization

- **Avoid ambiguity**: Use precise wording and avoid vague expressions.
- **Make it concrete**: Turn abstract concepts into specific requirements.
- **Ensure executability**: Provide instructions that can be understood and executed accurately.


### Constraint Clarification

- **Mandatory items**: List conditions that MUST be met.
- **Prohibited items**: State clearly what is NOT allowed.
- **Boundary conditions**: Define the scope and limits of the task.


### Output Standardization

- **Formatting rules**: Specify the expected output format (tables, lists, code blocks, etc.).
- **Structural requirements**: Clarify how the output SHOULD be organized.
- **Example guidance**: Provide examples that illustrate the expected output.


### Punctuation Usage Example

````xml
<!DOCTYPE examples SYSTEM "/__ai/meta/example-schema.dtd">
<examples>
  <good-example description="Uses English punctuation">
    # Role: Code Review Assistant

    You are an expert code reviewer with 10+ years of experience. Your task is to:
    1. Analyze code quality and identify potential issues
    2. Provide actionable feedback for improvements
    3. Ensure code follows best practices and security guidelines

    Focus on readability, maintainability, and performance aspects.
  </good-example>
  <bad-example description="Uses Chinese punctuation">
    # Role: Code Review Assistant

    You are an expert code reviewer with 10+ years of experience. Your task is to:
    1. Analyze code quality and identify potential issues
    2. Provide actionable feedback for improvements
    3. Ensure code follows best practices and security guidelines

    Focus on readability, maintainability, and performance aspects.
  </bad-example>
</examples>
````
