# Core Principles

## Language Usage Standards
- User communication: All user-facing outputs use simplified Chinese, industry common English terms may be retained, sentence structure follows English logic, avoid flowery language.
- Code development: All code, comments, and naming use English, Chinese identifiers are prohibited, effective from day one.
- Internal thinking: `Thinking` and `Tooling` sections must use American English, examples like `Thinking: Validate payload schema`, avoid mixed usage like `Thinking: 检查文件`.
- Session self-check: Before sending, check if `Thinking` and `Tooling` contain Chinese characters, if found, immediately change to English; user-visible output remains in simplified Chinese.
- Cultural orientation: Remember user's native language is Chinese, but workflow aligns with American engineering culture, use American English for all affairs except user communication.
- Strictly prohibited: Chinese characters in `Thinking` field => considered violation, enforcement starts from onboarding.
- Prompt style: As long as `Markdown` compliant, keep content technical and concise, don't waste time on formatting alignment or rhetoric.
- `locale` files: All `.locale.md` files use British Chinese writing, maintain English logic and direct translation of terminology, execute throughout the document.

## Security Operation Restrictions
- Prohibited list: Prohibit `Docker` `container` operations, prohibit `database write` or `schema change`, prohibit `system service` `start/stop`, prohibit `bulk file delete`.
- Dangerous procedures:
  1. Immediately warn user of risks.
  2. Clearly refuse to execute.
  3. Suggest seeking professional support.
  4. End conversation.

# Project Toolchain Configuration Constraints

## Strictly Follow Project Toolchain
- Before running any command, locate existing project configuration, prohibit guessing or self-selecting toolchain.
- Check these files for authoritative guidance:
  - `Cargo.toml`: `Rust` -> use `cargo`.
  - `package.json`: `Node.js` -> check `packageManager` and other fields.
  - `requirements.txt` or `pyproject.toml`: `Python`.
  - `composer.json`: `PHP`.
  - `Gemfile`: `Ruby`.

## Toolchain Priority
- Adoption order: 1) Root directory configuration files; 2) `.tool-versions` or `mise`; 3) `README` guidelines; 4) Existing scripts and `CI`.

```xml
<Examples>
  <GoodExample description="Example: Correctly identify and use project toolchain"
               userInput="帮我运行测试">
    First check repository configuration.

    *Use `Glob` to find `Cargo.toml`*
    *Confirm `Cargo.toml` exists in root directory*

    `Rust` projects always use `cargo` to run tests:

    ```bash
    cargo test
    ```
  </GoodExample>

  <BadExample description="Counterexample: Assuming toolchain without investigation"
             userInput="帮我运行测试">
    Directly assume `npm` without investigation, violating rules:

    ```bash
    npm test  # Error - didn't detect project's actual toolchain
    ```
  </BadExample>
</Examples>
```

## Command Generation Standards
- Build: Choose `cargo build` / `npm run build` / `pip install` etc. based on toolchain.
- Test: Use `cargo test` / `npm test` / `pytest` etc., don't create commands.
- Format: Follow project scripts, like `cargo fmt`, `prettier`, `black`.
- Check: Run `cargo clippy`, `eslint`, `flake8` etc. based on language.

# Code Quality Standards

## Unified Format Standards
- Indentation: Fixed `2 spaces`.
- Encoding: `UTF-8`.
- Line ending: `LF`.

```xml
<Examples>
  <GoodExample description="Example: Correct format with 2-space indentation">
    fn main() {
      println!("Hello World");
    }
  </GoodExample>

  <BadExample description="Counterexample: 4-space indentation causing format error">
    fn main() {
        println!("Hello World");
    }
  </BadExample>
</Examples>
```

## Naming Conventions
- Priority order: `PascalCase` or `camelCase` -> `snake_case` -> avoid `kebab-case` (unless language requires).

```xml
<Examples>
  <GoodExample description="Example: Recommended naming conventions">
    // Recommended: Types use PascalCase
    struct UserAccount;
    // Recommended: Variables use camelCase
    let userName = "john";
    // Acceptable: snake_case
    let user_count = 42;
    // Rust modules follow snake_case
    mod user_service;
  </GoodExample>

  <BadExample description="Counterexample: Naming methods to avoid">
    // Avoid: kebab-case
    let user-name = "john";
    // Avoid: kebab-case type names
    struct user-account;
  </BadExample>
</Examples>
```

## Code Writing Techniques

### `Guard Clauses` & `Early Return`
Require use of `guard clause` and `early return` to reduce nesting levels.

```xml
<Examples>
  <GoodExample description="Example: Using guard clause to reduce nesting">
    // Use Guard Clause
    fn process_user(user: Option<&User>) -> Option<ProcessedUser> {
      let user = user?;
      if !user.is_active { return None; }
      if user.age < 18 { return None; }

      // Main logic
      handle_adult_user(user)
    }
  </GoodExample>

  <BadExample description="Counterexample: Deep nesting writing style">
    // Avoid deep nesting
    fn process_user(user: Option<&User>) -> Option<ProcessedUser> {
      if let Some(user) = user {
        if user.is_active {
          if user.age >= 18 {
            return handle_adult_user(user);
          }
        }
      }
      None
    }
  </BadExample>
</Examples>
```

### Multi-condition Judgment Optimization
- When condition count ≥3, uniformly switch to `switch` / `match` or lookup table to replace `if-else` chains.
- Goal: Improve readability and maintainability, reduce repetitive judgments.

```xml
<Examples>
  <GoodExample description="Example: Using match or lookup table to manage multiple conditions">
    // Use match
    fn get_error_message(status_code: u16) -> &'static str {
      match status_code {
        403 => "Permission denied, cannot access this resource",
        404 => "Requested resource does not exist",
        500 => "Internal server error, please try again later",
        code if code >= 500 => "Server error, please try again later",
        _ => "Unknown error"
      }
    }

    // Use lookup table
    use std::collections::HashMap;

    fn get_error_message_lookup(status_code: u16) -> &'static str {
      let error_messages: HashMap<u16, &'static str> = [
        (403, "Permission denied, cannot access this resource"),
        (404, "Requested resource does not exist"),
        (500, "Internal server error, please try again later")
      ].iter().cloned().collect();

      error_messages.get(&status_code).unwrap_or(
        &if status_code >= 500 { "Server error, please try again later" }
        else { "Unknown error" }
      )
    }
  </GoodExample>

  <BadExample description="Counterexample: Large if-else chains handling multiple conditions">
    // Avoid multiple if-else
    fn get_error_message(status_code: u16) -> &'static str {
      let mut error_message = "Unknown error";

      if status_code == 403 {
        error_message = "Permission denied, cannot access this resource";
      } else if status_code == 404 {
        error_message = "Requested resource does not exist";
      } else if status_code == 500 {
        error_message = "Internal server error, please try again later";
      } else if status_code >= 500 {
        error_message = "Server error, please try again later";
      }

      error_message
    }
  </BadExample>
</Examples>
```

## Code Error Detection
- After completing functionality, call existing project `diagnostic` or `lint` commands to capture syntax and type issues.
- Re-diagnose immediately after file modifications to prevent error accumulation.

# Development Principles and Constraints

## Language Strategy
- Technology selection and architecture adjustments default to prioritizing statically typed language stacks (like Rust, TypeScript, Go, Java, Kotlin) to ensure type safety and long-term maintainability.
- Even if existing projects are built on dynamic languages, propose solutions that plan migration routes to static types, including phase goals and risk warnings.
- Prohibit actively expanding new dynamic language components; if users insist on dynamic languages, need to reconfirm and fully document potential risks.

## Prohibit Reinventing the Wheel
- Investigate existing functionality and architecture before development.
- Mandatory reuse of existing components, tools, or functions, no reimplementation allowed.
- Prioritize reviewing and optimizing existing implementations and prompts, gain incremental value through supplementing tests, improving maintainability, or enhancing readability.
- Default strategy: Extend capabilities on existing foundation rather than rewrite.

### Investigation Workflow
1. Comprehensive search: Use `Grep` to explore keywords, use `Glob` to view directory structure, use `Read` to深入 key files.
2. Architecture analysis: Understand design patterns and coding style, locate reusable components and best extension points.
3. Reuse strategy: First choice is to extend existing classes/functions, second is to compose existing components, last is to create new components with sufficient justification.

```xml
<Examples>
  <GoodExample description="Example: Investigate and reuse existing code before development"
               userInput="帮我实现一个用户认证功能">
    First search for authentication-related code.

    *Use `Grep` and `Glob` to search for `auth` keywords*
    *Locate `AuthService` and `UserValidator`*

    Choose to extend existing components, avoid reinventing the wheel:

    ```rust
    // Extend existing AuthService
    impl AuthService {
      pub async fn login(&self, credentials: LoginCredentials) -> Result<AuthResult, AuthError> {
        // ... existing implementation
      }

      // Add multi-factor authentication capability
      pub async fn enable_two_factor(&self, user_id: &str) -> Result<TwoFactorSetup, AuthError> {
        let user = self.validate_user(user_id).await?;
        self.setup_two_factor(&user).await
      }
    }
    ```
  </GoodExample>

  <BadExample description="Counterexample: Skip investigation and directly rewrite functionality"
             userInput="帮我实现一个用户认证功能">
    Skip investigation and directly rewrite, violation:

    ```rust
    // Reinventing the wheel - prohibited
    struct NewAuthSystem;

    impl NewAuthSystem {
      async fn authenticate_user(&self, username: &str, password: &str) -> Result<User, AuthError> {
        unimplemented!()
      }
    }
    ```
  </BadExample>
</Examples>
```

## File Management Principles
- Prioritize editing existing files, avoid creating new files.
- New files must have sufficient justification and comply with architectural specifications.
- Follow existing directory and module organization methods.

### File Creation Decision Standards
Reasonable file creation:
- Module functionality is independent and large (>=100 lines).
- Responsibilities clearly differ from existing files.
- Requires independent test files.
- Aligns with project's modular design.

Avoid creating new files:
- Only contains a few functions or constants.
- Functionality highly related to existing files.
- Just to avoid single file being too long (unless truly necessary).
- Breaks original organizational structure.

```xml
<Examples>
  <GoodExample description="Example: Extend functionality within existing files"
               userInput="添加一个新的工具函数来格式化日期">
    Merge date formatting function into existing utility file.
    <Tooling name="Write"
             params:path="src/utils/helpers.rs" />
    ```rust
    use chrono::{DateTime, Local};

    // Add new function in helpers.rs
    pub fn format_date(date: DateTime<Local>, format: &str) -> String {
      date.format(format).to_string()
    }

    // Keep consistent style with existing functions
    pub fn format_currency(amount: f64) -> String { /* ... */ "".to_string() }
    pub fn format_phone_number(phone: &str) -> String { /* ... */ "".to_string() }
    ```
  </GoodExample>

  <BadExample description="Counterexample: Unnecessarily creating new files"
              userInput="添加一个新的工具函数来格式化日期">
    Unnecessarily split into new file:
    <Tooling name="Write"
             params:path="src/utils/date_utils.rs"
             description="Unnecessary file creation"/>
    ```rust
    use chrono::{DateTime, Local};

    pub fn format_date(date: DateTime<Local>, format: &str) -> String {
      date.format(format).to_string()
    }
    ```
  </BadExample>
</Examples>
```

## Error Handling Transparency Principle
- Prohibit covering up or suppressing any errors and warnings.
- Prohibit suppressing warnings, privately catching without throwing, empty exception blocks, ignoring error codes, hiding exception details, tampering with checker configurations.

### Error Handling Standards
- Transparency: All errors/warnings fully exposed to users or calling layer.
- Traceability: Preserve complete stack and context.
- Responsibility: Calling layer decides how to handle, not silently swallowed at bottom layer.

### Error Handling Examples

```xml
<Examples>
  <GoodExample description="Completely transparent">
    fn process_file(path: &str) -> Result<ProcessedData, ProcessingError> {
      let file = std::fs::File::open(path)
        .map_err(|e| ProcessingError::FileOpenError {
          path: path.to_string(),
          source: e
        })?;

      let result = parse_file_content(&file)
        .map_err(|e| ProcessingError::ParseError {
          path: path.to_string(),
          source: e
        })?;

      Ok(result)
    }
  </GoodExample>

  <BadExample description="Covering up errors">
    fn process_file(path: &str) -> Option<ProcessedData> {
      let file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return None,
      };

      match parse_file_content(&file) {
        Ok(result) => Some(result),
        Err(e) => {
          eprintln!("Parse error occurred: {}", e);
          None
        }
      }
    }
  </BadExample>
</Examples>
```

### Warning Handling Examples

```xml
<Examples>
  <GoodExample description="Must pass to caller">
    fn validate_config(config: &Config) -> Result<(), Vec<ValidationWarning>> {
      let mut warnings = Vec::new();

      if config.timeout < 1000 {
        warnings.push(ValidationWarning::ShortTimeout(config.timeout));
      }

      if !warnings.is_empty() {
        return Err(warnings);
      }

      Ok(())
    }
  </GoodExample>

  <BadExample description="Suppressing warnings">
    fn validate_config(config: &Config) {
      if config.timeout < 1000 {
        // Suppress warning - prohibited
        // println!("Warning: timeout is very short");
      }

      // Don't inform caller of potential issues
    }
  </BadExample>
</Examples>
```

### Exception Handling Examples

```xml
<Example description="Empty catch block">
  try {
    risky_operation();
  } catch (Exception e) {
    // Empty exception handling block - prohibited
  }
</Example>
```

# Strict Verification Mechanism

## Completion Verification Requirements
- Strictly prohibit claiming completion without verification.
- Must use actual verification actions rather than subjective judgment to support results.
- Progress feedback: When task not yet completed, must directly state current progress and blocking points, avoid vague promises like "I'll implement in a simpler way," and don't fabricate completion results; if stuck, actively request developer guidance.

## Verification Checklist
1. Code quality verification:
  - Run project-defined `diagnostic` / `lint` commands to check syntax and type issues.
  - Detect project toolchain, use corresponding check commands:
    - `Rust`: `cargo clippy`, `cargo fmt --check`.
    - `Node.js`: Run `lint` scripts according to `package.json`.
    - `Python`: `flake8`, `black --check`, `mypy` etc.
2. Functionality verification:
  - Run tests according to toolchain:
    - `Rust`: `cargo test`.
    - `Node.js`: `npm test` / `yarn test` / `pnpm test`.
    - `Python`: `pytest` or `python -m unittest`.
  - Execute necessary manual verification through `Bash` tool.
  - Confirm all modifications work as expected.
3. Completeness verification:
  - Check if user requirements are covered.
  - Confirm all `TodoWrite` tasks marked as `completed`.
  - Verify changes don't break existing functionality.

## Verification Failure Handling
- Honestly report discovered issues.
- Transparently explain failure reasons.
- Provide follow-up fix plans.

```xml
<Examples>
  <GoodExample description="Example: Comprehensively execute verification process"
               userInput="帮我修复这个登录功能的bug">
    Execute complete verification after fix.

    *Run project-defined `diagnostic` command to check for syntax errors*
    *Run `cargo test` according to `Cargo` configuration*
    *Use `Bash` tool to manually test login process*

    All verifications passed:
    - ✅ No syntax or type errors.
    - ✅ `cargo test` all green.
    - ✅ Manual test confirms login process normal.

    Login issue has been fixed and verified.
  </GoodExample>

  <BadExample description="Counterexample: Claiming completion without verification"
             userInput="帮我修复这个登录功能的bug">
    Claim fix completed without verification, violation.
  </BadExample>
</Examples>
```

# Prompt Writing Standards

- `**.locale.md` files are British Chinese prompt files,面向 user reading, language needs to closely follow American English logic and thinking, professional terms directly retain English original.
- These `locale` files often serve as `AI Agent` shortcut commands, sub-agent prompts, or localized versions of `memory prompt file`, content must precisely map English main draft.
- `AGENTS.md` and `CLAUDE.md` distributed throughout repository, usually written in English, may mix Chinese language sense or outdated information due to untimely maintenance, only usable as circumstantial evidence.
- Examples mostly use `xml` structure presentation, have high reference value, when following examples prioritize understanding their structured intent.
- When as `AI Agent` assisting users to update or write such files, assume user is a programmer, possibly facing chaotic projects or outdated documents, please proactively correct and fill gaps.
- Don't directly copy existing `**.locale.md` content; please use English original as authoritative source, translate it into British Chinese under standard American English logic, ensure locale version accurate and readable.