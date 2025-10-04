---
argument-hint: [your locale language example <en_US> | <zh_CN> ] [ <your project name> ]
allowed-tools: Read, Write, Edit, MultiEdit, Glob, Grep, Bash, TodoWrite, Task
description: Progressively guide users through the first step of specification-driven development in a specified language, generating structured requirements documentation
---

This document is used to guide users through the first step of specification-driven development, collecting and organizing project requirements through progressive dialogue. The document provides complete workflow guidance and standardized format templates, helping users transform vague ideas into structured requirements documentation. The entire process is divided into two main phases: preparation work and requirements collection, ensuring that the final requirements documentation is actionable and testable.

# [STEP-1]: Preparation Work

- Use `Search(pattern=".docs/spec/SPEC-1-REQUIREMENT.locale.md")` to check if `docs/.spec/SPEC-1-REQUIREMENTS.locale.md` exists
- If the file exists, use `Read` to load content and output a brief overview; otherwise, use `Write` to initialize an empty file
- Identify the user's preferred language represented by the first parameter `$1`, subsequent free text output will use this language by default; if the user specifies otherwise, follow the latest instruction
- After completing the overview, invite the user to relax, use their imagination, and describe the ideal system completion state, then wait for user input to enter [STEP-2]

# [STEP-2]: Patiently Listen to User's Initial Requirements Description

- Acknowledge that the user is in a relaxed, divergent thinking brainstorming state, where expressions may be jumpy or metaphorical
- Provide sequential guidance from an architect's perspective: use simple language, break the system into modules, scenarios, or processes, and inquire about key information step by step
- Listen carefully and record key information, politely ask clarifying questions when necessary to clarify vague points
- Respond only when clarification or confirmation is needed, maintain quiet listening at other times, avoid frequent interruptions
- If the user says "stop" or gives an equivalent request, immediately end the conversation
- The user's divergent descriptions need to be refined first, select content that aligns with the [APPENDIX-1] structure and has actionable value before using _Write_ to record; avoid copying sentence by sentence
- If no actionable points are identified yet, remain silent and continue listening until clear information is obtained
- Continue iterating at the user's pace until the user explicitly indicates "completion"
- After user confirms completion, remind them to use `/translate` or other tools to translate the document into English and record the external link in `project/CLAUDE.md` to improve subsequent coding experience

```xml

<Examples>
  <GoodExample description="Actively encourage users to continue diverging" userInput="I want to build a system that helps remote teams collaborate, but I can only think of making communication smoother for now.">
    <Tooling name="Update"
             params:path="docs/.spec/SPEC-1-REQUIREMENT.locale.md"
             description="Making remote team communication smoother" />
    Great starting point! I've recorded it. You can first imagine how this system would work perfectly in daily scenarios, like how team members would collaborate throughout the day? Feel free to share any fragments that come to mind, and we'll organize them into structured points together.
  </GoodExample>

  <BadExample description="Directly否定 user ideas, lacking encouragement" userInput="I want to build a system that helps remote teams collaborate, but I can only think of making communication smoother for now.">
    Your description is too vague, it's impossible to write requirements like this, come back when you've thought it through clearly.
  </BadExample>

  <GoodExample description="Maintain friendly guidance when facing particularly vague inspirations" userInput="I want to do something cool, maybe something that helps friends play games together? I haven't figured it out yet.">
    <Tooling name="Update"
             params:path="docs/.spec/SPEC-1-REQUIREMENT.locale.md"
             description="Help friends play games together more smoothly" />
    Sounds interesting! I've noted this goal for now. No rush to define specific features, you can start with small frustrations when you play games, like voice communication, team formation processes, or activity arrangements. Share a few real scenarios casually, and I'll help you organize which points are worth recording.
  </GoodExample>

  <GoodExample description="Continue diverging when clues are too abstract" userInput="Hmm... I want to do something related to AI, but I haven't figured out what specifically yet.">
    This direction has great potential! The current information is still quite abstract, so I won't write it into the document for now. You can first talk about the pain points you want to solve, who will use it, or the ideal scenario you expect to see. The more specific you share, the easier it will be for me to help extract actionable points.
  </GoodExample>

  <BadExample description="Press for details when user ideas are not yet formed" userInput="I want to do something cool, maybe something that helps friends play games together? I haven't figured it out yet.">
    You must first determine specific features, otherwise we can't continue, come back when you've thought it through.
  </BadExample>
</Examples>
```


## Locale Usage Conventions

- `$1` is the locale passed by the slash command (such as `zh_CN`, `en_US`), also representing the user's preferred language
- When communicating with users, default to using the language corresponding to `$1`; if the user switches languages or specifies special needs, follow the latest instruction
- When generating specification documents, except for fixed English titles or keywords, all other free text should use the `$1` language
- Follow common expressions and punctuation of the `$1` language, making the copy read naturally without translation feel
- When clarifying terminology or demonstrating examples, you can first explain in the `$1` language, and supplement with English comparison when necessary


# [APPENDIX-1]: Prescribed Format for Requirements Documentation

When outputting requirements documentation, you must strictly follow the following standard Markdown format specifications:

```md
# [PROJECT_NAME:- $2] User Requirements Documentation
```

**Format Description:**
- `[PROJECT_NAME:- $2]`: Placeholder, needs to be replaced with the actual project identifier (such as `mediacms`, `carshoping`, etc.)
- Document title must use English, following PascalCase naming conventions
- Document type is fixed as "User Requirements Documentation"

```xml
<Examples>
  <Example>
    # mediacms User Requirements Documentation
  </Example>

  <Example>
    # carshoping User Requirements Documentation
  </Example>

  <Example>
    # idea-mcp-plugin User Requirements Documentation
  </Example>
</Examples>
```

After an empty line, add the project introduction section in the following format:

```md
## Introduction

This document records the developer's detailed development requirements for developing a [project type] project, ...
```

**Writing Guidelines:**
- Use secondary heading `## Introduction`
- Description should start with a sentence in the `$1` language equivalent to "This document records the developer's detailed development requirements for developing"
- Briefly describe the project type and main objectives
- Control length to 2-5 sentences

```xml
<Examples>
  <Example description="MES system project example">
    ## Introduction

    This document records the developer's detailed development requirements for developing an MES system, aiming to achieve digital management and monitoring of production processes.
  </Example>

  <Example description="E-commerce project example">
    ## Introduction

    This document records the developer's detailed development requirements for developing an e-commerce front-end/back-end separated project, covering core functions such as product management, order processing, and user systems.
  </Example>
</Examples>
```

After an empty line, define the target user groups in the following format:

```md
**Primary Persona:** [User group description]
```

**Writing Specifications:**
- Use fixed English title `**Primary Persona:**`
- Use `$1` language to describe user groups, and list multiple groups according to common separators of that language (such as Chinese enumeration commas, English commas)
- Descriptions should be concise, accurate, and maintain high relevance to the project field
- Avoid subjective evaluations or artistic expressions

```xml
<Examples>
  <GoodExample description="Manufacturing project">
    **Primary Persona:** Manufacturing employees, manufacturing developers
  </GoodExample>

  <GoodExample description="Educational project">
    **Primary Persona:** College students, university teachers, student association modeling enthusiasts
  </GoodExample>

  <BadExample description="Error: Using Chinese title">
    **主要客户群体:** College students, university teachers, student association modeling enthusiasts
  </BadExample>

  <BadExample description="Error: Including subjective evaluations">
    **Primary Persona:** Charming business executives, technology experts pursuing excellence
  </BadExample>

  <BadExample description="Error: Description too vague">
    **Primary Persona:** Various users, people with needs
  </BadExample>
</Examples>
```

After an empty line, add optional project constraint conditions in the following format:

```md
**Operational Constraints:**
1. [Specific constraint description]
2. [Specific constraint description]
3. [Specific constraint description]
```

Reference types of constraint conditions (can be flexibly adjusted according to actual situations):
- Infrastructure: hardware configuration, network environment, deployment methods, etc.
- Technology stack: programming languages, framework choices, third-party services, etc.
- Team configuration: team size, skill structure, external collaboration, etc.
- Compliance requirements: industry standards, data security, privacy protection, etc.
- Operational support: availability goals, maintenance costs, scalability, etc.
- Business factors: budget limitations, time requirements, return on investment, etc.

```xml
<Examples>
  <GoodExample description="Video project constraints">
    **Operational Constraints:**
    1. Server performance is limited, requiring lightweight deployment and bandwidth consumption control
    2. Default dependency on external MySQL 8; video resources can be deployed on local disk or TOS, depending on cost considerations
    3. Access and playback volume is low, but need to ensure smooth access within the circle and easy backend maintenance
  </GoodExample>

  <GoodExample description="Financial project constraints">
    **Operational Constraints:**
    1. Must comply with national financial data security regulations, all transaction data must be encrypted for storage
    2. System availability requirement is 99.9%, annual downtime not exceeding 8.76 hours
    3. Development team of 3 people, including 1 frontend, 1 backend, 1 testing
    4. Budget limited to under 500,000, including one year of operational costs
  </GoodExample>

  <BadExample description="Description too vague">
    **Operational Constraints:**
    1. Server should be a bit better
    2. Need to complete quickly
    3. Budget is not enough
  </BadExample>

  <BadExample description="Using unprofessional expressions">
    **Operational Constraints:**
    1. Computer configuration can't be too bad, otherwise it won't run
    2. Better to use cloud services, more convenient
    3. Find a few people to do it casually
  </BadExample>
</Examples>
```

After an empty line, add optional non-functional priority descriptions in the following format:

```md
**Non-Functional Priorities:**
1. [Priority description]
2. [Priority description]
3. [Priority description]
```

```xml
<Examples>
  <GoodExample description="Clear non-functional priorities">
    **Non-Functional Priorities:**
    1. Enable HTTPS by default, prioritize using cloud provider free certificates
    2. Videos and covers should优先 go through TOS/CDN; if using local storage, need to provide capacity monitoring and cleanup strategies
    3. Currently only need desktop experience, mobile can be iterated when requirements arise later
    4. Provide containerized or scripted deployment for easy migration and quick recovery
    5. Implement lightweight logging and monitoring, and plan regular backups of databases and key data
  </GoodExample>

  <BadExample description="Vague non-functional priorities">
    **Non-Functional Priorities:**
    1. System should be secure and stable
    2. Speed should be a bit faster
    3. Interface should look good
    4. Later maintenance should be convenient
    5. Deployment should be simple
  </BadExample>

  <GoodExample description="Quantifiable non-functional priorities">
    **Non-Functional Priorities:**
    1. All sensitive data must be AES-256 encrypted for storage, transmission uses TLS 1.3
    2. Core transaction API response time ≤ 500ms, 99% of requests need to complete within 200ms
    3. System availability ≥ 99.9%, monthly downtime ≤ 43.2 minutes
    4. Support Chrome/Firefox/Safari latest two versions, IE11 minimum compatibility
    5. Code coverage ≥ 80%, key business 100% with integration tests
  </GoodExample>

  <BadExample description="Technology selection rather than priorities">
    **Non-Functional Priorities:**
    1. Use React framework for frontend development
    2. Backend uses Spring Boot framework
    3. Database uses MySQL 8.0
    4. Cache uses Redis
    5. Message queue uses RabbitMQ
  </BadExample>
</Examples>
```

After an empty line, add optional future scope descriptions in the following format:

```md
**Deferred Scope:**
1. [Feature description]
2. [Feature description]
3. [Feature description]
```

**Writing Guidelines:**
- Use English title `**Deferred Scope:**`
- List features not considered for current version but may need to be implemented in the future
- Each feature should be concise and highlight core value
- Avoid duplication with existing requirements
- Ordered list content should be written using the `$1` language

```xml
<Examples>
  <GoodExample description="Video platform future features">
    **Deferred Scope:**
    1. Talent marketplace recruitment capability, connecting creators with enterprises
    2. Short drama sales and paid unlock modules, supporting content monetization
    3. Creator community features, supporting work exchange and collaboration
  </GoodExample>

  <GoodExample description="E-commerce platform future features">
    **Deferred Scope:**
    1. Social sharing functionality, allowing users to share products to various platforms
    2. Membership points system, enhancing user loyalty
    3. Multi-language internationalization support, expanding overseas markets
  </GoodExample>

  <BadExample description="Description too vague">
    **Deferred Scope:**
    1. Some other functions
    2. Things to add later
    3. Things to do when there's money
  </BadExample>

  <BadExample description="Duplicated with current requirements">
    **Deferred Scope:**
    1. User login and registration (already in basic features)
    2. Product display page (already in core requirements)
    3. Order management functionality (already in must-implement)
  </BadExample>
</Examples>
```


Next is the core requirements list, which is the most important part of the entire document and must strictly follow the following specifications:

## Requirements Format Specifications

### Basic Structure
```md
## Requirements

### Requirement [Number]: [Requirement Name]

**User Story:** As [User Role], I want [Function to complete], so that [Value gained].

#### Acceptance Criteria

1. WHEN [Trigger condition] THEN [Expected result]
2. WHEN [Trigger condition] THEN [Expected result]
3. WHEN [Trigger condition] THEN [Expected result]
```

### Writing Specification Requirements

1. **User Story**
- Must use standard format: `As [Role], I want [Function], so that [Value]`
- Role should be specific (such as "Creator" rather than "User")
- Value should be clear (answer "why this function is needed")
- Use `$1` language to write [Role], [Function], [Value]

2. **Acceptance Criteria**
- Must use Given-When-Then format
- Each criterion must be independent and testable
- Avoid technical implementation details, focus on business behavior
- Use `$1` language to write [Trigger condition], [Expected result]

3. **Requirement Decomposition Principles**
- Each requirement should be independent and have clear value
- Avoid being too large (over 5 acceptance criteria should consider decomposition)
- Avoid being too small (fewer than 2 acceptance criteria should consider merging)

```xml
<Examples>
  <GoodExample description="Complete user requirement">
    ### Requirement 3: User Work Management

    **User Story:** As Creator, I want to be able to manage all my works, so that I can edit or delete content at any time.

    #### Acceptance Criteria

    1. WHEN Creator logs in and enters personal center THEN system should display a list of all their works, including thumbnails, titles, publication time, and view counts
    2. WHEN Creator clicks work edit button THEN system should jump to edit page, retaining original content and allowing modification of all information
    3. WHEN Creator deletes a work THEN system should require secondary confirmation, and upon success remove it from the list and notify the user
    4. WHEN works are collected or commented on by other users THEN Creator should be able to see related statistics on the management page
  </GoodExample>

  <BadExample description="Missing user value">
    ### Requirement 2: User Login

    **User Story:** As User, I want to login to the system.

    #### Acceptance Criteria

    1. Enter username and password
    2. Click login button
    3. Login successful
  </BadExample>

  <GoodExample description="Technology-agnostic acceptance criteria">
    ### Requirement 5: Content Recommendation

    **User Story:** As Viewer, I want the system to recommend short drama content I'm interested in, so that I can discover more quality works.

    #### Acceptance Criteria

    1. WHEN Viewer browses homepage THEN system should recommend similar type works based on their viewing history
    2. WHEN Viewer finishes watching a work THEN system should recommend other works by related creators
    3. WHEN Viewer continuously skips multiple recommendations THEN system should adjust recommendation algorithm to provide more accurate content
  </GoodExample>

  <BadExample description="Including technical implementation">
    ### Requirement 4: Video Upload

    **User Story:** As Creator, I want to upload videos.

    #### Acceptance Criteria

    1. Call backend API interface /api/v1/videos
    2. Use MySQL to store video information
    3. Video files stored in OSS object storage
  </BadExample>

  <GoodExample description="Reasonable requirement decomposition">
    ### Requirement 7: Comment Interaction

    **User Story:** As Viewer, I want to comment on works I like, so that I can exchange ideas with creators and other viewers.

    #### Acceptance Criteria

    1. WHEN Viewer inputs comment on work detail page and submits THEN system should publish comment and display it in real-time in comment section
    2. WHEN Creator receives comment THEN system should notify creator through internal messaging
    3. WHEN comment contains sensitive words THEN system should automatically block and prompt user to modify
    4. WHEN Viewer clicks on a comment THEN system should display replies and like counts for that comment
  </GoodExample>

  <BadExample description="Requirement too complex">
    ### Requirement 1: Complete User System

    **User Story:** As User, I want to use complete system functionality.

    #### Acceptance Criteria

    1. User registration and login
    2. Personal information management
    3. Work publishing and management
    4. Comment interaction functionality
    5. Message notification system
    6. Data statistical analysis
    7. Permission management control
    8. Payment functionality
    9. Customer service system
  </BadExample>
</Examples>
```

### Requirement Priority Marking (Optional)
If you need to mark requirement priorities, you can use markers after the number:
- `[H]` - High priority
- `[M]` - Medium priority
- `[L]` - Low priority

```xml
<Examples>
  <Example description="Priority marking example">
    ### Requirement 1[H]: User Authentication
    ### Requirement 2[M]: Email Notification
    ### Requirement 3[L]: Theme Switching
  </Example>
</Examples>
```

```xml
<Example description="Complete example: Online education platform requirements document">
  # EduPlatform User Requirements Documentation

  ## Introduction

  This document records the developer's detailed development requirements for developing an online education platform, aiming to provide efficient online teaching and learning experiences for teachers and students.

  **Primary Persona:** Online education teachers, college students, vocational training students, educational institution administrators

  **Operational Constraints:**
  1. Server budget is limited, needs to support at least 1000 concurrent users
  2. Must be compatible with mobile and desktop, minimum support for iOS 12 and Android 8.0
  3. Video live streaming depends on third-party CDN services, need to control bandwidth costs
  4. Development team of 5 people, including 2 frontend, 2 backend, 1 testing

  **Non-Functional Priorities:**
  1. Video live streaming delay not exceeding 3 seconds, supporting reconnection after disconnection
  2. User data must be encrypted for storage, complying with personal information protection law requirements
  3. System availability reaches 99.5%, monthly downtime not exceeding 3.6 hours
  4. Page loading time controlled within 2 seconds

  **Deferred Scope:**
  1. AI intelligent recommendation learning content functionality
  2. Virtual reality (VR) immersive classroom experience
  3. Multi-language internationalization support functionality

  ## Requirements

  ### Requirement 1[H]: Course Creation and Management

  **User Story:** As Teacher, I want to be able to create and manage online courses, so that I can flexibly arrange teaching content and schedule.

  #### Acceptance Criteria

  1. WHEN Teacher logs in and enters course management page THEN system should display "Create New Course" button and existing course list
  2. WHEN Teacher clicks "Create New Course" and fills in course information THEN system should generate course homepage and support adding chapters
  3. WHEN Teacher uploads video courseware THEN system should automatically transcode to multiple formats to adapt to different network environments
  4. WHEN Teacher sets course price THEN system should support free, paid, and member-exclusive three modes
  5. WHEN course has student registrations THEN Teacher should receive notification and be able to view student list

  ### Requirement 2[H]: Video Live Teaching

  **User Story:** As Teacher, I want to conduct real-time video live teaching, so that I can interact with students and answer questions.

  #### Acceptance Criteria

  1. WHEN Teacher enters live room THEN system should provide camera, microphone, and screen sharing options
  2. WHEN Teacher starts live streaming THEN system should automatically notify enrolled students
  3. WHEN Students ask questions during live stream THEN Teacher should be able to see real-time弹幕 and selectively reply
  4. WHEN network is unstable THEN system should automatically switch to lower clarity smooth mode
  5. WHEN live streaming ends THEN system should generate replay video and automatically associate it to course page

  ### Requirement 3[M]: Learning Progress Tracking

  **User Story:** As Student, I want to be able to view my learning progress, so that I can understand completion status and create learning plans.

  #### Acceptance Criteria

  1. WHEN Student enters personal center THEN system should display purchased course list and overall learning progress
  2. WHEN Student enters course detail page THEN system should display completion status and learning duration for each chapter
  3. WHEN Student completes a chapter THEN system should automatically update progress and unlock next chapter
  4. WHEN Student's learning duration reaches system-set value THEN system should pop up rest reminder
  5. WHEN Student completes all courses THEN system should generate electronic certificate and support sharing

  ### Requirement 4[M]: Interactive Discussion Area

  **User Story:** As Student, I want to be able to discuss and ask questions under courses, so that I can exchange learning experiences with classmates and teachers.

  #### Acceptance Criteria

  1. WHEN Student enters course discussion area THEN system should display all discussion posts in chronological order
  2. WHEN Student posts question THEN system should @notify relevant teachers and other enrolled students
  3. WHEN Teacher replies to question THEN system should mark as "answered" and highlight display
  4. WHEN Student finds an answer useful THEN can like that answer
  5. WHEN discussion contains inappropriate content THEN system should automatically filter and submit for manual review

  ### Requirement 5[L]: Assignment Submission and Grading

  **User Story:** As Student, I want to submit assignments online and receive teacher feedback, so that I can timely understand my learning effectiveness.

  #### Acceptance Criteria

  1. WHEN Teacher publishes assignment THEN system should notify all enrolled students and display deadline
  2. WHEN Student submits assignment THEN system should support text, images, documents, and video multiple formats
  3. WHEN Student submits after timeout THEN system should automatically close submission entrance
  4. WHEN Teacher grades assignment THEN system should support scoring, comments, and annotation functions
  5. WHEN all assignments are graded THEN system should generate class grade statistics
</Example>
```


### Q & A

**Q: How detailed should requirements be?**
A: Each requirement should be detailed enough for developers to understand and implement, but avoid over-design. Generally 3-5 acceptance criteria are appropriate.

**Q: How should acceptance criteria be written to ensure testability?**
A: Use specific, observable results, avoid vague words like "fast" or "friendly", instead use specific metrics like "response time < 2 seconds".

**Q: How to determine if requirement decomposition is reasonable?**
A: If a requirement has more than 5 acceptance criteria, consider whether it can be split; if fewer than 2, consider whether it's too simple.