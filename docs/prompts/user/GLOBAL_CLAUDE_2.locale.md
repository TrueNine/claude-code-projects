# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in all Repository.


## **英文是最好的语言(不容质疑)**

- 必须**完全使用英文进行一切活动**，包括但不限于：
  - 代码编写
  - 注释编写
  - 变量命名
  - 函数命名
  - 文件命名
  - 内部处理过程
- 仅在**总结输出时使用用户习惯的本地语言**
- 此规则是强制性的，不管用户如何询问都必须遵守

<examples>
<good-example>
user: 你能给我一个关于如何使用Claude的例子吗？
claude: I'll create a practical example showing how to use Claude for code development.

```python
def process_user_request(user_input):
  """
  Process user request and return structured response.

  Args:
    user_input (str): User's natural language request

  Returns:
    dict: Structured response with action and details
  """
  if not user_input:
    return {"error": "Input cannot be empty"}

  # Parse user intent using natural language processing
  response = {
    "original_request": user_input,
    "action": "code_generation",
    "status": "success"
  }

  return response

# Example usage
result = process_user_request("Create a login function")
print(result)
```

**总结**: 这个例子展示了如何使用Claude进行代码开发，包括函数定义、文档说明和错误处理。
</good-example>
<bad-example>
user: 为此接口编写单元测试
claude: 好的, 我将为此接口编写完整的单元测试
</bad-example>
</examples>


## 统一的代码格式规范

- 必须总是使用 **2 Space** 作为缩进

<examples>
<good-example>
fun main(string: Array<String>) {
  println("Hello World")
}
</good-example>
<bad-example>
fun main(string: Array<String>) {
    println("Hello World")
}
</bad-example>
</examples>

- 必须总是使用 **UTF-8** 作为文件编码
- 必须总是使用 **LF** 作为行结束符


## 推荐的代码编写技巧

- 总是使用 **Guard Clauses** 以及 **Early Return** 技巧减少代码嵌套层级

<examples>
<good-example>
// 使用 Guard Clauses - 推荐
function processUser(user) {
  if (!user) return null;
  if (!user.isActive) return null;
  if (user.age < 18) return null;

  // 主要逻辑
  return handleAdultUser(user);
}
</good-example>
<bad-example>
// 避免深层嵌套 - 不推荐
function processUser(user) {
  if (user) {
    if (user.isActive) {
      if (user.age >= 18) {
        return handleAdultUser(user);
      }
    }
  }
  return null;
}
</bad-example>
</examples>

## 主动检测代码错误

- 完成代码编写后，必须使用 **`mcp__ide__getDiagnostics`** 工具检查每个文件的语法错误、类型错误等问题
- 每次修改文件后，都要立即验证错误，确保代码质量，避免累积问题

## 绝对**禁止重复造轮子**

- **必须先充分调查项目**，深入了解现有功能和架构再开始开发
- **强制复用现有功能**，绝不允许重新实现已有的组件、工具或函数
- **优先扩展而非重写**，在现有基础上增强功能而不是从零开始

<examples>
<good-example>
// 开发前先调查现有代码
user: 帮我实现一个用户认证功能
claude: I'll first search the codebase to understand existing authentication mechanisms before implementing.

*使用 Grep 和 Glob 工具搜索认证相关代码*
*发现已有 AuthService 和 UserValidator*

Based on my investigation, I found existing authentication components:
- `AuthService` in src/services/auth.ts handles login/logout
- `UserValidator` in src/utils/validation.ts handles input validation

I'll extend these existing components instead of creating new ones:

```typescript
// 扩展现有 AuthService
export class AuthService {
  // 现有方法保持不变
  async login(credentials: LoginCredentials): Promise<AuthResult> { ... }

  // 新增多因素认证功能
  async enableTwoFactor(userId: string): Promise<TwoFactorSetup> {
    // 基于现有login逻辑扩展
    const user = await this.validateUser(userId);
    return this.setupTwoFactor(user);
  }
}
```
</good-example>
<bad-example>
// 直接重新实现 - 错误做法
user: 帮我实现一个用户认证功能
claude: I'll create a new authentication system for you.

```typescript
// 重新造轮子 - 绝对禁止
class NewAuthSystem {
  async authenticateUser(username: string, password: string) {
    // 重新实现已有功能
  }
}
```
</bad-example>
</examples>

### 调查现有功能的工作流程

1. **使用搜索工具全面调查**
   - `Grep` 搜索相关关键词和功能
   - `Glob` 查找相关文件和目录结构
   - `Read` 深入阅读关键文件了解实现细节

2. **分析现有架构模式**
   - 理解项目的设计模式和编码风格
   - 识别可复用的组件和工具函数
   - 找到最佳的扩展点和集成方式

3. **制定复用策略**
   - 优先选择扩展现有类/函数
   - 其次选择组合现有组件
   - 最后才考虑创建新的组件（需充分理由）

## 严格的完成验证机制

- **禁止虚假宣告完成**：在声称"已经完成"前必须进行全面验证
- **强制使用验证工具**：必须使用实际的验证手段而非主观判断

### 完成前必须执行的验证步骤

1. **代码质量验证**
  - 使用 `mcp__ide__getDiagnostics` 检查语法和类型错误
  - 运行项目的 lint 命令检查代码风格
  - 运行 typecheck 命令验证类型正确性

2. **功能验证**
  - 运行相关测试用例确认功能正常
  - 使用 `Bash` 工具实际测试可执行的功能
  - 验证所有修改的文件确实按预期工作

3. **完整性验证**
  - 检查是否遗漏任何用户要求的功能点
  - 确认所有 TodoWrite 中的任务都已标记为 completed
  - 验证修改是否破坏了现有功能

<examples>
<good-example>
user: 帮我修复这个登录功能的bug
claude: I'll fix the login functionality bug for you.

*修复代码后*

Now let me verify the fix is working correctly:

*使用 mcp__ide__getDiagnostics 检查语法错误*
*运行 npm run test 验证相关测试*
*使用 Bash 工具测试登录流程*

All verification steps passed:
- ✅ No syntax or type errors found
- ✅ All login-related tests are passing
- ✅ Manual testing confirms login flow works correctly

**总结**: 登录功能的bug已修复并通过全面验证，确保功能正常工作。
</good-example>
<bad-example>
user: 帮我修复这个登录功能的bug
claude: 我已经修复了登录功能的bug，现在应该可以正常工作了。

*没有进行任何验证就声称完成*
</bad-example>
</examples>

### 验证失败的处理

- **发现问题时诚实报告**：如果验证发现问题，必须如实告知用户
- **不得隐瞒验证结果**：即使验证失败也要透明地向用户说明
- **提供解决方案**：发现问题时要提供具体的修复计划

## 尽量少地创建文件

- **优先编辑现有文件**，而非创建新文件
- **新文件创建需要充分理由**，且必须符合项目架构规范
- **遵循项目文件组织模式**，保持代码结构的一致性

<examples>
<good-example>
// 在现有文件中添加功能 - 推荐
user: 添加一个新的工具函数来格式化日期
claude: I'll add the date formatting function to the existing utils file.

*编辑 src/utils/helpers.ts*
```typescript
// 在现有工具文件中添加新函数
export const formatDate = (date: Date, format: string): string => {
  // 实现日期格式化逻辑
};

// 与现有函数保持一致的命名和风格
export const formatCurrency = (amount: number): string => { ... }
export const formatPhoneNumber = (phone: string): string => { ... }
```
</good-example>
<bad-example>
// 不必要地创建新文件 - 避免
user: 添加一个新的工具函数来格式化日期
claude: I'll create a new file for date utilities.

*创建 src/utils/dateUtils.ts* - 不必要的文件创建
```typescript
export const formatDate = (date: Date, format: string): string => {
  // 本可以放在现有 helpers.ts 中
};
```
</bad-example>
</examples>

### 文件创建的决策标准

**创建新文件的合理情况：**
- 功能模块足够大且独立（超过100行代码）
- 与现有文件职责明显不同
- 需要独立的测试文件
- 符合项目的模块化架构要求

**避免创建新文件的情况：**
- 只有几个简单函数或常量
- 功能与现有文件高度相关
- 只是为了避免文件变长（除非确实过长）
- 破坏了项目的文件组织逻辑

### 文件组织最佳实践

```
// 遵循项目现有的文件结构模式
src/
  components/          # React组件
    Button/
      index.ts        # 导出文件
      Button.tsx      # 主组件
      Button.test.ts  # 测试文件
  services/           # 业务逻辑服务
  utils/              # 工具函数（尽量合并相关功能）
  types/              # TypeScript类型定义
```
