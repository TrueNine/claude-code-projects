# Communication Language Usage Standards

## User Communication
- All user-visible output MUST use Simplified Chinese, retain industry-standard English terminology, ensure sentences follow English logic, and NEVER use flowery expressions.

## Code Development
- All code, comments, and naming MUST use English, Chinese identifiers are NEVER allowed, and this rule applies immediately from day one.

## Internal Reasoning
- The `Thinking` and `Tooling` sections MUST use American English; the correct example is `Thinking(desc: "Validate payload schema...")`, and NEVER `Thinking(desc: "检查文件...")`.

## Conversation Self-Check
- Before sending any response, ALWAYS verify that the `Thinking` and `Tooling` sections contain no Chinese characters; WHEN they do, THEN replace them with English immediately. User-visible output MUST remain in Simplified Chinese at all times.

## Cultural Orientation
- Remember the user's native language is Simplified Chinese. However, all workflows outside direct user communication MUST align with American engineering culture and operate entirely in American English.

## Strict Prohibition
- WHEN the `Thinking` field contains any Chinese characters, THEN it is treated as an immediate violation.

## Locale Tone
- `Memory prompt` content only needs to follow Markdown conventions, stay technical and concise, and MUST NOT waste time on layout or ornate wording.
- Files under [__ai/`**/*.locale.md`](/__ai/) MUST be written in British-style Chinese, maintain English logic with direct terminology translation, and enforce this rule throughout.



# Project Toolchain Configuration Constraints

## Toolchain Priority
- Rely on configuration sources in the following order: 1) Root-level configuration files; 2) `.tool-versions` or `mise`; 3) `README` guides; 4) Existing scripts and `CI`.

````xml
<!DOCTYPE examples SYSTEM "/__ai/meta/example-schema.dtd">
<examples>
  <good-example description="Correctly identify and use the project toolchain"
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

  <bad-example description="Assuming the toolchain without investigation"
              userInput="帮我运行测试">
    <tooling name="Bash"
             params:command="npm test" />
  </bad-example>
</examples>
````



# Code Quality Standards

## Unified Formatting Conventions
- Follow the configuration provided in [.idea/codeStyles/Project.xml](/.idea/codeStyles/Project.xml).
- Follow the configuration provided in [.editorconfig](/.editorconfig).



## File Naming Conventions
- Prefer `PascalCase` or `camelCase` for filenames.
- `snake_case` MAY be used as an optional alternative.
- Avoid `kebab-case`.

````xml
<!DOCTYPE examples SYSTEM "/__ai/meta/example-schema.dtd">
<examples>
  <good-example description="Component files use PascalCase">
    <tooling name="Create" params:name="UserAccount.ts"/>
    <tooling name="Create" params:name="UserProfile.tsx"/>
  </good-example>
  <good-example description="Utility files use camelCase">
    <tooling name="Create" params:name="userUtils.ts" />
    <tooling name="Create" params:name="apiClient.js" />
  </good-example>

  <good-example description="Configuration files MAY use snake_case">
    <tooling name="Create" params:name="user_config.json" />
    <tooling name="Create" params:name="database_settings.yaml" />
  </good-example>

  <good-example description="Rust module files use snake_case">
    <tooling name="Create" params:name="user_service.rs"/>
    <tooling name="Create" params:name="auth_handler.rs" />
  </good-example>

  <bad-example description="Filenames use kebab-case">
    <tooling name="Create" params:name="user-utils.ts"/>
    <tooling name="Create" params:name="api-client.js" />
  </bad-example>

  <bad-example description="Component files written in lowercase">
    <tooling name="Create" params:name="useraccount.ts" />
    <tooling name="Create" params:name="userprofile.tsx" />
  </bad-example>
</examples>
````



## Coding Style Constraints
- Place comments above statements; inline trailing comments are NEVER allowed because they elongate lines and reduce readability.
- Conditionals and loops MUST use explicit braces to avoid severe bugs caused by omission.

````xml
<!DOCTYPE examples SYSTEM "/__ai/meta/example-schema.dtd">
<examples>
  <good-example description="Conditional branches ALWAYS use braces">
    ```rust
    if (is_ready) {
      handle_ready();
    }
    ```
  </good-example>

  <bad-example description="Omitting braces causes logic to go out of control">
    ```rust
    if (is_ready)
      handle_ready();
      finalize();
    ```
  </bad-example>

  <bad-example description="Inline comments elongate code lines">
    ```rust
    let total = price * quantity; // skip tax for legacy orders
    ```
  </bad-example>
  <good-example description="Correct commenting style">
    ```rust
    // skip tax for legacy orders
    let total = price * quantity;
    ```
  </good-example>
</examples>
````



## Coding Techniques

### Guard Clauses and Early Return
- Guard clauses and early returns are MANDATORY to reduce nesting depth.

````xml
<!DOCTYPE examples "/__ai/meta/example-schema.dtd">
<examples>
  <good-example description="Use guard clauses to reduce nesting">
    ```rust
    fn process_user(user: Option<&User>) -> Option<ProcessedUser> {
      let user = user?;
      if !user.is_active { return None; }
      if user.age < 18 { return None; }
      handle_adult_user(user)
    }
    ```
  </good-example>

  <bad-example description="Deeply nested style">
    ```rust
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
    ```
  </bad-example>
</examples>
````

### Multi-Condition Optimization
- WHEN branch counts reach three or more, THEN replace `if-else` chains with `switch`/`match` or lookup-table solutions.
- The goal is to improve readability and maintainability while reducing repeated checks.

````xml
<!DOCTYPE examples "/__ai/meta/example-schema.dtd">
<examples>
  <good-example description="match covers multiple branches">
    ```rust
    fn get_error_message(status_code: u16) -> &'static str {
      match status_code {
        403 => "Permission denied, cannot access this resource",
        404 => "Requested resource does not exist",
        500 => "Internal server error, please try again later",
        code if code >= 500 => "Server error, please try again later",
        _ => "Unknown error"
      }
    }
    ```
  </good-example>

  <good-example description="Lookup table replaces multiple branches">
    ```rust
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
    ```
  </good-example>

  <bad-example description="Large if-else chain handling multiple conditions">
    ```rust
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
    ```
  </bad-example>
</examples>
````



# Development Principles and Constraints

## Language Strategy
- Technology choices and architectural adjustments MUST prioritize statically typed stacks (such as Rust, TypeScript, Go, Java, Kotlin) to ensure type safety and long-term maintainability.
- Even WHEN an existing project is built on a dynamic language, proposals MUST include a roadmap for migrating to a statically typed stack, covering phase goals and risk alerts.
- NEVER introduce new dynamic language components proactively; WHEN users insist on dynamic languages, THEN reconfirm and fully document potential risks.



## Ban on Reinventing the Wheel
- Investigate existing functionality and architecture before development.
- Existing components, tools, or functions MUST be reused; do NOT reimplement them.
- Prioritize reviewing and improving current implementations and prompts, adding value through tests, maintainability, or readability enhancements.
- The default strategy is to extend existing capabilities rather than rewrite them.

````xml
<!DOCTYPE examples SYSTEM "/__ai/meta/example-schema.dtd">
<examples>
  <good-example description="Investigate and reuse existing code before development"
                user-input="帮我实现一个用户认证功能">
    <tooling name="Search"
             params:pattern="auth" />
    <tooling name="Read"
             params:path="AuthService" />
    Choose to extend existing components and avoid reinventing the wheel:

    ```rust
    // Extend the existing AuthService
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
  </good-example>

  <bad-example description="Skipping investigation and rewriting the functionality"
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
````



## File Management Principles
- Prefer editing existing files and avoid creating new ones.
- New files MUST have solid justification and comply with the established architecture.
- Strictly follow the existing directory and module organization.
- NEVER edit [AGENTS.md](/AGENTS.md) or [CLAUDE.md](/CLAUDE.md); these files are maintained directly by the user.

### File Creation Decision Criteria
Reasonable scenarios for creating new files:
- The module is independent and sizable (at least 100 lines).
- Responsibilities differ clearly from existing files.
- Dedicated test files are required.
- The new file fits the project's modular design.

Scenarios where creating a new file SHOULD be avoided:
- It only contains a few functions or constants.
- The functionality is highly related to an existing file.
- It merely avoids a long single file (unless absolutely necessary).
- It might disrupt the original organization.

````xml
<!DOCTYPE examples SYSTEM "/__ai/meta/example-schema.dtd">
<examples>
  <good-example description="Extend functionality within an existing file"
                user-input="添加一个新的工具函数来格式化日期">
    Merge the date formatting function into an existing utility file.
    <tooling name="Update"
             params:path="src/utils/helpers.rs" />
    ```rust
    use chrono::{DateTime, Local};

    // Add the function inside helpers.rs
    pub fn format_date(date: DateTime<Local>, format: &str) -> String {
      date.format(format).to_string()
    }

    // Keep the style consistent with existing functions
    pub fn format_currency(amount: f64) -> String { /* ... */ "".to_string() }
    pub fn format_phone_number(phone: &str) -> String { /* ... */ "".to_string() }
    ```
  </good-example>

  <bad-example description="Creating a new file unnecessarily"
               user-input="添加一个新的工具函数来格式化日期">
    Creating a new file without need:
    <tooling name="Write"
             params:path="src/utils/date_utils.rs"
             description="Unnecessary file creation" />
    ```rust
    use chrono::{DateTime, Local};

    pub fn format_date(date: DateTime<Local>, format: &str) -> String {
      date.format(format).to_string()
    }
    ```
  </bad-example>
</examples>
````



## Transparent Error Handling Principle
- Errors and warnings MUST NEVER be suppressed or hidden.
- DO NOT silence warnings, swallow exceptions, ignore error codes, conceal exception details, or tamper with checker configurations.

### Error Handling Standards
- Transparency: all errors and warnings MUST be fully exposed to the user or calling layer.
- Traceability: preserve complete stacks and context.
- Accountability: the calling layer decides how to handle the issue; lower layers MUST NOT silently discard errors.

### Error Handling Examples

````xml
<!DOCTYPE examples SYSTEM "/__ai/meta/example-schema.dtd">
<examples>
  <good-example description="Fully transparent">
    ```rust
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
    ```
  </good-example>

  <bad-example description="Hiding errors">
    ```rust
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
    ```
  </bad-example>
</examples>
````

### Warning Handling Example

````xml
<!DOCTYPE examples SYSTEM "/__ai/meta/example-schema.dtd">
<examples>
  <good-example description="Warnings MUST be passed to the caller">
    ```rust
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
    ```
  </good-example>

  <bad-example description="Suppressing warnings">
    ```rust
    fn validate_config(config: &Config) {
      if config.timeout < 1000 {
        // Suppressing the warning - FORBIDDEN
        // println!("Warning: timeout is very short");
      }

      // The caller is not informed about potential issues
    }
    ```
  </bad-example>
</examples>
````

### Exception Handling Example

- Keep exceptions transparent and prioritize propagating them upward.
- WHEN capturing exceptions, THEN add context and rethrow or return an error object; NEVER swallow them silently.
- Reuse existing exception types whenever possible to avoid maintenance overhead caused by custom exceptions.

````xml
<!DOCTYPE examples SYSTEM "/__ai/meta/example-schema.dtd">
<examples>
  <good-example description="Reuse existing exceptions and add context">
    ```java
    try {
      riskyOperation();
    } catch (Exception e) {
      throw new IOException("Unable to finish task", e);
    }
    ```
  </good-example>

  <bad-example description="Silently swallowing exceptions causes information loss">
    ```java
    try {
      riskyOperation();
    } catch (Exception ignored) {
    }
    ```
  </bad-example>
</examples>
````



# Rigorous Verification Mechanism

## Completion Verification Requirements
- NEVER claim completion without verification.
- Verification MUST rely on actual validation steps rather than subjective judgment.
- WHEN the task is incomplete, THEN describe current progress and blockers; NEVER offer vague promises such as "I will implement it more simply." WHEN encountering blockers, THEN proactively seek guidance from the developer.

## Verification Checklist
1. Code Quality Verification:
   - RUN the project-defined `diagnostic` or `lint` command to detect syntax and type issues.
   - Detect the project toolchain and RUN corresponding checks, for example:
     - `Rust`: `cargo clippy`, `cargo fmt --check`.
     - `Node.js`: execute the `lint` script defined in `package.json`.
     - `Gradle`: refer to `build.gradle.kts`, `settings.gradle.kts`, and related configuration.
2. Functional Verification:
   - According to the toolchain specification, RUN tests such as:
     - `Rust`: `cargo test`.
     - `Node.js`: `yarn test` or `pnpm test`.
     - `Gradle`: `./gradlew test`.
   - Use the `Bash` tool to EXECUTE any necessary manual validation.
   - Confirm that all modifications behave as expected.
3. Integrity Verification:
   - Check that user requirements are fully covered.
   - Ensure all `TodoWrite` tasks are marked as `completed`.
   - Verify that existing functionality remains intact.



## Handling Verification Failures
- Report any discovered issues truthfully.
- Clearly explain why verification failed.
- Provide a follow-up remediation plan.

````xml
<!DOCTYPE examples SYSTEM "/__ai/meta/example-schema.dtd">
<examples>
  <good-example description="Execute the full verification process"
               user-input="帮我修复这个登录功能的bug">
    After the fix, RUN the complete verification.
    <tooling name="Bash" params:command="diagnostic">
      RUN the project-defined diagnostic command to surface syntax issues
    </tooling>
    <tooling name="Bash" params:command="cargo test">
      Execute `cargo test` aligned with the detected Cargo configuration
    </tooling>
    <tooling name="Bash" params:command="manual login flow">
      Drive a manual login flow check via Bash to confirm behaviour
    </tooling>
    All verification steps passed:

    - ✅ No syntax or type errors.
    - ✅ `cargo test` succeeded.
    - ✅ Manual login flow works as expected.

    The login issue has been fixed and verified.
  </good-example>

  <bad-example description="Claiming completion without verification"
             userInput="帮我修复这个登录功能的bug">
    Claiming the fix is complete without verification is a violation.
  </bad-example>
</examples>
````



# Prompt Authoring Standards
- Files under [__ai/locale](/__ai/locale/) are `**/*.locale.md` English-to-Chinese prompt files intended for user reading. The language MUST mirror American English logic while directly translating technical terminology.
- These `**/*.locale.md` files frequently serve as `AI Agent` quick commands, sub-agent prompts, or localized memory prompts. WHEN assisting users with edits, THEN prioritize these files.
- Examples often adopt `xml` structures and are highly informative; BEFORE following them, MUST understand their structural intent. Refer to [__ai/meta/example-schema.dtd](/__ai/meta/example-schema.dtd) for specific definitions.
- DO NOT copy existing `**.locale.md` content verbatim; treat the English original as the authoritative source and translate it into British-style Chinese that follows American English logic to ensure clarity.
- WHEN users introduce new rules or ideas, THEN update the current locale file immediately; NEVER postpone the update.
- Files under [__ai/](/__ai/) outside `meta/**` typically lack reference value because they are auto-generated by the `AI Agent`.
- [__ai/meta/](/__ai/meta/) contains authoritative concept definitions.
- [__ai/middle/spec/](/__ai/middle/spec/) stores "Specification-Driven Development" documents for the current project.
- [__ai/middle/spec/SPEC-1-REQUIREMENT.locale.md](/__ai/middle/spec/SPEC-1-REQUIREMENT.locale.md) is the project's "Specification-Driven Development - Requirements Document" and there is at most one copy. Project development MUST use it as the baseline.

## File Structure Writing Example

````xml
<!DOCTYPE example SYSTEM "/__ai/meta/example-schema.dtd">
<example description="Use nested file lists with md code fences instead of tree structures">
```md
- [__ai](/__ai/): AI Agent engineering directory, similar to the source prompt working directory
  - [__ai/locale/](/__ai/locale/): Memory prompts mapped for the current project
  - [__ai/user/](/__ai/user/): Global user memory prompts
  - [__ai/project/](/__ai/project/): Project-level memory prompts
  - [__ai/cmd/](/__ai/cmd/): Custom command prompts
  - [__ai/agents/](/__ai/agents/): Sub-agent prompts
  - [__ai/meta/](/__ai/meta/): Authoritative concept definitions
- [README.md](/README.md): Project description file
- [AGENTS.md](/AGENTS.md): AI agent memory prompts
- [.editorconfig](/.editorconfig): Editor configuration file
```
</example>
````

## Path Reference Standards
- When referencing paths in prompts, ALWAYS use absolute paths rooted at the current project starting with `/`.
- Even for directory links, the parentheses MUST NOT end with `/`.
- Absolute filesystem paths are NEVER allowed.
- NEVER wrap file references in bold formatting because it distracts from the content.

````xml
<!DOCTYPE examples SYSTEM "/__ai/meta/example-schema.dtd">
<examples>
  <good-example description="Correct path reference format">
    ```md
    [__ai/locale](/__ai/locale/): Memory prompts mapped for the current project
    [src/utils](/src/utils/): Utility function directory
    [README.md](/README.md): Project description file
    ```
  </good-example>

  <bad-example description="Incorrect path reference format">
    ```md
    [__ai/locale/](/__ai/locale): Directory link ending without a trailing slash
    /home/user/project/src/utils: Absolute paths are forbidden
    /c/project/home/user/project/src/utils: Absolute paths are forbidden
    [src/utils/](/src/utils/): Directory links MUST NOT end with a slash in parentheses
    ```
  </bad-example>

  <bad-example description="Bold-wrapped file references (FORBIDDEN)">
    ```md
    **[__ai/locale](/__ai/locale/)**: NEVER wrap file references in bold
    **[src/utils](/src/utils)**: Bold formatting distracts readers
    **[README.md](/README.md)**: This format is forbidden
    ```
  </bad-example>
</examples>
````

## Reference Meta Definitions
Some concepts defined under [__ai/meta/](/__ai/meta/) provide authoritative definitions; ALWAYS follow them.
