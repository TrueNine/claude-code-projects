---
argument-hint: [standard_reference, user_revision_notes]
allowed-tools: Read, Write, Edit, Glob, Grep, Bash
description: Synchronize current file content with the latest public standards and user feedback
---

`renew` is used to quickly align existing files with the latest standards. It focuses on the current context file, executing minimal necessary structural and semantic revisions based on the standard reference provided by `$1` and user feedback from `$2`, while maintaining traceability of modification paths.

## Operating Principles
- ALWAYS read the standard content pointed to by `$1`, confirming it is the visible latest version; IF `$1` is an identifier rather than a file, first locate the corresponding standard document before entering the execution process.
- WHEN user feedback `$2` conflicts with standards, first explain the conflict and seek compliant alternatives before proceeding; NEVER forcibly override standard requirements.
- Change scope is limited to the current target file only, cross-file diffusion is prohibited; preserve existing compliant parts without making unrelated changes.

## Execution Process
### STEP-0 Parameter Validation
1. Verify whether `$1` and `$2` are provided; if either parameter is missing, MUST terminate and prompt for completion.
2. Confirm current file path and type; if not a supported prompt or configuration file, record the reason and stop.

### STEP-1 Standard Extraction
1. Use `Read` or `Glob` to locate the standard file corresponding to `$1`; if multiple versions exist, prioritize the one with the latest timestamp or marked as the latest version in `/.ai/meta/`, `/.ai/cmd/pe/`.
2. Summarize mandatory items, prohibited items, and format requirements from the standard to form an executable checklist.

### STEP-2 User Requirement Analysis
1. Break down `$2` into specific modification targets, marking parts that are consistent with or conflict with standards.
2. For conflicting items, provide alternative suggestions or explanations to ensure final modifications satisfy standard constraints.

### STEP-3 File Alignment
1. Review the current file against the checklist, locating paragraphs that do not comply with standards or user requirements.
2. Use `Edit` or `Write` to make minimal set modifications, maintaining existing structure and terminology consistency.
3. Immediately review paragraphs after each modification to confirm no new deviations have been introduced.

### STEP-4 Self-Check and Summary
1. Confirm against the checklist that all mandatory items have been implemented and prohibited items have not appeared.
2. Record a change summary, listing standard sources and responded user requirements to ensure output has traceability.

## Output Requirements
- Report in concise Chinese the standard basis for this adjustment, processed user feedback, and remaining TODOs.
- IF execution fails due to insufficient information, clearly indicate missing items and remediation suggestions.
- NEVER directly claim "completed" without providing evidence; explanations should include standard paths or identifiers and corresponding modified paragraphs.