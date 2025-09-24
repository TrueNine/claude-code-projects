# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in all Repository.


## **English is the Best Language (Unquestionable)**

- Must **completely use English for all activities**, including but not limited to:
  - Code writing
  - Comment writing
  - Variable naming
  - Function naming
  - File naming
  - Internal processing
- **Use user's preferred local language scenarios**:
  - When summarizing output
  - When asking questions or raising issues to users
- This rule is mandatory and must be followed regardless of how the user asks

<examples>
<good-example>
user: Can you give me an example of how to use Claude?
claude: I'll create a practical example showing how to use Claude for code development.

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
</good-example>
<bad-example>
user: Write unit tests for this interface
claude: Okay, I will write complete unit tests for this interface
</bad-example>
</examples>


## Unified Code Format Standards

- Must always use **2 Space** as indentation

<examples>
<good-example>
fn main() {
  println!("Hello World");
}
</good-example>
<bad-example>
fn main() {
    println!("Hello World");
}
</bad-example>
</examples>

- Must always use **UTF-8** as file encoding
- Must always use **LF** as line ending


## Recommended Code Writing Techniques

- Always use **Guard Clauses** and **Early Return** techniques to reduce code nesting levels

<examples>
<good-example>
// Use Guard Clauses - Recommended
fn process_user(user: Option<&User>) -> Option<ProcessedUser> {
  let user = user?;
  if !user.is_active { return None; }
  if user.age < 18 { return None; }

  // Main logic
  handle_adult_user(user)
}
</good-example>
<bad-example>
// Avoid deep nesting - Not recommended
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

- Multi-condition judgment must use **Switch statements** or **lookup table approach** to replace multiple if-else conditions
  - This rule is mandatory when the number of judgment conditions is **≥3**
  - Improve code readability and maintainability
  - Reduce repetitive conditional judgment logic

<examples>
<good-example>
// Use Match statement - Recommended
fn get_error_message(status_code: u16) -> &'static str {
  match status_code {
    403 => "Permission denied, cannot access this resource",
    404 => "Requested resource does not exist",
    500 => "Internal server error, please try again later",
    code if code >= 500 => "Server error, please try again later",
    _ => "Unknown error"
  }
}

// Use lookup table approach - Recommended
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
<bad-example>
// Avoid multiple if-else conditions - Not recommended
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

## Proactive Code Error Detection

- After completing code writing, must use **`mcp__ide__getDiagnostics`** tool to check each file for syntax errors, type errors, and other issues
- After every file modification, immediately verify errors to ensure code quality and avoid accumulating problems

## Absolutely **Prohibit Reinventing the Wheel**

- **Must thoroughly investigate the project first**, deeply understand existing functionality and architecture before starting development
- **Mandatory reuse of existing functionality**, absolutely no re-implementation of existing components, tools, or functions
- **Prioritize extension over rewriting**, enhance functionality based on existing foundation rather than starting from scratch

<examples>
<good-example>
// Investigate existing code before development
user: Help me implement a user authentication feature
claude: I'll first search the codebase to understand existing authentication mechanisms before implementing.

*Uses Grep and Glob tools to search authentication-related code*
*Discovers existing AuthService and UserValidator*

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

  // Add new multi-factor authentication feature
  pub async fn enable_two_factor(&self, user_id: &str) -> Result<TwoFactorSetup, AuthError> {
    // Extend based on existing login logic
    let user = self.validate_user(user_id).await?;
    self.setup_two_factor(&user).await
  }
}
```
</good-example>
<bad-example>
// Directly re-implement - Wrong approach
user: Help me implement a user authentication feature
claude: I'll create a new authentication system for you.

```rust
// Reinventing the wheel - Absolutely prohibited
struct NewAuthSystem;

impl NewAuthSystem {
  async fn authenticate_user(&self, username: &str, password: &str) -> Result<User, AuthError> {
    // Re-implementing existing functionality
    unimplemented!()
  }
}
```
</bad-example>
</examples>

### Workflow for Investigating Existing Functionality

1. **Use search tools for comprehensive investigation**
   - `Grep` search for relevant keywords and functionality
   - `Glob` find related files and directory structure
   - `Read` thoroughly read key files to understand implementation details

2. **Analyze existing architecture patterns**
   - Understand the project's design patterns and coding style
   - Identify reusable components and utility functions
   - Find the best extension points and integration methods

3. **Develop reuse strategy**
   - Prioritize extending existing classes/functions
   - Second choice: compose existing components
   - Last resort: create new components (requires sufficient justification)

## Strict Completion Verification Mechanism

- **Prohibit false completion declarations**: Must conduct comprehensive verification before claiming "completed"
- **Mandatory use of verification tools**: Must use actual verification means rather than subjective judgment

### Verification Steps Required Before Completion

1. **Code Quality Verification**
  - Use `mcp__ide__getDiagnostics` to check syntax and type errors
  - Run project's lint command to check code style
  - Run typecheck command to verify type correctness

2. **Functionality Verification**
  - Run relevant test cases to confirm functionality works
  - Use `Bash` tool to actually test executable functionality
  - Verify all modified files actually work as expected

3. **Completeness Verification**
  - Check if any user-requested functionality points are missing
  - Confirm all tasks in TodoWrite are marked as completed
  - Verify modifications don't break existing functionality

<examples>
<good-example>
user: Help me fix this login functionality bug
claude: I'll fix the login functionality bug for you.

*After fixing the code*

Now let me verify the fix is working correctly:

*Uses mcp__ide__getDiagnostics to check syntax errors*
*Runs npm run test to verify related tests*
*Uses Bash tool to test login flow*

All verification steps passed:
- ✅ No syntax or type errors found
- ✅ All login-related tests are passing
- ✅ Manual testing confirms login flow works correctly

**Summary**: The login functionality bug has been fixed and thoroughly verified, ensuring the feature works correctly.
</good-example>
<bad-example>
user: Help me fix this login functionality bug
claude: I have fixed the login functionality bug, it should work normally now.

*No verification performed before claiming completion*
</bad-example>
</examples>

### Handling Verification Failures

- **Honestly report when problems are found**: If verification discovers issues, must truthfully inform the user
- **Must not hide verification results**: Even if verification fails, must transparently explain to the user
- **Provide solutions**: When problems are found, provide specific fix plans

## Create Files as Sparingly as Possible

- **Prioritize editing existing files** rather than creating new files
- **New file creation requires sufficient justification** and must comply with project architecture standards
- **Follow project file organization patterns** to maintain code structure consistency

<examples>
<good-example>
// Add functionality to existing file - Recommended
user: Add a new utility function to format dates
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
</good-example>
<bad-example>
// Unnecessarily create new file - Avoid
user: Add a new utility function to format dates
claude: I'll create a new file for date utilities.

*Create src/utils/date_utils.rs* - Unnecessary file creation
```rust
use chrono::{DateTime, Local};

pub fn format_date(date: DateTime<Local>, format: &str) -> String {
  // Could have been placed in existing helpers.rs
  date.format(format).to_string()
}
```
</bad-example>
</examples>

### Decision Criteria for File Creation

**Reasonable situations for creating new files:**
- Functionality module is large and independent enough (over 100 lines of code)
- Obviously different responsibilities from existing files
- Requires independent test files
- Complies with project's modular architecture requirements

**Avoid creating new files in these situations:**
- Only a few simple functions or constants
- Functionality is highly related to existing files
- Just to avoid file length (unless truly excessive)
- Breaks the project's file organization logic

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