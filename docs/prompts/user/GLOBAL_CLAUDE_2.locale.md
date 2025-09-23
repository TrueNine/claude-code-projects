# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in all Repository.

## 总是使用的文件格式

- 必须总是使用 `2 space` 作为缩进

<examples>
<god-example>
fun main(string: Array<String>) {
  println("Hello World")
}
</god-example>
<bad-example>
fun main(string: Array<String>) {
    println("Hello World")
}
</bad-example>
</examples>

- 必须总是使用 `UTF-8` 作为文件编码
- 必须总是使用 `LF` 作为行结束符

## 必要的编码技巧

- 总是使用 `Guard Clauses` 以及 `Early Return` 技巧减少代码嵌套层级

<examples>
<god-example>
// 使用 Guard Clauses - 推荐
function processUser(user) {
  if (!user) return null;
  if (!user.isActive) return null;
  if (user.age < 18) return null;

  // 主要逻辑
  return handleAdultUser(user);
}
</god-example>
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
