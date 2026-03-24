<!--
[AI Agent Instructions for Quality Score]
⚠️ WARNING: This document manages the current "Quality Score" of the entire project.
Agents should update this section to the latest state upon completion of a milestone.
-->

# QUALITY_SCORE.md (Current Quality Status)

This document is a dashboard showing to what extent the implemented system satisfies the quality criteria.

## 🎯 1. Mechanical Quality Standards

The system must satisfy the following mechanical quality indicators.

| Indicator | Target Value | Verification Method |
| :--- | :--- | :--- |
| **Test Success Rate** | 100% | `scripts/check_harness.sh` |
| **Tier 1 Test Density** | Avg 5.0+ per AC | Review of `docs/TEST_PLAN.md` |
| **Tier 2 Test Density** | Avg 3.0+ per AC | Review of `docs/TEST_PLAN.md` |
| **Tier 1 Coverage (Branch)**| 100% | Coverage measurement tool |
| **Lint Errors/Warnings** | Zero | Project Linter |
| **Format Violations** | Zero | Project Formatter |

---

## 📊 2. Current Project Quality Score

| Level | Score | Remarks |
| :--- | :--- | :--- |
| **UT** | [00]% | [TODO: Coverage or missing parts] |
| **IT** | [00]% | [TODO: FMEA coverage status, etc.] |
| **E2E** | [00]% | [TODO: Coverage status of main scenarios] |
| **Live**| [OK/--] | [TODO: SLO and monitoring status] |

- **Last Updated**: YYYY-MM-DD
- **Current Status**: [🟢 GREEN / 🟡 YELLOW / 🔴 RED]
- **Remarks**: [Example: Initial setup completed. Zero technical debt.]

## 🛠️ Continuous Improvement (GC)

Agents should perform refactoring (garbage collection) in the background between tasks to maintain and improve this quality score.
