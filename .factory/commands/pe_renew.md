---
argument-hint: standard_reference, user_revision_notes
allowed-tools: Read, Write, Edit, Glob, Grep, Bash
description: Synchronize file content based on latest standards and user feedback
---

`renew`: Synchronize current file based on `$1` (Standard) and `$2` (User Notes). Minimize revision, maintain traceability.

## Principles
- Deconstruct `$1`, `$2`.
- ALWAYS read latest `$1`. IF identifier -> Locate file.
- WHEN `$2` conflicts `$1` -> Explain and seek compliant alternative. NEVER force overwrite.
- Only modify current file. Retain compliant parts.

## Workflow

### STEP-0 Validate
1. Missing `$1`/`$2` -> Prompt to complete.
2. Confirm file type supported -> Else stop.

### STEP-1 Extract Standard
1. Locate `$1`. Prioritize latest/`_ai/meta/`/`_ai/commands/`.
2. Abstract: Mandatory/Forbidden/Format.

### STEP-2 Parse User Req
1. Deconstruct `$2`. Mark consistent/conflict.
2. Conflict -> Suggest/Explain.

### STEP-3 Align
1. Check against standard. Locate deviation.
2. `Edit`/`Write` minimal modification. Maintain structure/terms.
3. Immediate review.

### STEP-4 Review
1. Confirm: Mandatory implemented, Forbidden absent.
2. Record summary (Standard source/Response to req).

## Output
- Report: Standard basis/Handling opinion/Todo.
- Insufficient info -> Point out missing/Suggestion.
- NEVER empty talk "Completed", must list basis/changes.
