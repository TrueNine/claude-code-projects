---
argument-hint: [your locale language example <en_US> | <zh_CN> ] [ <your project name> ]
allowed-tools: Read, Write, Edit, MultiEdit, Glob, Grep, Bash, TodoWrite, Task
description: Progressively guide users through the first step of specification-driven development in a specified language, generating structured requirements documentation
---

This document guides users through the first step of specification-driven development, collecting and organizing project requirements through progressive dialogue. The document provides complete process guidance and standardized format templates to help users transform vague ideas into structured requirements documentation. The entire process is divided into two main phases: preparation and requirements collection, ensuring the final requirements documentation is actionable and testable.

# [STEP-1]: Preparation

- Use `Search(pattern: ".ai/middle/spec/SPEC-1-REQUIREMENT.locale.md")` to check if `.ai/middle/spec/SPEC-1-REQUIREMENTS.locale.md` exists
- If the file exists, use `Read(path: ".ai/middle/spec/SPEC-1-REQUIREMENT.locale.md")` to load content and provide a brief overview; otherwise use `Write(content: "")` to initialize an empty file
- Identify the user's preferred language represented by the first parameter `$1`, subsequent free text output defaults to this language; if the user specifies otherwise, follow the latest instructions
- After completing the overview, invite the user to stay relaxed, use their imagination, and describe the ideal system completion form, then wait for user input to proceed to [STEP-2]

# [STEP-2]: Patiently listen to user's initial requirements description

- Recognize that the user is in a relaxed, divergent thinking brainstorming state, expressions may be jumpy or metaphorical
- Guide progressively from an architect's perspective: use simple language, break the system into modules, scenarios, or processes, and explore key information section by section
- Listen carefully and record key information, clarify ambiguities with polite questions when necessary
- Respond only when clarification or confirmation is needed, remain quiet and listen at other times, avoid frequent interruptions
- If the user says "stop" or makes an equivalent request, immediately end the dialogue
- The user's divergent descriptions need to be refined first, select content that aligns with [APPENDIX-1] structure and has actionable value before using _Write_ to record; avoid sentence-by-sentence copying
- If no actionable points are currently identified, remain silent and continue listening until clear information is obtained
- Continuously iterate at the user's pace until the user explicitly indicates "completion"
- After user confirms completion, remind them to use `/translate` or other tools to translate the document into English, and record the translation link in `project/CLAUDE.md` to improve subsequent coding experience

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd" >
<examples>
  <good-example description="Actively encourage users to continue diverging" userInput="I want to create a system to help remote teams collaborate, but for now I just want to make communication smoother.">
    <tooling name="Update"
             params:path=".ai/middle/spec/SPEC-1-REQUIREMENT.locale.md"
             description="Make remote team communication smoother" />
    Great starting point! I've recorded that. You can imagine what this system would look like when it's working perfectly, for example, how would team members collaborate throughout a day? Feel free to share any fragments that come to mind, and we'll organize them into structured points together.
  </good-example>

  <bad-example description="Directly否定 user ideas, lacking encouragement" userInput="I want to create a system to help remote teams collaborate, but for now I just want to make communication smoother.">
    Your description is too vague, I can't write requirements like this. Come back when you've thought it through clearly.
  </bad-example>

  <good-example description="Maintain friendly guidance when facing particularly vague inspirations" userInput="I want to do something cool, maybe something to help friends play games together? I haven't figured it out yet.">
    <tooling name="Update"
             params:path=".ai/middle/SPEC-1-REQUIREMENT.locale.md"
             description="Help friends play games together more smoothly" />
    That sounds interesting! I've noted down this goal for now. No need to rush into specific features, you can start with small annoyances you encounter when playing games, like voice communication, team formation processes, or activity scheduling. Share a few real scenarios, and I'll help you organize which ones are worth recording.
  </good-example>

  <good-example description="Continue diverging when clues are too abstract" userInput="Hmm... I want to do something AI-related, but I haven't decided what specifically yet.">
    This direction has great potential! The current information is still quite abstract, so I won't write it to the document yet. We can first talk about the pain points you want to solve, who will use it, or the ideal scenario you envision. The more specific you share, the easier it will be for me to help you extract actionable points to record.
  </good-example>

  <bad-example description="Press for details when user ideas are not yet formed" userInput="I want to do something cool, maybe something to help friends play games together? I haven't figured it out yet.">
    You must determine specific features first, otherwise we can't continue. Come back when you've thought it through.
  </bad-example>
</examples>
```


## Locale Usage Conventions

- `$1` is the locale passed in by the slash command (such as `zh_CN`, `en_US`), also representing the user's preferred language
- When communicating with users, default to using the language corresponding to `$1`; if the user switches languages or specifies special requirements, follow the latest instructions
- When generating specification documents, except for fixed English titles or keywords, all other free text should use `$1` language
- Follow common expressions and punctuation of `$1` language, making the copy read naturally without translation flavor
- When clarifying terminology or demonstrating examples, you can first explain in `$1` language, and supplement with English comparison when necessary


# [APPENDIX-1]: Established Format for Requirements Documentation

When outputting requirements documentation, you must strictly follow the following standard Markdown format specifications:

```md
# [PROJECT_NAME:- $2] User Requirements Documentation
```

**Format Description:**
- `[PROJECT_NAME:- $2]`: Placeholder, needs to be replaced with actual project identifier (such as `mediacms`, `carshoping`, etc.)
- Document title must be in English, following PascalCase naming conventions
- Document type is fixed as "User Requirements Documentation"

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd" >
<examples>
  <example># mediacms User Requirements Documentation</example>
  <example># carshoping User Requirements Documentation</example>
  <example># idea-mcp-plugin User Requirements Documentation</example>
</examples>
```

After a blank line, add the project introduction section in the following format:

```md
## Introduction

This document records the detailed development requirements for developers developing [project type] projects, ...
```

**Writing Guidelines:**
- Use secondary heading `## Introduction`
- Description should start with a sentence in `$1` language equivalent to "This document records the detailed development requirements for developers developing"
- Briefly explain the project type and main goals
- Length控制在 2-5 sentences

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd" >
<examples>
  <example description="MES system project example">
    ## Introduction

    This document records the detailed development requirements for developers developing MES systems, aiming to achieve digital management and monitoring of production processes.
  </example>

  <example description="E-commerce project example">
    ## Introduction

    This document records the detailed development requirements for developers developing e-commerce front-end and back-end separated projects, covering core functions such as product management, order processing, and user systems.
  </example>
</examples>
```

After a blank line, define the target user groups in the following format:

```md
**Primary Persona:** [User group description]
```

**Writing Specifications:**
- Fixed use of English title `**Primary Persona:**`
- Use `$1` language to describe user groups, and list multiple groups according to common separators of that language (such as Chinese enumeration commas, English commas)
- Descriptions should be concise, accurate, and highly relevant to the project domain
- Avoid subjective evaluations or artistic expressions

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd" >
<examples>
  <good-example description="Manufacturing project">
    **Primary Persona:** Manufacturing employees, Manufacturing developers
  </good-example>

  <good-example description="Education project">
    **Primary Persona:** University students, University teachers, Student association modeling enthusiasts
  </good-example>

  <bad-example description="Error: Using Chinese title">
    **主要客户群体:** University students, University teachers, Student association modeling enthusiasts
  </bad-example>

  <bad-example description="Error: Including subjective evaluations">
    **Primary Persona:** Charming corporate executives, Technology experts pursuing excellence
  </bad-example>

  <bad-example description="Error: Description too vague">
    **Primary Persona:** Various users, People with needs
  </bad-example>
</examples>
```

After a blank line, add optional project constraint conditions in the following format:

```md
**Operational Constraints:**
1. [Specific constraint description]
2. [Specific constraint description]
3. [Specific constraint description]
```

Constraint type references (can be flexibly adjusted according to actual situation):
- Infrastructure: hardware configuration, network environment, deployment methods, etc.
- Technology stack: programming languages, framework choices, third-party services, etc.
- Team configuration: team size, skill structure, external collaboration, etc.
- Compliance requirements: industry standards, data security, privacy protection, etc.
- Operational support: availability goals, maintenance costs, scalability, etc.
- Business factors: budget limitations, time requirements, return on investment, etc.

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd" >
<examples>
  <good-example description="Video project constraints">
    **Operational Constraints:**
    1. Limited server performance, requires lightweight deployment and control bandwidth usage
    2. Default dependency on external MySQL 8; video resources can be deployed on local disk or TOS, depending on cost trade-offs
    3. Low access and playback volume, but need to ensure smooth access within the circle and easy backend maintenance
  </good-example>

  <good-example description="Financial project constraints">
    **Operational Constraints:**
    1. Must comply with national financial data security regulations, all transaction data needs encrypted storage
    2. System availability requirement 99.9%, annual downtime not exceeding 8.76 hours
    3. Development team of 3 people, including 1 frontend, 1 backend, 1 tester
    4. Budget limited to under 500,000, including one year of operational costs
  </good-example>

  <bad-example description="Description too vague">
    **Operational Constraints:**
    1. Server should be a bit better
    2. Need to complete quickly
    3. Budget is not enough
  </bad-example>

  <bad-example description="Using unprofessional expressions">
    **Operational Constraints:**
    1. Computer configuration can't be too bad, otherwise it won't run
    2. Better to use cloud services, more convenient
    3. Find a few people to do it casually
  </bad-example>
</examples>
```

After a blank line, add optional non-functional priority descriptions in the following format:

```md
**Non-Functional Priorities:**
1. [Priority description]
2. [Priority description]
3. [Priority description]
```

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd" >
<examples>
  <good-example description="Clear non-functional priorities">
    **Non-Functional Priorities:**
    1. Enable HTTPS by default, prioritize using cloud provider free certificates
    2. Videos and covers prioritize TOS/CDN; if using local storage, need to provide capacity monitoring and cleanup strategies
    3. Currently only need desktop experience, mobile can be iterated when future needs arise
    4. Provide containerized or scripted deployment for migration and quick recovery
    5. Implement lightweight logging and monitoring, and plan regular backups of databases and key data
  </good-example>

  <bad-example description="Vague non-functional priorities">
    **Non-Functional Priorities:**
    1. System needs to be secure and stable
    2. Speed should be a bit faster
    3. Interface should look good
    4. Later maintenance should be convenient
    5. Deployment should be simple
  </bad-example>

  <good-example description="Quantifiable non-functional priorities">
    **Non-Functional Priorities:**
    1. All sensitive data must be AES-256 encrypted for storage, transmission uses TLS 1.3
    2. Core transaction API response time ≤ 500ms, 99% of requests need to complete within 200ms
    3. System availability ≥ 99.9%, monthly downtime ≤ 43.2 minutes
    4. Support latest two versions of Chrome/Firefox/Safari, IE11 minimum compatibility
    5. Code coverage ≥ 80%, key business 100% have integration tests
  </good-example>

  <bad-example description="Technology selection rather than priorities">
    **Non-Functional Priorities:**
    1. Use React framework for frontend development
    2. Backend adopts Spring Boot framework
    3. Database uses MySQL 8.0
    4. Cache uses Redis
    5. Message queue uses RabbitMQ
  </bad-example>
</examples>
```

After a blank line, add optional deferred scope descriptions in the following format:

```md
**Deferred Scope:**
1. [Feature description]
2. [Feature description]
3. [Feature description]
```

**Writing Guidelines:**
- Use English title `**Deferred Scope:**`
- List features not considered in current version but may need implementation in the future
- Each feature should be concise and highlight core value
- Avoid duplication with existing requirements
- Ordered list content should be written in `$1` language

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd" >
<examples>
  <good-example description="Video platform deferred features">
    **Deferred Scope:**
    1. Talent marketplace recruitment capabilities, connecting creators with enterprises
    2. Short drama sales and paid unlock modules, supporting content monetization
    3. Creator community features, supporting work exchange and collaboration
  </good-example>

  <good-example description="E-commerce platform deferred features">
    **Deferred Scope:**
    1. Social sharing features, allowing users to share products to various platforms
    2. Membership points system, enhancing user loyalty
    3. Multi-language internationalization support, expanding overseas markets
  </good-example>

  <bad-example description="Description too vague">
    **Deferred Scope:**
    1. Some other features
    2. Things to add later
    3. Things to do when there's money
  </bad-example>

  <bad-example description="Duplicate with current requirements">
    **Deferred Scope:**
    1. User login registration (already in basic features)
    2. Product display pages (already in core requirements)
    3. Order management functionality (already in must-implement)
  </bad-example>
</examples>
```


Then follows the core requirements list, which is the most important part of the entire document and must strictly follow the following specifications:

## Requirements Format Specifications

### Basic Structure
```md
## Requirements

### Requirement [Number]: [Requirement Name]

**User Story:** As [User Role], I want [Desired Functionality], so that [Value Gained].

#### Acceptance Criteria

1. WHEN [Trigger Condition] THEN [Expected Result]
2. WHEN [Trigger Condition] THEN [Expected Result]
3. WHEN [Trigger Condition] THEN [Expected Result]
```

### Writing Specification Requirements

1. **User Story**
- Must use standard format: `As [Role], I want [Functionality], so that [Value]`
- Role should be specific (e.g., "Creator" rather than "User")
- Value should be clear (answer "why this functionality is needed")
- Use `$1` language to write [Role], [Functionality], [Value]

2. **Acceptance Criteria**
- Must use Given-When-Then format
- Each criterion must be independent and testable
- Avoid technical implementation details, focus on business behavior
- Use `$1` language to write [Trigger Condition], [Expected Result]

3. **Requirement Decomposition Principles**
- Each requirement should be independent and have clear value
- Avoid being too large (consider splitting if more than 5 acceptance criteria)
- Avoid being too small (consider merging if fewer than 2 acceptance criteria)

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd" >
<examples>
  <good-example description="Complete user requirement">
    ### Requirement 3: User Work Management

    **User Story:** As Creator, I want to be able to manage all my works, so that I can edit or delete content at any time.

    #### Acceptance Criteria

    1. WHEN Creator logs in and enters personal center THEN the system should display a list of all their works, including thumbnails, titles, publication time, and view counts
    2. WHEN Creator clicks the work edit button THEN the system should jump to the edit page, preserving original content and allowing modification of all information
    3. WHEN Creator deletes a work THEN the system should require secondary confirmation, remove it from the list after success, and notify the user
    4. WHEN a work is favorited or commented by other users THEN the creator should be able to see relevant statistical data on the management page
  </good-example>

  <bad-example description="Missing user value">
    ### Requirement 2: User Login

    **User Story:** As User, I want to log in to the system.

    #### Acceptance Criteria

    1. Enter username and password
    2. Click login button
    3. Login successful
  </bad-example>

  <good-example description="Technology-agnostic acceptance criteria">
    ### Requirement 5: Content Recommendation

    **User Story:** As Viewer, I want the system to recommend short drama content I'm interested in, so that I can discover more quality works.

    #### Acceptance Criteria

    1. WHEN Viewer browses the homepage THEN the system should recommend similar type works based on their viewing history
    2. WHEN Viewer completes watching a work THEN the system should recommend other works by related creators
    3. WHEN Viewer continuously skips multiple recommendations THEN the system should adjust the recommendation algorithm to provide more precise content
  </good-example>

  <bad-example description="Including technical implementation">
    ### Requirement 4: Video Upload

    **User Story:** As Creator, I want to upload videos.

    #### Acceptance Criteria

    1. Call backend API interface /api/v1/videos
    2. Use MySQL to store video information
    3. Video files stored in OSS object storage
  </bad-example>

  <good-example description="Reasonable requirement decomposition">
    ### Requirement 7: Comment Interaction

    **User Story:** As Viewer, I want to comment on works I like, so that I can exchange ideas with creators and other viewers.

    #### Acceptance Criteria

    1. WHEN Viewer enters comments on the work detail page and submits THEN the system should publish the comment and display it in real-time in the comment section
    2. WHEN Creator receives comments THEN the system should notify the creator through internal messaging
    3. WHEN comments contain sensitive words THEN the system should automatically intercept and prompt the user to modify
    4. WHEN Viewer clicks on a comment THEN the system should display replies and likes for that comment
  </good-example>

  <bad-example description="Requirement too complex">
    ### Requirement 1: Complete User System

    **User Story:** As User, I want to use complete system functionality.

    #### Acceptance Criteria

    1. User registration and login
    2. Personal information management
    3. Work publishing management
    4. Comment interaction features
    5. Message notification system
    6. Data statistical analysis
    7. Permission management control
    8. Payment functionality
    9. Customer service system
  </bad-example>
</examples>
```

### Requirement Priority Marking (Optional)
If you need to identify requirement priorities, you can use markers after the number:
- `[H]` - High priority
- `[M]` - Medium priority
- `[L]` - Low priority

```xml
<!DOCTYPE examples SYSTEM "/.ai/meta/example-schema.dtd" >
<examples>
  <example description="Priority marking example">
    ### Requirement 1[H]: User Authentication
    ### Requirement 2[M]: Email Notification
    ### Requirement 3[L]: Theme Switching
  </example>
</examples>
```

```xml
<!DOCTYPE example SYSTEM "/.ai/meta/example-schema.dtd" >
<example description="Complete example: Online education platform requirements document">
  # EduPlatform User Requirements Documentation

  ## Introduction

  This document records the detailed development requirements for developers developing online education platforms, aiming to provide efficient online teaching and learning experiences for teachers and students.

  **Primary Persona:** Online education teachers, University students, Vocational training students, Educational institution administrators

  **Operational Constraints:**
  1. Limited server budget, need to support at least 1000 concurrent users
  2. Must be compatible with mobile and desktop, minimum support iOS 12 and Android 8.0
  3. Video live streaming depends on third-party CDN services, need to control bandwidth costs
  4. Development team of 5 people, including 2 frontend, 2 backend, 1 tester

  **Non-Functional Priorities:**
  1. Video live streaming delay not exceeding 3 seconds, support reconnection after disconnection
  2. User data must be encrypted for storage, complying with personal information protection law requirements
  3. System availability reaches 99.5%, monthly downtime not exceeding 3.6 hours
  4. Page loading time controlled within 2 seconds

  **Deferred Scope:**
  1. AI intelligent recommendation of learning content functionality
  2. Virtual reality (VR) immersive classroom experience
  3. Multi-language internationalization support functionality

  ## Requirements

  ### Requirement 1[H]: Course Creation and Management

  **User Story:** As Teacher, I want to be able to create and manage online courses, so that I can flexibly arrange teaching content and progress.

  #### Acceptance Criteria

  1. WHEN Teacher logs in and enters course management page THEN the system should display "Create New Course" button and existing course list
  2. WHEN Teacher clicks "Create New Course" and fills in course information THEN the system should generate course homepage and support adding chapters
  3. WHEN Teacher uploads video course materials THEN the system should automatically transcode to multiple formats to adapt to different network environments
  4. WHEN Teacher sets course price THEN the system should support free, paid, and member-exclusive three modes
  5. WHEN course has student enrollment THEN Teacher should receive notification and be able to view student list

  ### Requirement 2[H]: Video Live Teaching

  **User Story:** As Teacher, I want to conduct real-time video live teaching, so that I can interact with students and answer questions.

  #### Acceptance Criteria

  1. WHEN Teacher enters the live room THEN the system should provide camera, microphone, and screen sharing options
  2. WHEN Teacher starts live streaming THEN the system should automatically notify enrolled students
  3. WHEN Students ask questions during live streaming THEN Teacher should be able to see real-time弹幕 and selectively reply
  4. WHEN network is unstable THEN the system should automatically switch to lower clarity smooth mode
  5. WHEN live streaming ends THEN the system should generate replay video and automatically link to course page

  ### Requirement 3[M]: Learning Progress Tracking

  **User Story:** As Student, I want to be able to view my learning progress, so that I can understand completion status and develop learning plans.

  #### Acceptance Criteria

  1. WHEN Student enters personal center THEN the system should display purchased course list and overall learning progress
  2. WHEN Student enters course detail page THEN the system should show completion status and learning time for each chapter
  3. WHEN Student completes a chapter THEN the system should automatically update progress and unlock the next chapter
  4. WHEN Student's learning time reaches system set value THEN the system should pop up rest reminder
  5. WHEN Student completes all courses THEN the system should generate electronic certificate and support sharing

  ### Requirement 4[M]: Interactive Discussion Area

  **User Story:** As Student, I want to be able to discuss and ask questions under courses, so that I can exchange learning experiences with classmates and teachers.

  #### Acceptance Criteria

  1. WHEN Student enters course discussion area THEN the system should display all discussion posts in chronological order
  2. WHEN Student posts questions THEN the system should @notify relevant teachers and other enrolled students
  3. WHEN Teacher replies to questions THEN the system should mark as "Answered" and highlight display
  4. WHEN Student feels a certain answer is useful THEN they can like that answer
  5. WHEN discussion contains inappropriate content THEN the system should automatically filter and submit for manual review

  ### Requirement 5[L]: Assignment Submission and Grading

  **User Story:** As Student, I want to submit assignments online and receive teacher feedback, so that I can timely understand my learning effectiveness.

  #### Acceptance Criteria

  1. WHEN Teacher publishes assignments THEN the system should notify all enrolled students and display deadline
  2. WHEN Student submits assignments THEN the system should support text, images, documents, and video multiple formats
  3. WHEN Student submits after timeout THEN the system should automatically close submission入口
  4. WHEN Teacher grades assignments THEN the system should support scoring, comments, and annotation functionality
  5. WHEN all assignments are graded THEN the system should generate class grade statistics
</example>
```


### Q & A

**Q: How detailed should requirements be?**
A: Each requirement should be detailed enough for developers to understand and implement, but avoid over-design. Generally 3-5 acceptance criteria are appropriate.

**Q: How should acceptance criteria be written to ensure testability?**
A: Use specific, observable results, avoid vague terms like "fast", "friendly", etc., instead use specific metrics like "response time < 2 seconds".

**Q: How to determine if requirement decomposition is reasonable?**
A: If a requirement has more than 5 acceptance criteria, consider whether it can be split; if fewer than 2, consider whether it's too simple.