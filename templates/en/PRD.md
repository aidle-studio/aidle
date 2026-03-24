<!--
[AI Agent Instructions for PRD]
⚠️ WARNING: This file is the top-level document defining the product's "Requirements (What)" and "Purpose (Why)."
To prevent context bloat, strictly adhere to the following rules:
1. **Do Not Write Implementation Details**: Only write "what users can do" here. Details such as "API schemas" or "specific UI button placements" should be written in `docs/features/*.md`.
2. **Maintain the Core Feature List Table**: Ensure the table in "3. Core Features" functions as a table of contents (index) for all functional specifications (Functional Specs) in this project.
-->

# Product Requirements Document (PRD)

## 1. 🎯 Project Overview and Goals (Vision & Goals)
- **Elevator Pitch**: [TODO: Describe what the product is in one sentence]
- **Core Value**: [TODO: What is the greatest value provided to users?]

## 2. 👤 Target Audience and Problem Statement (Target Audience)
- **Who**: [TODO: Who will use it? (Persona)]
- **Problem**: [TODO: What is the current problem they are facing?]

## 2. 🎯 Product Sense (Core Values)

AI agents should prioritize the following values when making implementation decisions. If you are unsure between multiple tasks or implementation approaches, return to these values to make a judgment, and seek agreement from humans if you have any doubts.

- **Simplicity**: Prefer the simplest approach to solving user problems over complex features.
- **Speed**: Minimize waiting time and ensure a stress-free response.
- **Accuracy**: Always provide consistent and correct results relative to user expectations.
- **UX Principles**: Aim for a state where only necessary information is provided at the appropriate timing, and it works well "right out of the box (default optimization)" without the user having to configure anything.

## 3. ✨ Core Features
* Note: Specific behaviors and acceptance criteria (AC) for each feature should be recorded in the linked "Functional Specification." Do not write implementation details in this file.

| Feature | Description | Link to Functional Specs |
| :--- | :--- | :--- |
| [e.g., User Authentication] | [e.g., Login functionality using email address] | ➡️ `docs/features/auth.md` |
| [TODO] | [TODO] | ➡️ `docs/features/[TODO].md` |

## 4. 🚫 Out of Scope
- [TODO: Clearly state "what we intentionally will not do this time" to prevent scope creep]

## 5. Robustness and Non-Functional Requirements
- **Performance**: [TODO: Goals for response time, number of concurrent connections, etc.]
- **Constraints**: [TODO: Constraints such as required platforms or technologies that must not be used]
