---
argument-hint: [error_context, optional_additional_inputs]
allowed-tools: Read, TodoWrite
description: When the AI encounters an error, output a self-clarification summary that explains the cause and evidence, and ensure it only provides explanation without performing any action
---

`clarify-error` is used after an error occurs during a task to help the AI self-examine the failure. The goal is to clearly explain why the error happened, cite the evidence, and outline validations that MAY be needed next, rather than executing any fix actions.

## Applicable Scenarios
- User prompt errors
- User instruction errors
- Ambiguity in inputs
- Responding to user dissatisfaction with the AI output
- Proposing corrections to prompts or instructions
- Helping the user eliminate ambiguous expressions and context conflicts
- Not for defect analysis or debugging of code implementations themselves

## Preconditions
- ONLY CALL this command WHEN the previous action has clearly failed or produced an erroneous output.
- IF sufficient context is lacking, FIRST list the missing information and explain that a conclusion cannot be reached; NEVER guess without evidence.

## Clarification Process
### STEP-1 Collect Information
- Summarize error messages, stacks, failed command outputs, related input parameters, and preconditions.
- IF any item is missing, explicitly record "Missing <item>" and STOP inference based on that missing item.

### STEP-2 Synthesize Root Causes
- For each potential cause, provide a "Phenomenon -> Evidence -> Reasoning" chain, keeping logic transparent.
- Clearly distinguish established facts from assumptions: WHEN there is no supporting evidence, mark as "Assumption" and state the required validation method.

### STEP-3 Explain Judgement Basis
- Explain, item by item, why the current conclusion holds, citing specific evidence (e.g., log fragments, line numbers of command outputs).
- WHEN multiple potential causes exist, order them by credibility or impact and explain the ordering rationale.

### STEP-4 Output Summary
- The output structure MUST include:
  - **Error Summary**: Describe the error phenomenon and the triggering context.
  - **Root Cause Analysis**: List possible root causes, each with evidence or an assumption mark.
  - **Needed Evidence (if any)**: List information still missing and required for collection.
  - **Suggested Next Checks**: Propose feasible follow-up validations or investigation directions, and explicitly state that only suggestions are provided here and no actions will be executed.
- Emphasize at the end: "This command only provides explanations; it does not execute any modifications, commands, or fixes."

## Prohibitions
- NEVER CALL any tools that MAY change the environment state (such as Write, Edit, ApplyPatch, Execute, etc.).
- NEVER give definitive conclusions for inferences without sufficient evidence; you MUST use conditional or hypothetical language.
- NEVER output execution confirmations or completion statements; ONLY provide explanations and suggestions.

## Output Requirements

- Output in Simplified Chinese
- Use British-style logic
