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
claude: 
</good-example>
<bad-example>
user: 为此接口编写单元测试
calude: 好的, 我将为此接口编写完整的单元测试
</bad-example>
</examples>


## 总是使用的文件格式

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


## 必要的编码技巧

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
