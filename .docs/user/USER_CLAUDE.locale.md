# CLAUDE 用户配置指南

> **项目级Claude配置文件** - 优先级高于全局配置
> 此文件定义了在此仓库中使用Claude Code的核心原则和规范

---

## 核心原则

### 语言使用规范
- **用户交流**: 100%中文回复
- **代码开发**: 英文编写、注释、命名
- **内部思考**: 100%英文thinking过程
- **严格禁止**: thinking中使用任何中文字符

### 安全操作限制
**绝对禁止**:
- Docker容器操作
- 数据库修改操作
- 系统服务启停
- 批量文件删除

**遇到危险操作时**:
1. 立即警告用户风险
2. 拒绝执行操作
3. 建议专业处理
4. 结束对话

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

### 工具链优先级
**选择顺序** (当存在多种配置时):
1. 项目根目录明确配置文件 (Cargo.toml 优先使用 cargo)
2. .tool-versions 或 mise 配置
3. README 中指定的工具
4. 现有脚本和 CI 配置中使用的工具

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

### 命名规范
**优先级顺序**:
1. **首选**: PascalCase (大驼峰) 或 camelCase (小驼峰)
2. **次选**: snake_case (蛇形)
3. **避免**: kebab-case (烤串) - 除非语言特性强制要求

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

### 代码编写技巧

#### Guard Clauses & Early Return
**强制要求**: 使用 Guard Clauses 和 Early Return 减少嵌套层级

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

#### 多条件判断优化
**强制要求**: 条件数量≥3个时, 使用 Switch语句 或 查表方式替代 if-else 链
**目标**: 提高可读性和维护性, 减少重复判断逻辑

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

```xml

<Examples>
  <GoodExample description="Shows recommended approach of adding functionality to existing files"
               userInput="添加一个新的工具函数来格式化日期">
    我将把日期格式化函数添加到现有的工具文件中。
    <Tooling name="Write"
             params:path="src/utils/helpers.rs" />
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
    <Tooling name="Write"
             params:path="src/utils/date_utils.rs"
             description="不必要的文件创建"/>
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

### 错误处理透明化原则
- **强制要求**: 禁止一切错误掩盖行为，确保问题完全暴露
- **绝对禁止的行为**:
  - 镇压警告信息
  - 本地捕获错误而不上报
  - 使用空的异常处理块
  - 忽略函数返回的错误码
  - 隐藏或简化异常信息
  - 镇压检查器警告
  - 修改任何检查器配置文件

#### 错误处理规范
- **透明原则**: 所有错误、警告必须完整暴露给用户或调用者
- **追溯原则**: 保留完整的错误堆栈和上下文信息
- **责任原则**: 错误处理责任应由调用层决定，而非被调用层隐藏

#### 错误处理示例

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

#### 警告处理示例

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

#### 异常处理示例

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
