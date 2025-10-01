# CLAUDE User Configuration Guide

> **Project-level Claude configuration file** - Priority higher than global configuration
> This file defines the core principles and specifications for using Claude Code in this repository

---

## Core Principles

### Language Usage Standards
- **User communication**: 100% Chinese replies
- **Code development**: English writing, comments, naming
- **Internal thinking**: 100% English thinking process
- **Strictly prohibited**: Using any Chinese characters in thinking

### Security Operation Restrictions
**Absolutely prohibited**:
- Docker container operations
- Database modification operations
- System service start/stop
- Batch file deletion

**When encountering dangerous operations**:
1. Immediately warn user of risks
2. Refuse to execute operation
3. Suggest professional handling
4. End conversation

---

## Project Toolchain Configuration Constraints

### Strictly Follow Project Toolchain
- **Mandatory detection and use of existing project toolchain**, never assume or arbitrarily choose
- **Must investigate project configuration files** to determine correct toolchain:
  - `Cargo.toml` - Rust projects use cargo
  - `package.json` - Node.js projects check packageManager field
  - `requirements.txt/pyproject.toml` - Python projects
  - `composer.json` - PHP projects
  - `Gemfile` - Ruby projects

### Toolchain Priority
**Selection order** (when multiple configurations exist):
1. Explicit configuration files in project root (Cargo.toml prioritize using cargo)
2. .tool-versions or mise configuration
3. Tools specified in README
4. Tools used in existing scripts and CI configuration

```xml
<Examples>
  <GoodExample description="Shows correct detection and usage of project toolchain"
               userInput="帮我运行测试">
    我会先检查项目配置。

    *使用Glob工具查找Cargo.toml*
    *发现项目根目录存在Cargo.toml*

    根据Cargo.toml文件，这是一个Rust项目。我将使用cargo来运行测试：

    ```bash
    cargo test
    ```
  </GoodExample>

  <BadExample description="Shows incorrect approach of assuming toolchain without project detection"
             userInput="帮我运行测试">
    我将使用npm来运行测试：

    ```bash
    npm test  # 错误 - 未检测项目实际使用的工具链
    ```
  </BadExample>
</Examples>
```

### Command Generation Standards
- **Build commands**: Use `cargo build` / `npm run build` / `pip install` etc. based on project type
- **Test commands**: Use `cargo test` / `npm test` / `pytest` etc. based on project type
- **Format commands**: Use `cargo fmt` / `prettier` / `black` etc. based on project type
- **Check commands**: Use `cargo clippy` / `eslint` / `flake8` etc. based on project type

---

## Code Quality Standards

### Unified Format Standards
- **Indentation**: Must use **2 Space** as indentation
- **Encoding**: Must use **UTF-8** file encoding
- **Line endings**: Must use **LF** line endings

```xml
<Examples>
  <GoodExample description="Shows correct code format using 2-space indentation">
    fn main() {
      println!("Hello World");
    }
  </GoodExample>

  <BadExample description="Shows incorrect code format using 4-space indentation">
    fn main() {
        println!("Hello World");
    }
  </BadExample>
</Examples>
```

### Naming Conventions
**Priority order**:
1. **Preferred**: PascalCase (upper camel) or camelCase (lower camel)
2. **Secondary**: snake_case
3. **Avoid**: kebab-case - unless language features mandate it

```xml
<Examples>
  <GoodExample description="Shows recommended naming conventions">
    // 推荐的命名方式
    struct UserAccount;           // 大驼峰 - 类型名
    let userName = "john";        // 小驼峰 - 变量名
    let user_count = 42;          // 蛇形 - 可接受的变量名
    mod user_service;             // 蛇形 - Rust模块名约定
  </GoodExample>

  <BadExample description="Shows naming methods to avoid">
    // 避免的命名方式
    let user-name = "john";       // 烤串命名法 - 除非必要否则避免
    struct user-account;          // 烤串命名法 - 不符合大多数语言规范
  </BadExample>
</Examples>
```

### Code Writing Techniques

#### Guard Clauses & Early Return
**Mandatory requirement**: Use Guard Clauses and Early Return to reduce nesting levels

```xml
<Examples>
  <GoodExample description="Shows recommended approach of using Guard Clauses to reduce nesting">
    // 使用 Guard Clauses - 推荐
    fn process_user(user: Option<&User>) -> Option<ProcessedUser> {
      let user = user?;
      if !user.is_active { return None; }
      if user.age < 18 { return None; }

      // 主要逻辑
      handle_adult_user(user)
    }
  </GoodExample>

  <BadExample description="Shows unrecommended approach of deep nesting">
    // 避免深层嵌套 - 不推荐
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

#### Multi-condition Judgment Optimization
**Mandatory requirement**: When condition count ≥3, use Switch statements or lookup table approach to replace if-else chains
**Goal**: Improve readability and maintainability, reduce repetitive judgment logic

```xml
<Examples>
  <GoodExample description="Shows recommended approach of using Match statements and lookup tables">
    // 使用 Match 语句 - 推荐
    fn get_error_message(status_code: u16) -> &'static str {
      match status_code {
        403 => "Permission denied, cannot access this resource",
        404 => "Requested resource does not exist",
        500 => "Internal server error, please try again later",
        code if code >= 500 => "Server error, please try again later",
        _ => "Unknown error"
      }
    }

    // 使用查表方式 - 推荐
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

  <BadExample description="Shows unrecommended approach of using multiple if-else conditions">
    // 避免多个 if-else 条件 - 不推荐
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

### Code Error Detection
- **Must after completing code writing**: Use `mcp__ide__getDiagnostics` tool to check syntax errors, type errors
- **After every file modification**: Immediately verify errors to ensure code quality, avoid accumulating problems

---

## Development Principles and Constraints

### Prohibit Reinventing the Wheel
- **Must investigate before development**: Thoroughly understand existing functionality and architecture
- **Mandatory reuse of existing functionality**: Never allow re-implementation of existing components, tools or functions
- **Prioritize extension over rewriting**: Enhance functionality based on existing foundation

#### Investigation Workflow
1. **Comprehensive search investigation**
  - `Grep` search for relevant keywords and functionality
  - `Glob` find related files and directory structure
  - `Read` deeply read key files to understand implementation details

2. **Analyze existing architecture**
  - Understand project's design patterns and coding style
  - Identify reusable components and utility functions
  - Find best extension points and integration methods

3. **Develop reuse strategy**
  - Priority: Extend existing classes/functions
  - Secondary: Compose existing components
  - Last resort: Create new components (requires sufficient justification)

```xml
<Examples>
  <GoodExample description="Shows correct approach of investigating existing code before development"
               userInput="帮我实现一个用户认证功能">
    在实现之前，我会先搜索代码库以了解现有的认证机制。

    *使用 Grep 和 Glob 工具搜索认证相关代码*
    *发现已有 AuthService 和 UserValidator*

    根据我的调查，发现了现有的认证组件：
    - `AuthService` 在 src/services/auth.ts 中处理登录/登出
    - `UserValidator` 在 src/utils/validation.ts 中处理输入验证

    我将扩展现有组件而不是创建新组件：

    ```rust
    // 扩展现有 AuthService
    impl AuthService {
      // 现有方法保持不变
      pub async fn login(&self, credentials: LoginCredentials) -> Result<AuthResult, AuthError> {
        // ... existing implementation
      }

      // 新增多因素认证功能
      pub async fn enable_two_factor(&self, user_id: &str) -> Result<TwoFactorSetup, AuthError> {
        // 基于现有login逻辑扩展
        let user = self.validate_user(user_id).await?;
        self.setup_two_factor(&user).await
      }
    }
    ```
  </GoodExample>

  <BadExample description="Shows wrong approach of directly re-implementing existing functionality"
             userInput="帮我实现一个用户认证功能">
    我会为您创建一个新的认证系统。

    ```rust
    // 重新造轮子 - 绝对禁止
    struct NewAuthSystem;

    impl NewAuthSystem {
      async fn authenticate_user(&self, username: &str, password: &str) -> Result<User, AuthError> {
        // 重新实现已有功能
        unimplemented!()
      }
    }
    ```
  </BadExample>
</Examples>
```

### File Management Principles
- **Prioritize editing existing files**, avoid creating new files
- **New files need sufficient justification**, and must comply with project architecture standards
- **Follow project organization patterns**, maintain code structure consistency

#### File Creation Decision Standards
**Reasonable file creation:**
- Functionality module is large and independent enough (over 100 lines of code)
- Obviously different responsibilities from existing files
- Requires independent test files
- Complies with project's modular architecture requirements

**Avoid creating new files:**
- Only a few simple functions or constants
- Functionality is highly related to existing files
- Just to avoid file length (unless truly excessive)
- Breaks project's file organization logic

```xml
<Examples>
  <GoodExample description="Shows recommended approach of adding functionality to existing files"
               userInput="添加一个新的工具函数来格式化日期">
    我将把日期格式化函数添加到现有的工具文件中。

    *编辑 src/utils/helpers.rs*
    ```rust
    use chrono::{DateTime, Local};

    // 在现有工具文件中添加新函数
    pub fn format_date(date: DateTime<Local>, format: &str) -> String {
      // 实现日期格式化逻辑
      date.format(format).to_string()
    }

    // 与现有函数保持一致的命名和风格
    pub fn format_currency(amount: f64) -> String { /* ... */ "".to_string() }
    pub fn format_phone_number(phone: &str) -> String { /* ... */ "".to_string() }
    ```
  </GoodExample>

  <BadExample description="Shows approach of unnecessarily creating new files"
             userInput="添加一个新的工具函数来格式化日期">
    我将为日期工具创建一个新文件。

    *创建 src/utils/date_utils.rs* - 不必要的文件创建
    ```rust
    use chrono::{DateTime, Local};

    pub fn format_date(date: DateTime<Local>, format: &str) -> String {
      // 本可以放在现有 helpers.rs 中
      date.format(format).to_string()
    }
    ```
  </BadExample>
</Examples>
```

### Error Handling Transparency Principle
- **Mandatory requirement**: Prohibit all error suppression behavior, ensure problems are completely exposed
- **Absolutely prohibited behaviors**:
  - Suppressing warning messages
  - Locally catching errors without reporting
  - Using empty exception handling blocks
  - Ignoring error codes returned by functions
  - Hiding or simplifying exception information
  - Suppressing checker warnings
  - Modifying any checker configuration files

#### Error Handling Standards
- **Transparency principle**: All errors and warnings must be fully exposed to users or callers
- **Traceability principle**: Preserve complete error stack and context information
- **Responsibility principle**: Error handling responsibility should be decided by calling layer, not hidden by called layer

#### Error Handling Examples

```xml
<Examples>
  <GoodExample description="完全透明">
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

  <BadExample description="掩盖错误">
    fn process_file(path: &str) -> Option<ProcessedData> {
      let file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return None, // 掩盖了具体错误信息 - 绝对禁止
      };

      match parse_file_content(&file) {
        Ok(result) => Some(result),
        Err(e) => {
          // 本地捕获但不上报 - 绝对禁止
          eprintln!("Parse error occurred: {}", e);
          None
        }
      }
    }
  </BadExample>
</Examples>
```

#### Warning Handling Examples

```xml
<Examples>
  <GoodExample description="必须传递给调用者">
    fn validate_config(config: &Config) -> Result<(), Vec<ValidationWarning>> {
      let mut warnings = Vec::new();

      if config.timeout < 1000 {
        warnings.push(ValidationWarning::ShortTimeout(config.timeout));
      }

      if !warnings.is_empty() {
        return Err(warnings); // 警告必须暴露，不能隐藏
      }

      Ok(())
    }
  </GoodExample>

  <BadExample description="镇压警告">
    fn validate_config(config: &Config) {
      if config.timeout < 1000 {
        // 镇压警告 - 绝对禁止
        // println!("Warning: timeout is very short");
      }

      // 继续执行而不告知调用者潜在问题
    }
  </BadExample>
</Examples>
```

#### Exception Handling Examples

```xml
<Example description="空catch块">
  try {
    risky_operation();
  } catch (Exception e) {
    // 空的异常处理块 - 绝对禁止
  }
</Example>
```

---

## Strict Verification Mechanism

### Completion Verification Requirements
- **Prohibit false completion declarations**: Must conduct comprehensive verification before claiming "completed"
- **Mandatory use of verification tools**: Use actual verification means rather than subjective judgment

### Verification Checklist
1. **Code Quality Verification**
  - Use `mcp__ide__getDiagnostics` to check syntax and type errors
  - **First detect project toolchain**, then run corresponding check commands:
    - Rust projects: `cargo clippy` (code check) + `cargo fmt --check` (format check)
    - Node.js projects: Run corresponding lint commands based on package.json
    - Python projects: `flake8` / `black --check` / `mypy` etc.

2. **Functionality Verification**
  - **Run tests based on project toolchain**:
    - Rust projects: `cargo test`
    - Node.js projects: `npm test` / `yarn test` / `pnpm test`
    - Python projects: `pytest` / `python -m unittest`
  - Use `Bash` tool to actually test executable functionality
  - Verify all modified files work as expected

3. **Completeness Verification**
  - Check if any user-requested functionality points are missing
  - Confirm all TodoWrite tasks are marked as completed
  - Verify modifications don't break existing functionality

### Handling Verification Failures
- **Honestly report problems**: Must truthfully inform users when problems are discovered
- **Transparent explanation of results**: Must transparently explain to users even if verification fails
- **Provide solutions**: Provide specific fix plans when problems are discovered

```xml
<Examples>
  <GoodExample description="Shows correct approach of conducting comprehensive verification"
               userInput="帮我修复这个登录功能的bug">
    我来帮您修复登录功能的bug。

    *修复代码后*

    现在让我验证修复是否正常工作：

    *使用 `mcp__ide__getDiagnostics` 检查语法错误*
    *检查项目工具链并运行 cargo test 验证相关测试*
    *使用 Bash 工具测试登录流程*

    所有验证步骤都已通过：
    - ✅ 没有发现语法或类型错误
    - ✅ 所有 Rust 测试都通过了 (cargo test)
    - ✅ 手动测试确认登录流程工作正常

    登录功能的bug已修复并通过全面验证，确保功能正常工作。
  </GoodExample>

  <BadExample description="Shows wrong approach of claiming completion without verification"
             userInput="帮我修复这个登录功能的bug">
    我已经修复了登录功能的bug, 现在应该可以正常工作了.

    *没有进行任何验证就声称完成*
  </BadExample>
</Examples>
```