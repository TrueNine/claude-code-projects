---
argument-hint: [standard_reference, user_revision_notes]
allowed-tools: Read, Write, Edit, Glob, Grep, Bash
description: Synchronize current file content with latest public standards and user feedback
---

`renew` is used to quickly align current files with the latest standards. It focuses on the current context file, executing minimum necessary structural and semantic revisions based on the standard reference provided by `$1` and user feedback from `$2`, while maintaining traceable modification paths.

## Operational Principles
- Extract parameters `$1` and optional parameter `$2` from `$ARGUMENTS`
- ALWAYS read the standard content pointed to by `$1`, confirming it's the visible latest version; IF `$1` is an identifier rather than a file, first locate the corresponding standard document before entering the execution flow.
- WHEN user feedback `$2` conflicts with standards, first explain the conflict and seek compliant alternatives before continuing; NEVER forcibly override standard requirements.
- Change scope is limited to the current target file only, cross-file propagation is prohibited; preserve existing compliant parts without making unrelated changes.

## Execution Flow
### STEP-0 Parameter Validation
1. Verify that `$1` and `$2` are provided; missing either parameter MUST terminate execution and prompt for completion.
2. Confirm current file path and type; if not a supported prompt or configuration file, record the reason and stop.

### STEP-1 Standard Extraction
1. Use `Read` or `Glob` to locate the standard file corresponding to `$1`; if multiple versions exist, prioritize the one with the latest timestamp or marked as the latest version in `/__ai/meta/` or `/__ai/cmd/pe/`.
2. Extract mandatory items, prohibited items, and format requirements from the standard to form an executable checklist.

### STEP-2 User Requirement Analysis
1. Break down `$2` into specific modification targets, marking parts that are consistent or conflicting with standards.
2. Provide alternative suggestions or explanations for conflicting items, ensuring final modifications satisfy standard constraints.

### STEP-3 File Alignment
1. Review the current file against the checklist, locating paragraphs that don't comply with standards or user requirements.
2. Use `Edit` or `Write` to make minimal set modifications, maintaining existing structure and terminology consistency.
3. Immediately review paragraphs after each modification, confirming no new deviations have been introduced.

### STEP-4 Self-Check and Summary
1. Confirm against the checklist that all mandatory items are implemented and prohibited items are absent.
2. Record change summary, listing standard sources and addressed user requirements, ensuring output has traceability.

## Output Requirements
- Report in concise English the standard basis for this adjustment, processed user feedback, and remaining todos.
- IF execution cannot be completed due to insufficient information, clearly indicate missing items and remediation suggestions.
- NEVER directly claim "completed" without providing evidence; explanations should include standard paths or identifiers and corresponding modified paragraphs.
