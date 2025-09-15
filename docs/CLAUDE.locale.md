# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this Repository.

---

## 提示词编写规范

文档结构必须由 `Tree Structure` 表示法来编写. 

<example>
  ```text
    ├── architecture/ # 系统架构设计文档
    │ ├── api-design.md # RESTful API 设计规范
    │ └── database-schema.md # 数据库模式设计
  ```
</example>

## 翻译规范

- 所有的 `*.locale.md` 文件都需要翻译为同名去除了 `.locale` 的文件
- 翻译目标统一使用美式英文

<example>
  `CLAUDE.locale.md` → `CLAUDE.md`
  `TEST.locale.md` -> `TEST.md`
</example>

## 代码库文档架构示例

**AI助手工作时的文档结构理解:**

```

docs/
├── architecture/ # 系统架构设计文档
│ ├── api-design.md # RESTful API 设计规范
│ └── database-schema.md # 数据库模式设计
├── development/ # 开发流程与规范
│ ├── coding-standards.md # 代码编写标准
│ ├── git-workflow.md # Git 工作流程
│ └── testing-guide.md # 测试指南
├── deployment/ # 部署与运维
│ ├── docker-setup.md # 容器化部署
│ └── ci-cd-pipeline.md # 持续集成流程
└── troubleshooting/ # 问题排查指南
├── common-errors.md # 常见错误解决方案
└── performance-tuning.md # 性能优化指南

```

**AI助手应当理解的文档分类逻辑:**

- 架构文档：系统设计决策和技术选型依据
- 开发文档：编码实践和团队协作规范
- 部署文档：环境配置和发布流程
- 故障文档：问题诊断和解决方案库
