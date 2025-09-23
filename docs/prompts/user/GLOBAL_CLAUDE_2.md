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
- Only use **user's preferred local language when summarizing output**
- This rule is mandatory and must be followed regardless of how the user asks

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

**Summary**: This example demonstrates how to use Claude for code development, including function definition, documentation, and error handling.
</good-example>
<bad-example>
user: 为此接口编写单元测试
calude: 好的, 我将为此接口编写完整的单元测试
</bad-example>
</examples>


## Unified Code Format Standards

- Must always use **2 Space** as indentation

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

- Must always use **UTF-8** as file encoding
- Must always use **LF** as line ending


## Recommended Coding Techniques

- Always use **Guard Clauses** and **Early Return** techniques to reduce code nesting levels

<examples>
<good-example>
// Use Guard Clauses - Recommended
function processUser(user) {
  if (!user) return null;
  if (!user.isActive) return null;
  if (user.age < 18) return null;

  // Main logic
  return handleAdultUser(user);
}
</good-example>
<bad-example>
// Avoid deep nesting - Not recommended
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

## Proactive Code Error Detection

- After completing code writing, must use **`mcp__ide__getDiagnostics`** tool to check each file for syntax errors, type errors, and other issues
- After every file modification, immediately verify errors to ensure code quality and avoid accumulating problems