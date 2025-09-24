# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in all Repository.

---

## 基础原则与安全限制

### 语言使用规范
- **必须完全使用英文进行一切技术活动**, 包括:
  - 代码编写、注释编写
  - 变量命名、函数命名、文件命名
  - 内部处理过程
  - **Thinking过程(内部思考)** - 绝对强制使用英文
- **使用用户本地语言的场景**:
  - 总结输出时
  - 向用户提问或抛出问题时
- **此规则强制执行**, 不管用户如何询问都必须遵守

#### Thinking过程语言强制规范
- **所有thinking/思考过程必须100%使用英文**
- **禁止在thinking中出现任何中文字符**, 包括:
  - 中文汉字、标点符号
  - 中文逗号(，)、句号(。)、冒号(：)
  - 任何中文语言表达
- **违规示例**: `现在我看到还有一些中文标点符号需要处理。让我继续处理这些标点符号。`
- **正确示例**: `Now I can see there are some Chinese punctuation marks that need to be processed. Let me continue processing these punctuation marks.`

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

**总结**: 这个例子展示了如何使用Claude进行代码开发, 包括函数定义, 文档说明和错误处理.
</GoodExample>
<BadExample>
user: 为此接口编写单元测试
claude:
*Thinking: 用户要求为接口编写单元测试，我需要开始编写测试代码。*

好的, 我将为此接口编写完整的单元测试
</BadExample>
</Examples>

### 安全操作限制
- **绝对禁止操作Docker容器或数据库**, 除非用户明确要求且任务确实需要
- **遇到危险操作时必须**:
  1. 立即提醒用户操作的潜在风险
  2. 明确告知不会执行相关操作
  3. 建议用户自行操作或寻求专业帮助
  4. 结束当前对话以避免意外操作

#### 操作限制清单
**绝对禁止的操作:**
- Docker容器的启动、停止、删除、修改
- 数据库的增删改查操作(除非只是查看配置)
- 系统服务的启停操作
- 网络配置的修改
- 用户权限的变更
- 批量文件删除操作

**需要特别谨慎的操作:**
- 任何涉及`sudo`的命令
- 系统配置文件的修改
- 环境变量的永久性修改
- 包管理器的全局安装操作

<Examples>
<GoodExample>
user: 帮我清理一下数据库中的无用数据
claude: 我注意到您要求进行数据库操作. 为了您的数据安全, 我不会执行任何数据库修改操作, 因为这可能导致数据丢失或损坏.

建议您:
1. 先备份重要数据
2. 在测试环境中验证操作
3. 由专业数据库管理员执行此类操作

我无法继续执行此任务, 请谅解.
</GoodExample>
<BadExample>
user: 清理数据库无用数据
claude: 好的, 我来帮您清理数据库中的无用数据...
*执行危险的数据库操作*
</BadExample>
</Examples>

---

## 项目工具链配置约束

### 严格遵守项目工具链
- **强制检测并使用项目现有工具链**, 绝不能假设或随意选择
- **必须调查项目配置文件**来确定正确的工具链:
  - `Cargo.toml` - Rust项目使用cargo
  - `package.json` - Node.js项目检查packageManager字段
  - `requirements.txt/pyproject.toml` - Python项目
  - `composer.json` - PHP项目
  - `Gemfile` - Ruby项目

### 工具链使用优先级
**当存在多种配置时的选择顺序:**
1. **项目根目录的明确配置文件** (如Cargo.toml存在则优先使用cargo)
2. **项目中.tool-versions或mise配置**
3. **项目README中明确指定的工具**
4. **项目现有脚本和CI配置中使用的工具**

<Examples>
<GoodExample>
// 检测到项目有Cargo.toml - 使用cargo工具链
user: 帮我运行测试
claude: I'll check the project configuration first.

*使用Glob工具查找Cargo.toml*
*发现项目根目录存在Cargo.toml*

Based on the Cargo.toml file, this is a Rust project. I'll use cargo for testing:

```bash
cargo test
```
</GoodExample>
<BadExample>
// 未检测项目配置就假设使用npm - 错误
user: 帮我运行测试
claude: I'll run the tests using npm:

```bash
npm test  # 错误 - 未检测项目实际使用的工具链
```
</BadExample>
</Examples>

### 命令生成规范
- **构建命令**: 根据项目类型使用 `cargo build` / `npm run build` / `pip install` 等
- **测试命令**: 根据项目类型使用 `cargo test` / `npm test` / `pytest` 等
- **格式化命令**: 根据项目类型使用 `cargo fmt` / `prettier` / `black` 等
- **检查命令**: 根据项目类型使用 `cargo clippy` / `eslint` / `flake8` 等

---

## 代码质量标准

### 统一格式规范
- **缩进**: 必须使用 **2 Space** 作为缩进
- **编码**: 必须使用 **UTF-8** 文件编码
- **行结束符**: 必须使用 **LF** 行结束符

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

### 命名规范
**优先级顺序**:
1. **首选**: 大驼峰命名法 (PascalCase) 或小驼峰命名法 (camelCase)
2. **次选**: 蛇形命名法 (snake_case)
3. **避免**: 烤串命名法 (kebab-case) - 除非语言特性或框架强制要求

<Examples>
<GoodExample>
// 推荐的命名方式
struct UserAccount;           // 大驼峰 - 类型名
let userName = "john";        // 小驼峰 - 变量名
let user_count = 42;          // 蛇形 - 可接受的变量名
mod user_service;             // 蛇形 - Rust模块名约定
</GoodExample>
<BadExample>
// 避免的命名方式
let user-name = "john";       // 烤串命名法 - 除非必要否则避免
struct user-account;          // 烤串命名法 - 不符合大多数语言规范
</BadExample>
</Examples>

### 代码编写技巧
#### Guard Clauses & Early Return
- **必须使用** Guard Clauses 和 Early Return 技巧减少代码嵌套层级

<Examples>
<GoodExample>
// 使用 Guard Clauses - 推荐
fn process_user(user: Option<&User>) -> Option<ProcessedUser> {
  let user = user?;
  if !user.is_active { return None; }
  if user.age < 18 { return None; }

  // 主要逻辑
  handle_adult_user(user)
}
</GoodExample>
<BadExample>
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

#### 多条件判断优化
- **条件数量≥3个时强制执行**: 使用 Switch语句 或 查表方式替代多个if-else
- 提高代码可读性和维护性, 减少重复的条件判断逻辑

<Examples>
<GoodExample>
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
<BadExample>
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

### 代码错误检测
- **完成代码编写后必须**: 使用 `mcp__ide__getDiagnostics` 工具检查语法错误, 类型错误
- **每次修改文件后**: 立即验证错误, 确保代码质量, 避免累积问题

---

## 开发原则与约束

### 禁止重复造轮子
- **开发前必须调查**: 充分了解现有功能和架构
- **强制复用现有功能**: 绝不允许重新实现已有组件, 工具或函数
- **优先扩展而非重写**: 在现有基础上增强功能

#### 调查工作流程
1. **全面搜索调查**
  - `Grep` 搜索相关关键词和功能
  - `Glob` 查找相关文件和目录结构
  - `Read` 深入阅读关键文件了解实现细节

2. **分析现有架构**
  - 理解项目的设计模式和编码风格
  - 识别可复用的组件和工具函数
  - 找到最佳的扩展点和集成方式

3. **制定复用策略**
  - 优先: 扩展现有类/函数
  - 次选: 组合现有组件
  - 最后: 创建新组件(需充分理由)

<Examples>
<GoodExample>
// 开发前先调查现有代码
user: 帮我实现一个用户认证功能
claude: I'll first search the codebase to understand existing authentication mechanisms before implementing.

*使用 Grep 和 Glob 工具搜索认证相关代码*
*发现已有 AuthService 和 UserValidator*

Based on my investigation, I found existing authentication components:
- `AuthService` in src/services/auth.ts handles login/logout
- `UserValidator` in src/utils/validation.ts handles input validation

I'll extend these existing components instead of creating new ones:

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
<BadExample>
// 直接重新实现 - 错误做法
user: 帮我实现一个用户认证功能
claude: I'll create a new authentication system for you.

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

### 文件管理原则
- **优先编辑现有文件**, 避免创建新文件
- **新文件需充分理由**, 且必须符合项目架构规范
- **遵循项目组织模式**, 保持代码结构一致性

#### 文件创建决策标准
**合理创建新文件:**
- 功能模块足够大且独立(超过100行代码)
- 与现有文件职责明显不同
- 需要独立的测试文件
- 符合项目的模块化架构要求

**避免创建新文件:**
- 只有几个简单函数或常量
- 功能与现有文件高度相关
- 仅为避免文件变长(除非确实过长)
- 破坏项目的文件组织逻辑

<Examples>
<GoodExample>
// 在现有文件中添加功能 - 推荐
user: 添加一个新的工具函数来格式化日期
claude: I'll add the date formatting function to the existing utils file.

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
<BadExample>
// 不必要地创建新文件 - 避免
user: 添加一个新的工具函数来格式化日期
claude: I'll create a new file for date utilities.

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

---

## 严格验证机制

### 完成验证要求
- **禁止虚假完成声明**: 声称"已完成"前必须进行全面验证
- **强制使用验证工具**: 使用实际验证手段而非主观判断

### 验证步骤清单
1. **代码质量验证**
  - 使用 `mcp__ide__getDiagnostics` 检查语法和类型错误
  - **首先检测项目工具链**, 然后运行对应的检查命令:
    - Rust项目: `cargo clippy` (代码检查) + `cargo fmt --check` (格式检查)
    - Node.js项目: 根据package.json运行相应的 lint 命令
    - Python项目: `flake8` / `black --check` / `mypy` 等

2. **功能验证**
  - **根据项目工具链运行测试**:
    - Rust项目: `cargo test`
    - Node.js项目: `npm test` / `yarn test` / `pnpm test`
    - Python项目: `pytest` / `python -m unittest`
  - 使用 `Bash` 工具实际测试可执行功能
  - 验证所有修改的文件按预期工作

3. **完整性验证**
  - 检查是否遗漏任何用户要求的功能点
  - 确认所有 TodoWrite 任务都已标记为 completed
  - 验证修改未破坏现有功能

### 验证失败处理
- **诚实报告问题**: 发现问题必须如实告知用户
- **透明说明结果**: 即使验证失败也要透明向用户说明
- **提供解决方案**: 发现问题时提供具体修复计划

<Examples>
<GoodExample>
user: 帮我修复这个登录功能的bug
claude: I'll fix the login functionality bug for you.

*修复代码后*

Now let me verify the fix is working correctly:

*使用 mcp__ide__getDiagnostics 检查语法错误*
*检查项目工具链并运行 cargo test 验证相关测试*
*使用 Bash 工具测试登录流程*

All verification steps passed:
- ✅ No syntax or type errors found
- ✅ All Rust tests are passing (cargo test)
- ✅ Manual testing confirms login flow works correctly

**总结**: 登录功能的bug已修复并通过全面验证, 确保功能正常工作.
</GoodExample>
<BadExample>
user: 帮我修复这个登录功能的bug
claude: 我已经修复了登录功能的bug, 现在应该可以正常工作了.

*没有进行任何验证就声称完成*
</BadExample>
</Examples>

### 文件组织最佳实践
```
// 遵循项目现有的文件结构模式
src/
  components/          # UI组件模块
    button/
      mod.rs          # 导出文件
      button.rs       # 主组件
      tests.rs        # 测试文件
  services/           # 业务逻辑服务
  utils/              # 工具函数(尽量合并相关功能)
  types/              # 类型定义
  lib.rs              # 库入口文件
```
