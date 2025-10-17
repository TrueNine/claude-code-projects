---
argument-hint: [error_context, optional_additional_inputs]
allowed-tools: Read, TodoWrite
description: Output self-clarification summary WHEN AI encounters errors, explaining causes and evidence, and ensuring only explanations without executing any operations
---

`clarify-error` is used after errors occur during task execution to help AI conduct self-review of failures. The goal is to clearly explain the causes of errors, cite evidence, and subsequent verification that may be needed, rather than executing any repair actions.

## Applicable Scenarios
- User prompt errors
- User instruction errors
- Input ambiguity
- Responding to user dissatisfaction with AI output results
- Providing correction suggestions for prompts, instructions, etc.
- Helping users eliminate ambiguous expressions and context conflicts
- Not for defect analysis or debugging of code implementation itself

## Preconditions for Use
- ONLY call this command when the previous action has clearly failed or produced error output.
- IF sufficient context is lacking, first list missing information and explain that conclusions cannot be drawn, never guess.

## Clarification Process
### STEP-1 Collect Information
- Summarize error messages, stacks, failed command outputs, relevant input parameters, and preconditions.
- IF a piece of information is missing, directly record "Missing <information item>" and stop inference based on that information.

### STEP-2 Summarize Causes
- Provide "phenomenon -> evidence -> reasoning" chains for each potential cause, maintaining logical transparency.
- Clearly distinguish between established facts and hypotheses: mark as "hypothesis" when no evidence supports it, and explain required verification methods.

### STEP-3 Explain Judgment Basis
- Explain item by item why the current conclusion holds, citing specific evidence (such as log fragments, command output line numbers).
- IF multiple possible causes exist, sort by credibility or impact and explain the reasoning for sorting.

### STEP-4 Output Summary
- Output structure must include:
  - **Error Summary**: Describe error phenomena and trigger scenarios.
  - **Root Cause Analysis**: List possible root causes, each with evidence or hypothesis markers.
  - **Needed Evidence (if any)**: List information still missing and needs to be collected.
  - **Suggested Next Checks**: Propose feasible subsequent verification or troubleshooting directions, and clearly state that only suggestions are provided here, no operations will be executed.
- Emphasize at the end of summary: "This command only provides explanations and does not execute any modifications, commands, or repair actions."

## Prohibited Actions
- Prohibit calling any tools that may change environment state (such as Write, Edit, ApplyPatch, Execute, etc.).
- Prohibit giving definite conclusions for inferences with insufficient evidence, must use conditional or hypothetical language.
- Prohibit outputting execution confirmations or completion statements, only provide explanations and suggestions.

## Output Requirements

- Output in Simplified Chinese
- Use British logic