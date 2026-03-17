# QUALITY_SCORE.md

This document visualizes the project's quality standards and the current "Quality Score."
AI agents must verify these standards upon task completion.

## 🎯 Quality Standards

| Metric | Target | Verification Method |
| :--- | :--- | :--- |
| **Test Success Rate** | 100% | `cargo test` etc. |
| **UT Coverage (Branch)** | Tier-specific (80-100%) | `cargo llvm-cov` etc. |
| **IT/E2E Coverage** | Full AC/State Coverage | Verify `docs/TEST_PLAN.md` |
| **Live Health** | SLO Met / Smoke Pass | Check monitoring results |
| **AC Density** | Avg 2.0+ per AC | `docs/TEST_PLAN.md` matrix |
| **Lint Errors/Warnings** | Zero | `cargo clippy` etc. |
| **Formatting Violations** | Zero | `cargo fmt --check` etc. |

## 📊 Current Quality Score

| Level | Score | Notes |
| :--- | :--- | :--- |
| **UT** | [00]% | Tier 1 coverage gaps, etc. |
| **IT** | [00]% | Major transitions covered |
| **E2E** | [00]% | All ACs linked |
| **Live** | [OK/--] | Canary tests running |

- **Last Updated**: 20XX-MM-DD
- **Current Status**: [🟢 GREEN / 🟡 YELLOW / 🔴 RED]
- **Remarks**: [e.g., Initial setup complete]

## 🛠️ Continuous Improvement (GC)

Agents should perform background refactoring (Garbage Collection) between tasks to improve the quality score. Fixing stale patterns or outdated documentation is also a vital part of quality improvement.
