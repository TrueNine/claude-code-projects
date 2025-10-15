---
argument-hint: [error_context, optional_additional_inputs]
allowed-tools: Read, TodoWrite
description: OUTPUT self-clarification summary WHEN AI encounters errors, explaining causes and evidence, ensuring only explanations without executing any operations
---

`clarify-error` is used to help AI conduct self-reflection after errors occur during task execution. The goal is to clearly explain why the error happened, cite evidence, and what verification may be needed, rather than executing any fix actions.

## Applicable Scenarios
- User prompt errors
- User instruction errors
- Ambiguous input
- Addressing user dissatisfaction with AI output results
- Providing correction suggestions for prompts, instructions, etc.
- Helping users eliminate ambiguous expressions and context conflicts
- NOT for code implementation defect analysis or debugging

## Pre-conditions for Use
- ONLY invoke this command when the previous action has clearly failed or produced error output.
- IF sufficient context is missing, first list missing information and explain why conclusions cannot be reached, never guess without basis.

## Clarification Process
### STEP-1 Collect Information
- Summarize error messages, stack traces, failed command outputs, relevant input parameters, and preconditions.
- IF any information item is missing, directly record "Missing <information item>" and stop inferences based on that information.

### STEP-2 Identify Causes
- FOR each potential cause, provide "Phenomenon -> Evidence -> Reasoning" chain, maintaining logical transparency.
- Clearly distinguish between established facts and assumptions: WHEN lacking evidence, mark as "Assumption" and explain required verification methods.

### STEP-3 Explain Judgment Basis
- Explain item by item why the current conclusion holds, citing specific evidence (such as log snippets, command output line numbers).
- WHEN multiple possible causes exist, sort by credibility or impact, explaining the reasoning for the ordering.

### STEP-4 Output Summary
- Output structure MUST include:
  - **Error Summary**: Describe error phenomenon and trigger scenarios.
  - **Root Cause Analysis**: List possible root causes, each with evidence or assumption marking.
  - **Needed Evidence (if any)**: List still missing information that needs to be collected.
  - **Suggested Next Checks**: Propose feasible verification or troubleshooting directions for subsequent steps, clearly stating this only provides suggestions and no operations will be executed.
- At the end of the summary, emphasize: "This command only provides explanations and does not execute any modifications, commands, or fix actions."

## Prohibited Actions
- NEVER call any tools that might change environment state (such as Write, Edit, ApplyPatch, Execute, etc.).
- NEVER provide definite conclusions for inferences with insufficient evidence, MUST use conditional or hypothetical language.
- NEVER output execution confirmations or completion statements, may only provide explanations and suggestions.