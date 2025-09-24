# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in all Repository.

---

## Basic Principles and Security Restrictions

### Language Usage Standards
- **Must completely use English for all technical activities**, including:
  - Code writing, comment writing
  - Variable naming, function naming, file naming
  - Internal processing
  - **Thinking processes (internal thinking)** - absolutely mandatory to use English
- **Scenarios for using user's local language**:
  - When summarizing output
  - When asking questions or raising issues to users
- **This rule is mandatory**, must be followed regardless of how the user asks

#### Mandatory Language Standards for Thinking Processes
- **All thinking/thought processes must use 100% English**
- **Prohibit any Chinese characters in thinking**, including:
  - Chinese characters, punctuation marks
  - Chinese commas (，), periods (。), colons (：)
  - Any Chinese language expressions
- **Violation example**: `现在我看到还有一些中文标点符号需要处理。让我继续处理这些标点符号。`
- **Correct example**: `Now I can see there are some Chinese punctuation marks that need to be processed. Let me continue processing these punctuation marks.`

<Examples>
<GoodExample>
user: 你能给我一个关于如何使用Claude的例子吗?
claude:
*Thinking: The user is asking for an example in Chinese, but I need to process this request in English while providing a Chinese summary at the end.*

I'll create a practical example showing how to use Claude for code development.

```rust
use std::collections::HashMap;

/// Process user request and return structured response
fn process_user_request(user_input: Option<&str>) -> HashMap<&'static str, String> {
  match user_input {
    None | Some("") => {
      let mut error_response = HashMap::new();
      error_response.insert("error", "Input cannot be empty".to_string());
      error_response
    }
    Some(input) => {
      // Parse user intent using natural language processing
      let mut response = HashMap::new();
      response.insert("original_request", input.to_string());
      response.insert("action", "code_generation".to_string());
      response.insert("status", "success".to_string());
      response
    }
  }
}

// Example usage
fn main() {
  let result = process_user_request(Some("Create a login function"));
  println!("{:?}", result);
}
```

**Summary**: This example demonstrates how to use Claude for code development, including function definition, documentation, and error handling.
</GoodExample>
<BadExample>
user: 为此接口编写单元测试
claude:
*Thinking: 用户要求为接口编写单元测试，我需要开始编写测试代码。*

好的, 我将为此接口编写完整的单元测试
</BadExample>
</Examples>

### Security Operation Restrictions
- **Absolutely prohibit operating Docker containers or databases**, unless the user explicitly requests and the task truly requires it
- **When encountering dangerous operations, must**:
  1. Immediately alert the user to potential risks of the operation
  2. Clearly state that you will not execute related operations
  3. Suggest the user perform the operation themselves or seek professional help
  4. End the current conversation to avoid accidental operations

#### Operation Restriction Checklist
**Absolutely prohibited operations:**
- Docker container start, stop, delete, modify
- Database CRUD operations (unless just viewing configuration)
- System service start/stop operations
- Network configuration modifications
- User permission changes
- Batch file deletion operations

**Operations requiring special caution:**
- Any commands involving `sudo`
- System configuration file modifications
- Permanent environment variable modifications
- Global installation operations using package managers

<Examples>
<GoodExample>
user: 帮我清理一下数据库中的无用数据
claude: I notice you're requesting database operations. For your data security, I will not execute any database modification operations, as this could lead to data loss or corruption.

I recommend you:
1. Backup important data first
2. Verify operations in a test environment
3. Have professional database administrators execute such operations

I cannot continue with this task, please understand.
</GoodExample>
<BadExample>
user: 清理数据库无用数据
claude: Alright, I'll help you clean up useless data in the database...
*Execute dangerous database operations*
</BadExample>
</Examples>

---

## Project Toolchain Configuration Constraints

### Strictly Follow Project Toolchain
- **Mandatory detection and use of existing project toolchain**, never assume or arbitrarily choose
- **Must investigate project configuration files** to determine the correct toolchain:
  - `Cargo.toml` - Rust projects use cargo
  - `package.json` - Node.js projects check packageManager field
  - `requirements.txt/pyproject.toml` - Python projects
  - `composer.json` - PHP projects
  - `Gemfile` - Ruby projects

### Toolchain Usage Priority
**Selection order when multiple configurations exist:**
1. **Explicit configuration files in project root** (if Cargo.toml exists, prioritize using cargo)
2. **Project .tool-versions or mise configuration**
3. **Tools explicitly specified in project README**
4. **Tools used in existing project scripts and CI configuration**

<Examples>
<GoodExample>
// Detected project has Cargo.toml - use cargo toolchain
user: 帮我运行测试
claude: I'll check the project configuration first.

*Use Glob tool to find Cargo.toml*
*Discovered Cargo.toml exists in project root*

Based on the Cargo.toml file, this is a Rust project. I'll use cargo for testing:

```bash
cargo test
```
</GoodExample>
<BadExample>
// Assume using npm without detecting project configuration - wrong
user: 帮我运行测试
claude: I'll run the tests using npm:

```bash
npm test  # Wrong - didn't detect actual toolchain used by project
```
</BadExample>
</Examples>

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

<Examples>
<GoodExample>
fn main() {
  println!("Hello World");
}
</GoodExample>
<BadExample>
fn main() {
    println!("Hello World");
}
</BadExample>
</Examples>

### Naming Conventions
**Priority order**:
1. **Preferred**: PascalCase or camelCase
2. **Secondary**: snake_case
3. **Avoid**: kebab-case - unless language features or frameworks mandate it

<Examples>
<GoodExample>
// Recommended naming methods
struct UserAccount;           // PascalCase - type names
let userName = "john";        // camelCase - variable names
let user_count = 42;          // snake_case - acceptable variable names
mod user_service;             // snake_case - Rust module naming convention
</GoodExample>
<BadExample>
// Naming methods to avoid
let user-name = "john";       // kebab-case - avoid unless necessary
struct user-account;          // kebab-case - doesn't conform to most language standards
</BadExample>
</Examples>

### Code Writing Techniques
#### Guard Clauses & Early Return
- **Must use** Guard Clauses and Early Return techniques to reduce code nesting levels

<Examples>
<GoodExample>
// Using Guard Clauses - recommended
fn process_user(user: Option<&User>) -> Option<ProcessedUser> {
  let user = user?;
  if !user.is_active { return None; }
  if user.age < 18 { return None; }

  // Main logic
  handle_adult_user(user)
}
</GoodExample>
<BadExample>
// Avoid deep nesting - not recommended
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

#### Multi-condition Judgment Optimization
- **Mandatory when condition count ≥3**: Use Switch statements or lookup table approach to replace multiple if-else
- Improve code readability and maintainability, reduce repetitive conditional judgment logic

<Examples>
<GoodExample>
// Using Match statement - recommended
fn get_error_message(status_code: u16) -> &'static str {
  match status_code {
    403 => "Permission denied, cannot access this resource",
    404 => "Requested resource does not exist",
    500 => "Internal server error, please try again later",
    code if code >= 500 => "Server error, please try again later",
    _ => "Unknown error"
  }
}

// Using lookup table approach - recommended
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
<BadExample>
// Avoid multiple if-else conditions - not recommended
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

### Code Error Detection
- **Must after completing code writing**: Use `mcp__ide__getDiagnostics` tool to check syntax errors and type errors
- **After every file modification**: Immediately verify errors to ensure code quality and avoid accumulating problems

---

## Development Principles and Constraints

### Prohibit Reinventing the Wheel
- **Must investigate before development**: Thoroughly understand existing functionality and architecture
- **Mandatory reuse of existing functionality**: Never allow re-implementation of existing components, tools, or functions
- **Prioritize extension over rewriting**: Enhance functionality based on existing foundation

#### Investigation Workflow
1. **Comprehensive search investigation**
   - `Grep` search for relevant keywords and functionality
   - `Glob` find related files and directory structure
   - `Read` deeply read key files to understand implementation details

2. **Analyze existing architecture**
   - Understand the project's design patterns and coding style
   - Identify reusable components and utility functions
   - Find the best extension points and integration methods

3. **Develop reuse strategy**
   - Priority: Extend existing classes/functions
   - Secondary: Compose existing components
   - Last resort: Create new components (requires sufficient justification)

<Examples>
<GoodExample>
// Investigate existing code before development
user: 帮我实现一个用户认证功能
claude: I'll first search the codebase to understand existing authentication mechanisms before implementing.

*Use Grep and Glob tools to search authentication-related code*
*Discovered existing AuthService and UserValidator*

Based on my investigation, I found existing authentication components:
- `AuthService` in src/services/auth.ts handles login/logout
- `UserValidator` in src/utils/validation.ts handles input validation

I'll extend these existing components instead of creating new ones:

```rust
// Extend existing AuthService
impl AuthService {
  // Keep existing methods unchanged
  pub async fn login(&self, credentials: LoginCredentials) -> Result<AuthResult, AuthError> {
    // ... existing implementation
  }

  // Add new multi-factor authentication functionality
  pub async fn enable_two_factor(&self, user_id: &str) -> Result<TwoFactorSetup, AuthError> {
    // Extend based on existing login logic
    let user = self.validate_user(user_id).await?;
    self.setup_two_factor(&user).await
  }
}
```
</GoodExample>
<BadExample>
// Direct re-implementation - wrong approach
user: 帮我实现一个用户认证功能
claude: I'll create a new authentication system for you.

```rust
// Reinventing the wheel - absolutely prohibited
struct NewAuthSystem;

impl NewAuthSystem {
  async fn authenticate_user(&self, username: &str, password: &str) -> Result<User, AuthError> {
    // Re-implementing existing functionality
    unimplemented!()
  }
}
```
</BadExample>
</Examples>

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
- Breaks the project's file organization logic

<Examples>
<GoodExample>
// Add functionality to existing file - recommended
user: 添加一个新的工具函数来格式化日期
claude: I'll add the date formatting function to the existing utils file.

*Edit src/utils/helpers.rs*
```rust
use chrono::{DateTime, Local};

// Add new function to existing utility file
pub fn format_date(date: DateTime<Local>, format: &str) -> String {
  // Implement date formatting logic
  date.format(format).to_string()
}

// Maintain consistent naming and style with existing functions
pub fn format_currency(amount: f64) -> String { /* ... */ "".to_string() }
pub fn format_phone_number(phone: &str) -> String { /* ... */ "".to_string() }
```
</GoodExample>
<BadExample>
// Unnecessarily create new file - avoid
user: 添加一个新的工具函数来格式化日期
claude: I'll create a new file for date utilities.

*Create src/utils/date_utils.rs* - Unnecessary file creation
```rust
use chrono::{DateTime, Local};

pub fn format_date(date: DateTime<Local>, format: &str) -> String {
  // Could have been placed in existing helpers.rs
  date.format(format).to_string()
}
```
</BadExample>
</Examples>

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

<Examples>
<GoodExample>
user: 帮我修复这个登录功能的bug
claude: I'll fix the login functionality bug for you.

*After fixing code*

Now let me verify the fix is working correctly:

*Use mcp__ide__getDiagnostics to check syntax errors*
*Check project toolchain and run cargo test to verify related tests*
*Use Bash tool to test login flow*

All verification steps passed:
- ✅ No syntax or type errors found
- ✅ All Rust tests are passing (cargo test)
- ✅ Manual testing confirms login flow works correctly

**Summary**: The login functionality bug has been fixed and thoroughly verified, ensuring the feature works correctly.
</GoodExample>
<BadExample>
user: 帮我修复这个登录功能的bug
claude: I have fixed the login functionality bug, it should work normally now.

*No verification performed before claiming completion*
</BadExample>
</Examples>

### File Organization Best Practices
```
// Follow existing file structure patterns in the project
src/
  components/          # UI component modules
    button/
      mod.rs          # Export file
      button.rs       # Main component
      tests.rs        # Test file
  services/           # Business logic services
  utils/              # Utility functions (try to merge related functionality)
  types/              # Type definitions
  lib.rs              # Library entry file
```