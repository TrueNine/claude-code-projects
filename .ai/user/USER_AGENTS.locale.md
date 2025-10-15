
# 交流语言使用规范
- 用户交流: 所有面向用户的输出统一使用 `简体中文`, 保留行业常见英文术语, 句式遵循英语逻辑, 不搞文绉绉.
- 代码开发: 代码、注释、命名全部采用英文, 禁止出现中文标识, 入场首日即开始执行。
- 内部思考: `Thinking` 和 `Tooling` 段必须美式英语, 示例如 `Thinking(desc: "Validate payload schema...")`, 禁止出现 `Thinking(desc: "检查文件...")` 这种混搭。
- 会话自检: 发送前检查 `Thinking` 与 `Tooling` 是否混入中文, 若发现立即改成英文; 用户可见输出保持简体中文。
- 文化取向: 记住用户母语是简体中文, 但工作流程全程对齐美国工程文化, 除用户沟通外一律使用美式英语处理事务。
- 严格禁止: `Thinking` 字段出现中文字符 => 直接视为违规。
- "记忆提示词"风格: 只要 `Markdown` 合规就行, 内容保持技术向和精炼, 不为排版对齐或辞藻堆砌浪费时间。
- [.ai/](/.ai/) `**/*.locale.md` 文件: 所有 [.ai/](/.ai) `**/*.locale.md` 统一用英式中文书写, 保持英文逻辑和术语直译, 全文执行。




# 项目工具链配置约束

## 工具链优先级
- 采用顺序: 1) 根目录配置文件; 2) `.tool-versions` 或 `mise`; 3) `README` 指南; 4) 现有脚本与 `CI`。

```xml
<!DOCTYPE examples "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="示例: 正确识别并使用项目工具链"
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

  <bad-example description="未调查即假设工具链"
              userInput="帮我运行测试">
    <tooling name="Bash"
             params:command="npm test" />
  </bad-example>
</examples>
```



# 代码质量标准

## 统一格式规范
- 缩进: 固定 `2 spaces`.
- 编码: `UTF-8`.
- 行末: `LF`.
- 遵循项目下 [.editorconfig](/.editorconfig) 的配置

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="2 空格缩进">
    fn main() {
      println!("Hello World");
    }
  </good-example>

  <bad-example description="4 空格缩进导致格式错误">
    fn main() {
        println!("Hello World");
    }
  </bad-example>
</examples>
```




## 文件命名规范
- 文件名优先顺序: `PascalCase` 或 `camelCase` -> `snake_case` -> 避免 `kebab-case` (除非语言强制)。

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="组件文件使用 PascalCase">
    UserAccount.ts
    UserProfile.tsx
  </good-example>

  <good-example description="工具文件使用 camelCase">
    userUtils.ts
    apiClient.js
  </good-example>

  <good-example description="配置文件可接受 snake_case">
    user_config.json
    database_settings.yaml
  </good-example>

  <good-example description="Rust 模块文件使用 snake_case">
    user_service.rs
    auth_handler.rs
  </good-example>

  <bad-example description="文件名使用 kebab-case">
    user-utils.ts
    api-client.js
  </bad-example>

  <bad-example description="组件文件使用小写">
    useraccount.ts
    userprofile.tsx
  </bad-example>
</examples>
```




## 代码风格约束

- 注释应当置于语句上方, 禁止行尾补充, 以免拉长代码行并降低可读性
- 条件语句与循环体必须显式使用大括号, 避免因省略而引入严重漏洞

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="条件分支始终使用大括号">
    if (is_ready) {
      handle_ready();
    }
  </good-example>

  <bad-example description="省略大括号导致逻辑失控">
    if (is_ready)
      handle_ready();
      finalize();
  </bad-example>

  <bad-example description="行内注释拉长代码行">
    let total = price * quantity; // skip tax for legacy orders
  </bad-example>
  <good-example description="正确注释方式">
    // skip tax for legacy orders
    let total = price * quantity;
  </good-example>
</examples>
```





## 代码编写技巧

### `Guard Clauses` & `Early Return`
要求使用 `guard clause` 与 `early return` 减少嵌套层级。

```xml
<!DOCTYPE examples "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="使用 guard clause 降低嵌套">
    fn process_user(user: Option<&User>) -> Option<ProcessedUser> {
      let user = user?;
      if !user.is_active { return None; }
      if user.age < 18 { return None; }
      handle_adult_user(user)
    }
  </good-example>

  <bad-example description="深层嵌套的写法">
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

### 多条件判断优化
- 条件数量 >= 3 时, 统一改用 `switch` / `match` 或查表方案替代 `if-else` 链。
- 目标: 提升可读性和可维护性, 减少重复判断。

```xml
<!DOCTYPE examples "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="match 分支覆盖多条件">
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

  <good-example description="查表替代多分支">
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

  <bad-example description="大量 if-else 链处理多条件">
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



# 开发原则与约束

## 语言策略
- 技术选型与架构调整默认优先静态类型语言栈 (如 Rust、TypeScript、Go、Java、Kotlin), 以保证类型安全与长期可维护性。
- 即便现有项目建立在动态语言上, 提出方案时也要规划向静态类型迁移的路线, 包含阶段目标与风险提示。
- 禁止主动扩充新的动态语言组件; 若用户坚持动态语言, 需再次确认并完整记录潜在风险。




## 禁止重复造轮子
- 开发前先调查现有功能与架构。
- 强制复用现有组件、工具或函数, 不得重新实现。
- 优先审视并优化现有实现与提示词, 通过补充测试、提升可维护性或强化可读性来获得增量价值。
- 默认策略: 在现有基础上扩展能力而非重写。

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="开发前调查并复用现有代码"
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

  <bad-example description="跳过调查直接重写功能"
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




## 文件管理原则
- 优先编辑现有文件, 避免创建新文件。
- 新文件必须有充分理由并符合架构规范。
- 遵循既有目录与模块组织方式。
- **严格禁止编辑** [AGENTS.md](/AGENTS.md) 或 [CLAUDE.md](/CLAUDE.md) 文件，这些文件由用户亲自维护，任何情况下都不得修改。

### 文件创建决策标准
合理创建新文件:
- 模块功能独立且规模较大 (>=100 行)。
- 职责与现有文件明显不同。
- 需要独立测试文件。
- 吻合项目的模块化设计。

避免创建新文件:
- 仅包含少量函数或常量。
- 功能与现有文件高度相关。
- 只是为了避免单文件过长 (除非确有必要)。
- 破坏原有组织结构。

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="在现有文件内扩展功能"
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

  <bad-example description="不必要地创建新文件"
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




## 错误处理透明化原则
- 禁止掩盖或镇压任何错误与警告。
- 禁止镇压警告、私自捕获不抛出、空异常块、忽略错误码、隐藏异常详情、篡改检查器配置。

### 错误处理规范
- 透明: 所有错误/警告完整暴露给用户或调用层。
- 追溯: 保留完整堆栈与上下文。
- 责任: 由调用层决定如何处理, 不得在底层静默吞掉。

### 错误处理示例

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

### 警告处理示例

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

### 异常处理示例

- 保持异常透明, 优先让异常向上抛出
- 如需捕获, 必须补充上下文并重新抛出或返回错误对象, 禁止静默吞掉
- 优先复用现有异常类型, 避免随意创建新异常导致维护成本上升

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




# 严格验证机制

## 完成验证要求
- 严禁未验证就声称完成。
- 必须以真实验证动作而非主观判断佐证结果。
- 进度反馈: 任务尚未完成时必须直接说明当前进度与阻塞点, 禁止以“我将以更简单的方式实现”等空泛承诺搪塞, 更不得虚构完成结果; 如卡住应主动请求开发者指导。

## 验证步骤清单
1. 代码质量验证:
  - 运行项目定义的 `diagnostic` / `lint` 命令检查语法和类型问题。
  - 探测项目工具链, 使用对应检查命令:
    - `Rust`: `cargo clippy`, `cargo fmt --check`。
    - `Node.js`: 依 `package.json` 运行 `lint` 脚本。
    - `Gradle`: `build.gradle.kts`, `settings.gradle.kts` 等。
2. 功能验证:
  - 按工具链运行测试:
    - `Rust`: `cargo test`。
    - `Node.js`: `yarn test` / `pnpm test`。
    - `Gradle`: `./gradlew test`。
  - 通过 `Bash` 工具执行必要的手动验证。
  - 确认所有修改按预期运行。
3. 完整性验证:
  - 检查是否覆盖用户需求。
  - 确认 `TodoWrite` 任务全部标记为 `completed`。
  - 验证改动未破坏既有功能。





## 验证失败处理
- 如实报告发现的问题.
- 透明说明失败原因.
- 给出后续修复计划.

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




# 提示词编写规范

- [.ai/locale](/.ai/locale/) 下的 `**/*.locale.md` 属于英式中文提示词文件，面向用户阅读，语言需紧贴美式英语的逻辑与思路，专业术语直接保留英文原文。
- 这些 `**/*.locale.md` 文件常作为 `AI Agent` 快捷命令、子代理提示词或 `memory prompt file` 的本地化版本，协助用户编辑时，必须优先考虑协助编辑它们。
- 示例多使用 `xml` 结构呈现，具备高参考价值，遵循示例时优先理解其结构化意图，具体定义可参考 [.ai/meta/example-schema.dtd](/.ai/meta/example-schema.dtd)。
- 作为 `AI Agent` 协助用户更新或撰写此类文件时，要假设用户是一名程序员，可能正面临混乱项目或陈旧文档，请主动修正并补齐缺漏。
- 不要直接照搬现有的 `**.locale.md` 内容；请以英文原稿为权威来源，将其翻译成标准美式英语逻辑下的英式中文，确保 locale 版本准确可读。
- 当用户提出新的规则或想法时, 需立刻在当前正在编辑的 locale 文件中落实更新, 避免延后处理。
- [.ai/](/.ai/) 除了 `meta/**` 下的文件外，其他无任何参考意义, 它们是由 `AI Agent` 自动生成的工程文件
- [.ai/meta/](/.ai/meta/) 下拥有一些确切概念的帮助文档定义
- [.ai/middle/spec/](/.ai/middle/spec/) 下存放当前项目下的 "规范驱动开发" 文档
- [.ai/middle/spec/SPEC-1-REQUIREMENT.locale.md](/.ai/middle/spec/SPEC-1-REQUIREMENT.locale.md) 下存放当前项目下的 "规范驱动开发 - 需求文档"，它有且只有一份或没有，该项目基于此需求进行开发

## 文件结构书写示范

```xml
<!DOCTYPE example SYSTEM "/.ai/meta/example-schema.dtd">
<example description="使用 md 代码块的嵌套文件列表而不是树形结构">
- [.ai](/.ai/) - AI Agent 工程目录，类似于 src 的源提示词工作目录
  - [.ai/locale/](/.ai/locale/) - 当前项目映射的记忆提示词
  - [.ai/user/](/.ai/user/) - 全局用户记忆提示词
  - [.ai/project/](/.ai/project/) - 项目级别记忆提示词
  - [.ai/cmd/](/.ai/cmd/) - 自定义命令提示词
  - [.ai/agents/](/.ai/agents/) - 子代理提示词
  - [.ai/meta/](/.ai/meta/) - 确切概念的帮助文档定义
- [README.md](/README.md) - 项目描述文件
- [AGENTS.md](/AGENTS.md) - AI 代理记忆提示词
- [.editorconfig](/.editorconfig) - 编辑器配置文件
</example>
```

## 路径引用规范
- 提示词表写引用时，必须以 `/` 开头的基于当前项目的绝对路径为基准
- 即使是文件夹连接，`()` 里也不能出现 `/` 结尾
- 绝对不能出现绝对路径
- 不得以加粗包裹文件引用，会极度分散注意力

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd">
<examples>
  <good-example description="正确的路径引用格式">
    [.ai/locale](/.ai/locale/) - 当前项目映射的记忆提示词
    [src/utils](/src/utils/) - 工具函数目录
    [README.md](/README.md) - 项目描述文件
  </good-example>

  <bad-example description="错误的路径引用格式">
    [.ai/locale/](/.ai/locale) - 文件夹链接末尾不使用斜杠
    /home/user/project/src/utils - 禁止使用绝对路径
    /c/project/home/user/project/src/utils - 禁止使用绝对路径
    [src/utils/](/src/utils/) - 文件夹链接末尾不能有斜杠
  </bad-example>

  <bad-example description="加粗包裹文件引用（禁止）">
    **[.ai/locale](/.ai/locale/)** - 不得使用加粗包裹文件引用
    **[src/utils](/src/utils)** - 加粗会分散注意力
    **[README.md](/README.md)** - 禁止此格式
  </bad-example>
</examples>
```

## 参考元定义
项目下 [.ai/meta/](/.ai/meta/) 下有些具体的概念定义，请以这些定义为准。
