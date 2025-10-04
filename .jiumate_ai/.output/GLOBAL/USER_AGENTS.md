# Communication Language Usage Standards
- User communication: All user-facing outputs use `Simplified Chinese`, industry common English terms may be retained, sentence structure follows English logic, avoid flowery language.
- Code development: All code, comments, and naming use English, Chinese identifiers are prohibited, effective from day one.
- Internal thinking: `Thinking` and `Tooling` sections must use American English, examples like `Thinking: Validate payload schema`, avoid mixed usage like `Thinking: 检查文件`.
- Session self-check: Before sending, check if `Thinking` and `Tooling` contain Chinese characters, if found, immediately change to English; user-visible output remains in simplified Chinese.
- Cultural orientation: Remember user's native language is Chinese, but workflow aligns with American engineering culture, use American English for all affairs except user communication.
- Strictly prohibited: Chinese characters in `Thinking` field => considered violation, enforcement starts from onboarding.
- Prompt style: As long as `Markdown` compliant, keep content technical and concise, don't waste time on formatting alignment or rhetoric.
- `**/*.locale.md` files: All `**/*.locale.md` use British Chinese writing, maintain English logic and direct translation of terminology, execute throughout the document.




# Project Toolchain Configuration Constraints

## Toolchain Priority
- Adoption order: 1) Root directory configuration files; 2) `.tool-versions` or `mise`; 3) `README` guidelines; 4) Existing scripts and `CI`.

```xml
<Examples>
  <GoodExample description="Example: Correctly identify and use project toolchain"
               userInput="帮我运行测试">
    <Tooling name="Search" params:pattern="Cargo.toml">
      Locate Cargo.toml within the workspace
    </Tooling>
    <Tooling name="Bash" params:command="test -f Cargo.toml">
      Confirm Cargo.toml exists at the repository root
    </Tooling>
  </GoodExample>

  <BadExample description="Counterexample: Assuming toolchain without investigation"
             userInput="帮我运行测试">
    <Tooling name="Bash"
             params:command="npm test" />
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
  <GoodExample description="Example: 2-space indentation correct format">
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
  <GoodExample description="Types use PascalCase">
    struct UserAccount;
  </GoodExample>

  <GoodExample description="Variables use camelCase">
    let userName = "john";
  </GoodExample>

  <GoodExample description="Variables acceptable snake_case">
    let user_count = 42;
  </GoodExample>

  <GoodExample description="Rust modules use snake_case">
    mod user_service;
  </GoodExample>

  <BadExample description="Variables use kebab-case">
    let user-name = "john";
  </BadExample>

  <BadExample description="Types use kebab-case">
    struct user-account;
  </BadExample>
</Examples>
```




## Code Style Constraints

- Comments should be placed above statements, prohibited inline supplements, to avoid elongating code lines and reducing readability
- Conditional statements and loop bodies must explicitly use braces, to avoid introducing serious vulnerabilities due to omission

```xml
<Examples>
  <GoodExample description="Conditional branches always use braces">
    if (is_ready) {
      handle_ready();
    }
  </GoodExample>

  <BadExample description="Omitting braces leads to logic失控">
    if (is_ready)
      handle_ready();
      finalize();
  </BadExample>

  <BadExample description="Inline comments elongate code lines">
    let total = price * quantity; // skip tax for legacy orders
  </BadExample>
  <GoodExample description="Correct comment method">
    // skip tax for legacy orders
    let total = price * quantity;
  </GoodExample>
</Examples>
```




## Code Writing Techniques

### `Guard Clauses` & `Early Return`
Require use of `guard clause` and `early return` to reduce nesting levels.

```xml
<Examples>
  <GoodExample description="Use guard clause to reduce nesting">
    fn process_user(user: Option<&User>) -> Option<ProcessedUser> {
      let user = user?;
      if !user.is_active { return None; }
      if user.age < 18 { return None; }
      handle_adult_user(user)
    }
  </GoodExample>

  <BadExample description="Deep nesting writing style">
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
  <GoodExample description="match branches cover multiple conditions">
    fn get_error_message(status_code: u16) -> &'static str {
      match status_code {
        403 => "Permission denied, cannot access this resource",
        404 => "Requested resource does not exist",
        500 => "Internal server error, please try again later",
        code if code >= 500 => "Server error, please try again later",
        _ => "Unknown error"
      }
    }
  </GoodExample>

  <GoodExample description="Lookup table replaces multiple branches">
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

  <BadExample description="Large if-else chains handle multiple conditions">
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
1. Comprehensive search: Use `Search` tool to explore keywords, use `Glob` to view directory structure, use `Read` to深入 key files.
2. Architecture analysis: Understand design patterns and coding style, locate reusable components and best extension points.
3. Reuse strategy: First choice is to extend existing classes/functions, second is to compose existing components, last is to create new components with sufficient justification.

```xml
<Examples>
  <GoodExample description="Investigate and reuse existing code before development"
               userInput="帮我实现一个用户认证功能">
    先搜索认证相关代码。
    <Tooling name="Search"
             params:words="auth" />
    <Tooling name="Read"
             params:path="AuthService" />
    选择扩展现有组件, 避免重造轮子:

    ```rust
    // 扩展现有 AuthService
    impl AuthService {
      pub async fn login(&self, credentials: LoginCredentials) -> Result<AuthResult, AuthError> {
        // ... existing implementation
      }

      // 新增多因素认证能力
      pub async fn enable_two_factor(&self, user_id: &str) -> Result<TwoFactorSetup, AuthError> {
        let user = self.validate_user(user_id).await?;
        self.setup_two_factor(&user).await
      }
    }
    ```
  </GoodExample>

  <BadExample description="Skip investigation and directly rewrite functionality"
             userInput="帮我实现一个用户认证功能">
    跳过调查直接重写, 违规:
    <Tooling name="Write"
             params:path="NewAuthSystem" />
    ```rust
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
  <GoodExample description="Extend functionality within existing files"
               userInput="添加一个新的工具函数来格式化日期">
    将日期格式化函数合并进现有工具文件。
    <Tooling name="Write"
             params:path="src/utils/helpers.rs" />
    ```rust
    use chrono::{DateTime, Local};

    // 在 helpers.rs 中新增函数
    pub fn format_date(date: DateTime<Local>, format: &str) -> String {
      date.format(format).to_string()
    }

    // 保持与现有函数风格一致
    pub fn format_currency(amount: f64) -> String { /* ... */ "".to_string() }
    pub fn format_phone_number(phone: &str) -> String { /* ... */ "".to_string() }
    ```
  </GoodExample>

  <BadExample description="Counterexample: Unnecessarily creating new files"
              userInput="添加一个新的工具函数来格式化日期">
    不必要地拆出新文件:
    <Tooling name="Write"
             params:path="src/utils/date_utils.rs"
             description="不必要的文件创建"/>
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
        // 镇压警告 - 禁止
        // println!("Warning: timeout is very short");
      }

      // 未告知调用者潜在问题
    }
  </BadExample>
</Examples>
```

### Exception Handling Examples

- Keep exceptions transparent, prioritize letting exceptions bubble up
- If catching is needed, must supplement context and rethrow or return error objects, prohibit silent swallowing
- Prioritize reusing existing exception types, avoid arbitrarily creating new exceptions leading to increased maintenance costs

```xml
<Examples>
  <GoodExample description="Reuse existing exceptions and supplement context">
    try {
      riskyOperation();
    } catch (Exception e) {
      throw new IOException("Unable to finish task", e);
    }
  </GoodExample>

  <BadExample description="Silent swallowing exceptions causes information loss">
    try {
      riskyOperation();
    } catch (Exception ignored) {
    }
  </BadExample>
</Examples>
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
    - `Rust`: `cargo clippy`, `cargo fmt --check`。
    - `Node.js`: Run `lint` scripts according to `package.json`。
    - `Gradle`: `build.gradle.kts`, `settings.gradle.kts` etc.。
2. Functionality verification:
  - Run tests according to toolchain:
    - `Rust`: `cargo test`。
    - `Node.js`: `yarn test` / `pnpm test`。
    - `Gradle`: `./gradlew test`。
  - Execute necessary manual verification through `Bash` tool.
  - Confirm all modifications work as expected.
3. Completeness verification:
  - Check if user requirements are covered.
  - Confirm all `TodoWrite` tasks marked as `completed`。
  - Verify changes don't break existing functionality.




## Verification Failure Handling
- Honestly report discovered issues.
- Transparently explain failure reasons.
- Provide follow-up fix plans.

```xml
<Examples>
  <GoodExample description="Comprehensively execute verification process"
               userInput="帮我修复这个登录功能的bug">
    修复后执行完整验证。
    <Tooling name="Bash" params:command="diagnostic">
      Run project-defined diagnostic command to surface syntax issues
    </Tooling>
    <Tooling name="Bash" params:command="cargo test">
      Execute `cargo test` aligned with the detected Cargo configuration
    </Tooling>
    <Tooling name="Bash" params:command="manual login flow">
      Drive a manual login flow check via Bash to confirm behaviour
    </Tooling>
    全部验证通过:

    - ✅ 无语法或类型错误。
    - ✅ `cargo test` 全绿。
    - ✅ 手测登录流程正常。

    登录问题已修复并验证。
  </GoodExample>

  <BadExample description="Claiming completion without verification"
             userInput="帮我修复这个登录功能的bug">
    未验证即声称修复完成, 违规。
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
- When user proposes new rules or ideas, need to immediately implement updates in the currently editing locale file, avoid delayed processing.