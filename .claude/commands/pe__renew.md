---
argument-hint: [standard_reference, user_revision_notes]
allowed-tools: Read, Write, Edit, Glob, Grep, Bash
description: Align the current file with the latest public standard while incorporating user feedback.
---

`renew` is used to rapidly synchronize an existing file with the latest standards. It focuses on the current context file, using the standard reference supplied by `$1` and the user feedback in `$2` to perform the minimal structural and semantic adjustments required, while keeping all change paths traceable.

## Operating Principles
- Decompose parameter `$1` and optional parameter `$2` from `$ARGUMENTS`.
- ALWAYS read the standard content pointed to by `$1` and confirm it is the visible latest version; IF `$1` is an identifier rather than a file, locate the associated standard document before proceeding.
- WHEN user feedback `$2` conflicts with the standard, explain the conflict and propose a compliant alternative before continuing; NEVER override standard requirements.
- Keep the scope of changes limited to the current target file. Preserve already compliant sections and avoid unrelated modifications.

## Execution Flow
### STEP-0 Parameter Validation
1. Verify whether `$1` and `$2` are provided; IF either is missing, THEN terminate and request completion.
2. Confirm the current file path and type. IF it is not a supported prompt or configuration file, record the reason and stop.

### STEP-1 Standard Extraction
1. Use `Read` or `Glob` to locate the standard file referenced by `$1`. IF multiple versions exist, prioritize the newest timestamp or the version marked as latest under `/__ai/meta/` or `/__ai/cmd/pe/`.
2. Summarize mandatory items, prohibitions, and formatting requirements from the standard to form an actionable checklist.

### STEP-2 User Requirement Analysis
1. Break `$2` down into concrete modification goals and mark which items align with or conflict with the standard.
2. Provide alternative suggestions or explanations for conflicting items to ensure the final adjustments satisfy the standard constraints.

### STEP-3 File Alignment
1. Review the current file against the checklist to pinpoint sections that violate the standard or user requirements.
2. Use `Edit` or `Write` to make the minimal set of modifications, ensuring structural and terminology consistency.
3. Recheck each paragraph immediately after modification to confirm no new deviations were introduced.

### STEP-4 Self-Check and Summary
1. Confirm that all mandatory items have been implemented and no prohibitions appear, referencing the checklist.
2. Record a change summary that lists the standard sources and addresses the user requirements so the output remains traceable.

## Output Requirements
- Provide a concise Chinese report summarizing the standards applied, the user feedback addressed, and any remaining TODO items.
- IF execution is blocked due to insufficient information, clearly identify the missing elements and offer remediation suggestions.
- NEVER claim "已完成" without supporting evidence. The report MUST include the standard path or identifier and the corresponding modified sections.
