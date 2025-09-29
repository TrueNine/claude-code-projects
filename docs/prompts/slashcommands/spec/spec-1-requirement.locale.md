---
argument-hint: no_parameters
allowed-tools: Read, Write, Edit, MultiEdit, Glob, Grep, Bash, TodoWrite, Task
description: 规范驱动开发步骤1, 按照严格文档格式，协助用户完成序文文档编写
---

协助用户渐进式完成规范驱动开发（Specification-Driven Development）流程第一步，需求文档编写。

# [STEP 1]: 准备工作
# [STEP 2]: 耐心倾听用户首次描述需求
# [LOOP STEP 3]: 按照严格的规范格式整理文档
# [LOOP STEP 4]: 耐心倾听用户需求并分析文档

# [附录 1]: 需求文档的既定格式

在输出文档时，应当遵循以下标准来书写标准的 `markdown` 格式
```md
# [PROJECT_NAME] User Requirements Documentation
```
> `$PROJECT_NAME` 为当前项目的id

<Examples>
<Example description="用户文档示例1">
## Introduction

# mediacms User Requirements Documentation
</Example>
<Example description="用户文档示例2">
## Introduction

# carshoping User Requirements Documentation
</Example>
<Example description="用户文档示例2">
## Introduction

# idea-mcp-plugin User Requirements Documentation
</Example>
</Examples>

空行一下后，紧跟的就是一个最简短的描述

<Examples>
<Example description="描述1">
此文档记录了用户在开发 MES 系统详细的开发需求，...
</Example>
<Example description="描述2">
此文档记录了用户在开发 电商前后端分离项目的详细的开发需求，...
</Example>
</Examples>

然后就是记录主体用户，空一行之后格式如下：

以 `**Primary Persona:** ` 开头，加上简短的描述性，与领域高度契合且可读性高的用户

<Examples>
<GoodExample>
**Primary Persona:** 制造业员工、制造业开发者
</GoodExample>
<GoodExample>
**Primary Persona:** 在校大学生、高校老师、社团建模爱好者
</GoodExample>
<BadExample description="使用了中文描述">
**主要客户群体:** 在校大学生、高校老师、社团建模爱好者
</BadExample>
<BadExample description="艺术化表达">
**Primary Persona:** 富有魅力的企业高管、...
</BadExample>
</Examples>

可选的规格需求约束，格式以 `**Operational Constraints:**`起行随后附加的有序列表

<Example>
**Operational Constraints:**
1. 服务器性能有限, 需要轻量化部署并控制带宽占用.
2. 默认依赖外部 MySQL 8; 视频资源可部署在本地磁盘或 TOS, 视成本取舍.
3. 访问与播放量较低, 但需确保圈内访问流畅与后台易维护.

</Example>


可选的非功能性优先级，格式以 `**Non-Functional Priorities:** `起行随后附加的有序列表

<Example>
**Non-Functional Priorities:**
1. 默认启用 HTTPS, 优先使用云厂商免费证书.
2. 视频与封面优先经由 TOS/CDN; 若采用本地存储, 需提供容量监控与清理策略.
3. 当前仅需桌面端体验, 移动端可在后续需求出现时迭代.
4. 提供容器或脚本化部署以便迁移与快速恢复.
5. 实现轻量日志与监控, 并规划数据库与关键数据的定期备份.

</Example>

可选的后续可能需要的功能

<Example>
**Deferred Scope:**
1. 人才市场招聘能力.
2. 短剧贩售与付费解锁模块.

</Example>


随后跟随需求列表

<Example>
## Requirements

### Requirement 1: AI 短剧作品展示

**User Story:** As 创作者, 我希望有一个主页集中展示我最新和代表性的 AI 短剧作品, 以便朋友们能快速浏览和播放.

#### Acceptance Criteria

1. WHEN 访客打开首页 THEN 系统应按发布时间或推荐顺序展示视频卡片, 包含封面,标题,简介和标签信息.
2. WHEN 访客点击视频卡片 THEN 系统应打开支持本地存储或 TOS 播放的作品详情页.
3. WHEN 访客查找特定内容 THEN 系统应提供搜索与标签或类型筛选, 帮助其快速定位作品.

</Example>
