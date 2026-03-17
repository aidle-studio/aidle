# DESIGN.md

This document defines the project's design and UX guidelines, common components, and styling conventions.

> **💡 Pointer**
> For the fundamental UX philosophy, target personas, and the "Why" behind the design, please refer to **[`PRODUCT_SENSE.md`](PRODUCT_SENSE.md)** first. This document serves as the implementation guide to translate those intents into concrete UI components and styles.

## 🎨 Design Principles

AI agents must adhere to the following design guidelines:

1. **Consistency**: Use identical designs and behaviors for identical features and operations.
2. **Hierarchy**: Use typography, color, and spacing to clarify information priority.
3. **Interactivity**: Provide appropriate feedback for all user actions.

## 🧩 Common Components

Maximize the reuse of existing components when implementing new screens or features.

- **Button**: Use standard colors (Primary, Secondary, Danger) based on role.
- **Input**: Standardize error state displays.
- **Layout**: Follow the grid system or standard layout templates.

## 💄 Styling

- **CSS/UI Framework**: [e.g., Tailwind CSS / Shadcn UI]
- **Naming Convention**: [e.g., BEM / Utility-First]
- **Accessibility**: Ensure contrast ratios, screen reader support, and keyboard navigability.
