# Language Use Standards
- User Communication: All user-facing output must use Simplified Chinese, preserve common industry English terms, follow English sentence logic, avoid overly formal language.
- Code Development: Code, comments, and naming must all be in English, Chinese identifiers are forbidden, implement from day one.
- Internal Thinking: `Thinking` and `Tooling` sections must use American English, examples like `Thinking(desc: "Validate payload schema...")`, avoid mixed usage like `Thinking(desc: "检查文件...")`.
- Session Self-Check: Before sending, check if `Thinking` and `Tooling` contain Chinese, immediately change to English if found; user-visible output remains in Simplified Chinese.
- Cultural Orientation: Remember user's native language is Simplified Chinese, but workflow aligns with American engineering culture, use American English for all tasks except user communication.
- Strict Prohibition: Chinese characters in `Thinking` field => considered violation.
- "Memory Prompt" Style: Just ensure `Markdown` compliance, keep content technical and concise, don't waste time on layout alignment or rhetorical embellishment.
- [.ai/](/.ai) `**/*.locale.md` files: All [.ai/](/.ai) `**/*.locale.md` files use British-style Chinese writing, maintain English logic and direct term translation, full document execution.




# Project Tool Chain Configuration Constraints

## Tool Chain Priority
- Adoption order: 1) Root directory configuration files; 2) `.tool-versions` or `mise`; 3) `README` guidelines; 4) Existing scripts and `CI`.

```xml
<!DOCTYPE examples "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="Example: Correctly identify and use project tool chain"
                user-input="帮我运行测试">
    <tooling name="Search"
             params:pattern="Cargo.toml">
      Locate Cargo.toml within the workspace
    </tooling>
    <tooling name="Bash"
             params:command="test -f Cargo.toml">
      Confirm Cargo.toml exists at the repository root
    </tooling>
  </good-example>

  <bad-example description="Assume tool chain without investigation"
              userInput="帮我运行测试">
    <tooling name="Bash"
             params:command="npm test" />
  </bad-example>
</examples>
```




# Code Quality Standards

## Unified Format Standards
- Indentation: Fixed `2 spaces`.
- Encoding: `UTF-8`.
- Line ending: `LF`.
- Follow project under [.editorconfig](/.editorconfig) configuration

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="2-space indentation">
    fn main() {
      println!("Hello World");
    }
  </good-example>

  <bad-example description="4-space indentation causing format error">
    fn main() {
        println!("Hello World");
    }
  </bad-example>
</examples>
```




## File Naming Standards
- File name priority order: `PascalCase` or `camelCase` -> `snake_case` -> avoid `kebab-case` (unless language requires).

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="Component files use PascalCase">
    UserAccount.ts
    UserProfile.tsx
  </good-example>

  <good-example description="Utility files use camelCase">
    userUtils.ts
    apiClient.js
  </good-example>

  <good-example description="Configuration files accept snake_case">
    user_config.json
    database_settings.yaml
  </good-example>

  <good-example description="Rust module files use snake_case">
    user_service.rs
    auth_handler.rs
  </good-example>

  <bad-example description="File names use kebab-case">
    user-utils.ts
    api-client.js
  </bad-example>

  <bad-example description="Component files use lowercase">
    useraccount.ts
    userprofile.tsx
  </bad-example>
</examples>
```




## Code Style Constraints

- Comments should be placed above statements, avoid inline comments to prevent code line lengthening and reduce readability
- Conditional statements and loop bodies must explicitly use braces, avoid serious vulnerabilities from omission

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="Always use braces for conditional branches">
    if (is_ready) {
      handle_ready();
    }
  </good-example>

  <bad-example description="Omitting braces causes logic失控">
    if (is_ready)
      handle_ready();
      finalize();
  </bad-example>

  <bad-example description="Inline comments lengthen code lines">
    let total = price * quantity; // skip tax for legacy orders
  </bad-example>
  <good-example description="Correct comment style">
    // skip tax for legacy orders
    let total = price * quantity;
  </good-example>
</examples>
```




## Code Writing Techniques

### `Guard Clauses` & `Early Return`
Require using `guard clause` and `early return` to reduce nesting levels.

```xml
<!DOCTYPE examples "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="Use guard clause to reduce nesting">
    fn process_user(user: Option<&User>) -> Option<ProcessedUser> {
      let user = user?;
      if !user.is_active { return None; }
      if user.age < 18 { return None; }
      handle_adult_user(user)
    }
  </good-example>

  <bad-example description="Deep nesting style">
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
  </bad-example>
</examples>
```

### Multi-conditional Optimization
- When conditions ≥3, use `switch` / `match` or lookup table instead of `if-else` chain.
- Goal: Improve readability and maintainability, reduce repeated judgments.

```xml
<!DOCTYPE examples "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="match branches cover multiple conditions">
    fn get_error_message(status_code: u16) -> &'static str {
      match status_code {
        403 => "Permission denied, cannot access this resource",
        404 => "Requested resource does not exist",
        500 => "Internal server error, please try again later",
        code if code >= 500 => "Server error, please try again later",
        _ => "Unknown error"
      }
    }
  </good-example>

  <good-example description="Lookup table instead of multiple branches">
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
  </good-example>

  <bad-example description="Large if-else chain for multiple conditions">
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
  </bad-example>
</examples>
```




# Development Principles and Constraints

## Language Strategy
- Technical selection and architecture adjustments default to static type language stacks (like Rust, TypeScript, Go, Java, Kotlin) to ensure type safety and long-term maintainability.
- Even if existing project is built on dynamic languages, proposed solutions must plan migration to static types with stage goals and risk warnings.
- Prohibit actively expanding new dynamic language components; if user insists on dynamic languages, reconfirm and fully document potential risks.




## Prohibit Reinventing the Wheel
- Investigate existing functionality and architecture before development.
- Mandatory reuse of existing components, tools, or functions, no reimplementation.
- Prioritize reviewing and optimizing existing implementations and prompts, gain incremental value through adding tests, improving maintainability, or enhancing readability.
- Default strategy: Extend capabilities on existing foundation rather than rewrite.

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="Investigate before development and reuse existing code"
                user-input="帮我实现一个用户认证功能">
    <tooling name="Search"
             params:pattern="auth" />
    <tooling name="Read"
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
  </good-example>

  <bad-example description="Skip investigation and directly rewrite functionality"
             user-input="帮我实现一个用户认证功能">
    <tooling name="Write"
             params:path="NewAuthSystem" />
    ```rust
    struct NewAuthSystem;

    impl NewAuthSystem {
      async fn authenticate_user(&self, username: &str, password: &str) -> Result<User, AuthError> {
        unimplemented!()
      }
    }
    ```
  </bad-example>
</examples>
```




## File Management Principles
- Prioritize editing existing files, avoid creating new files.
- New files must have sufficient reason and conform to architectural specifications.
- Follow existing directory and module organization.

### File Creation Decision Standards
Reasonable file creation:
- Module functionality is independent and large (>=100 lines).
- Responsibilities clearly differ from existing files.
- Need independent test files.
- Align with project's modular design.

Avoid creating new files:
- Only contain few functions or constants.
- Functionality highly related to existing files.
- Just to avoid single file being too long (unless necessary).
- Break existing organizational structure.

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="Extend functionality within existing files"
               user-input="添加一个新的工具函数来格式化日期">
    将日期格式化函数合并进现有工具文件。
    <tooling name="Update"
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
  </good-example>

  <bad-example description="Unnecessarily create new files"
              user-input="添加一个新的工具函数来格式化日期">
    不必要地拆出新文件:
    <tooling name="Write"
             params:path="src/utils/date_utils.rs"
             description="不必要的文件创建" />
    ```rust
    use chrono::{DateTime, Local};

    pub fn format_date(date: DateTime<Local>, format: &str) -> String {
      date.format(format).to_string()
    }
    ```
  </bad-example>
</examples>
```




## Error Handling Transparency Principle
- Prohibit covering or suppressing any errors and warnings.
- Prohibit suppressing warnings,私自捕获不抛出, empty exception blocks, ignoring error codes, hiding exception details, modifying inspector configuration.

### Error Handling Standards
- Transparency: All errors/warnings fully exposed to users or calling layers.
- Traceability: Preserve complete stack and context.
- Responsibility: Calling layer decides how to handle, no silent swallowing at bottom layer.

### Error Handling Examples

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="完全透明">
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
  </good-example>

  <bad-example description="掩盖错误">
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
  </bad-example>
</examples>
```

### Warning Handling Examples

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="必须传递给调用者">
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
  </good-example>

  <bad-example description="镇压警告">
    fn validate_config(config: &Config) {
      if config.timeout < 1000 {
        // 镇压警告 - 禁止
        // println!("Warning: timeout is very short");
      }

      // 未告知调用者潜在问题
    }
  </bad-example>
</examples>
```

### Exception Handling Examples

- Keep exceptions transparent, prioritize letting exceptions bubble up
- If catching, must add context and rethrow or return error object,禁止静默吞掉
- Prioritize reusing existing exception types, avoid casually creating new exceptions that increase maintenance costs

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="复用现有异常并补充上下文">
    try {
      riskyOperation();
    } catch (Exception e) {
      throw new IOException("Unable to finish task", e);
    }
  </good-example>

  <bad-example description="静默吞掉异常导致信息丢失">
    try {
      riskyOperation();
    } catch (Exception ignored) {
    }
  </bad-example>
</examples>
```




# Strict Validation Mechanism

## Completion Validation Requirements
- Strictly prohibit claiming completion without validation.
- Must use actual validation actions rather than subjective judgment to support results.
- Progress feedback: When task not yet completed, must directly explain current progress and blocking points, avoid vague promises like "I'll implement in a simpler way", don't fabricate completion results; if stuck, actively request developer guidance.

## Validation Step Checklist
1. Code Quality Validation:
  - Run project-defined `diagnostic` / `lint` commands to check syntax and type issues.
  - Detect project tool chain, use corresponding check commands:
    - `Rust`: `cargo clippy`, `cargo fmt --check`.
    - `Node.js`: Run `lint` script according to `package.json`.
    - `Gradle`: `build.gradle.kts`, `settings.gradle.kts` etc.
2. Functionality Validation:
  - Run tests according to tool chain:
    - `Rust`: `cargo test`.
    - `Node.js`: `yarn test` / `pnpm test`.
    - `Gradle`: `./gradlew test`.
  - Execute necessary manual validation via `Bash` tool.
  - Confirm all modifications run as expected.
3. Completeness Validation:
  - Check if user requirements are covered.
  - Confirm all `TodoWrite` tasks marked as `completed`.
  - Verify changes haven't broken existing functionality.




## Validation Failure Handling
- Report discovered problems truthfully.
- Explain failure reasons transparently.
- Provide follow-up fix plan.

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="全面执行验证流程"
               user-input="帮我修复这个登录功能的bug">
    修复后执行完整验证。
    <tooling name="Bash" params:command="diagnostic">
      Run project-defined diagnostic command to surface syntax issues
    </tooling>
    <tooling name="Bash" params:command="cargo test">
      Execute `cargo test` aligned with the detected Cargo configuration
    </tooling>
    <tooling name="Bash" params:command="manual login flow">
      Drive a manual login flow check via Bash to confirm behaviour
    </tooling>
    全部验证通过:

    - ✅ 无语法或类型错误。
    - ✅ `cargo test` 全绿。
    - ✅ 手测登录流程正常。

    登录问题已修复并验证。
  </good-example>

  <bad-example description="未验证即声称完成"
             userInput="帮我修复这个登录功能的bug">
    未验证即声称修复完成, 违规。
  </bad-example>
</examples>
```




# Prompt Writing Standards

- `**/*.locale.md` files under [.ai/locale](/.ai/locale) are British-style Chinese prompt files,面向用户阅读, language must closely follow American English logic and thinking, professional terms directly retain English original text.
- These `**/*.locale.md` files often serve as `AI Agent` shortcut commands, sub-agent prompts, or `memory prompt file` localized versions, when assisting users in editing, must prioritize assisting with editing them.
- Examples often use `xml` structure presentation, with high reference value, when following examples prioritize understanding their structured intent, specific definitions can reference [.ai/meta/example-schema.dtd](/.ai/meta/example-schema.dtd).
- When `AI Agent` assists users in updating or writing such files, assume user is a programmer, possibly facing messy projects or outdated documentation, proactively correct and fill in omissions.
- Don't directly copy existing `**.locale.md` content; use English original manuscript as authoritative source, translate into standard American English logic下的英式中文, ensure locale version accurate and readable.
- When users propose new rules or ideas, immediately implement updates in the currently editing locale file, avoid delayed processing.
- [.ai](/.ai) files except under `meta/**` have no reference significance, they are engineering files automatically generated by `AI Agent`
- [.ai/meta](/.ai/meta) contains some specific concept help documentation definitions
- [.ai/middle/spec](/.ai/middle/spec) stores current project's "规范驱动开发" documents
- [.ai/middle/spec/SPEC-1-REQUIREMENT.locale.md](/.ai/middle/spec/SPEC-1-REQUIREMENT.locale.md) stores current project's "规范驱动开发 - 需求文档", it has and only has one copy or none, the project is developed based on this requirement

## File Structure Writing Examples

```xml
<!DOCTYPE example SYSTEM "/.ai/meta/example-schema.dtd">
<example description="使用 md 代码块的嵌套文件列表而不是树形结构">
- [.ai](/.ai) - AI Agent 工程目录，类似于 src 的源提示词工作目录
  - [.ai/locale/](/.ai/locale) - 当前项目映射的记忆提示词
  - [.ai/user/](/.ai/user) - 全局用户记忆提示词
  - [.ai/project/](/.ai/project) - 项目级别记忆提示词
  - [.ai/cmd/](/.ai/cmd) - 自定义命令提示词
  - [.ai/sa/](/.ai/sa) - 子代理提示词
  - [.ai/meta/](/.ai/meta) - 确切概念的帮助文档定义
- [README.md](/README.md) - 项目描述文件
- [AGENTS.md](/AGENTS.md) - AI 代理记忆提示词
- [.editorconfig](/.editorconfig) - 编辑器配置文件
</example>
```

## Path Reference Standards
- When writing prompt table references, must use `/` starting current project-based absolute paths as baseline
- Even for folder links, `/` ending cannot appear in `()`
- Absolute paths must not appear
- Don't wrap file references in bold, greatly distracts attention

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="正确的路径引用格式">
    [.ai/locale](/.ai/locale) - 当前项目映射的记忆提示词
    [src/utils](/src/utils) - 工具函数目录
    [README.md](/README.md) - 项目描述文件
  </good-example>

  <bad-example description="错误的路径引用格式">
    [.ai/locale/](/.ai/locale/) - 文件夹链接末尾不能有斜杠
    /home/user/project/src/utils - 禁止使用绝对路径
    [src/utils/](/src/utils/) - 文件夹链接末尾不能有斜杠
  </bad-example>

  <bad-example description="加粗包裹文件引用（禁止）">
    **[.ai/locale](/.ai/locale)** - 不得使用加粗包裹文件引用
    **[src/utils](/src/utils)** - 加粗会分散注意力
    **[README.md](/README.md)** - 禁止此格式
  </bad-example>
</examples>
```

## Reference Meta Definitions
Project under [.ai/meta](/.ai/meta) has some specific concept definitions, please use these definitions as standard.