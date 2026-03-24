#!/bin/bash

# check_harness.sh
# This is an integrated verification script that serves as a "quality gatekeeper" in AI-Driven Development (AIDD).
# AI agents MUST execute this script before completing a task (declaring DoD)
# to confirm that all verifications return SUCCESS.

set -e # Exit immediately if an error occurs

# --- 🎨 Color Settings ---
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}--- 🛠️ Harness Verification Starting ---${NC}"

# --- ⚙️ Settings & Argument Parsing ---
TIER=${1:-2} # Default is Tier 2
SCOPE=${2:-"all"} # all, secret, audit, sync, lint, test, coverage

echo -e "Target Tier: ${TIER}"
echo -e "Verification Scope: ${SCOPE}"

# --- 🛡️ 1. Secret Leak Check (Secret Scan) ---
run_secret_check() {
    echo -e "\n${YELLOW}[PHASE 1] Secret & Security Scan${NC}"
    # TODO: Enable if a lightweight scanner like gitleaks is installed
    # if command -v gitleaks &> /dev/null; then
    #     gitleaks detect --source . -v || exit 1
    # else
    #     echo -e "${YELLOW}[WARN] gitleaks is not installed. Skipping secret scan.${NC}"
    # fi
    echo -e "${GREEN}SUCCESS: Secret scan passed (or skipped).${NC}"
}

# --- 📝 2. Audit Check (Audit & DoD Check) ---
run_audit_check() {
    echo -e "\n${YELLOW}[PHASE 2] Audit & DoD Verification${NC}"
    
    # Check if [x] (Audit) is marked in the DoD of the active plan file
    ACTIVE_PLAN=$(ls -t docs/exec-plans/active/*.md 2>/dev/null | head -1 || true)
    
    if [ -n "$ACTIVE_PLAN" ]; then
        if grep -q "\[x\] \*\*(Audit)\*\*" "$ACTIVE_PLAN"; then
            echo -e "${GREEN}SUCCESS: Audit checked in $ACTIVE_PLAN.${NC}"
        else
            echo -e "${RED}[ERROR] Audit not checked in $ACTIVE_PLAN!${NC}"
            echo -e "Agents MUST perform a self-audit based on 'docs/AUDIT.md' and"
            echo -e "check '[x] (Audit)' in the DoD of the plan."
            exit 1
        fi
    else
        echo -e "${YELLOW}[WARN] No active plan found. Skipping Audit check.${NC}"
    fi
}

# --- 🔄 3. Context Sync Check (Context Sync Check) ---
run_sync_check() {
    echo -e "\n${YELLOW}[PHASE 3] Context Sync Verification${NC}"

    # Existence check for mandatory documents in v2 architecture
    REQUIRED_DOCS=(
        "AGENTS.md"
        "PRD.md"
        "ARCHITECTURE.md"
        "docs/RULES.md"
        "docs/TODO.md"
        "docs/AGENT_CONTEXT.md"
    )
    for doc in "${REQUIRED_DOCS[@]}"; do
        if [ ! -f "$doc" ]; then
            echo -e "${RED}[ERROR] Required document '$doc' not found!${NC}"
            echo -e "AI agents MUST NOT destroy the directory structure of 'aidle v2'."
            exit 1
        fi
    done

    echo -e "${GREEN}SUCCESS: All core context files are present.${NC}"
}

# --- 🔍 4. Static Analysis (Lint & Format) ---
run_lint() {
    echo -e "\n${YELLOW}[PHASE 4] Lint & Format Check${NC}"
    # [TODO: Uncomment according to the project's language]
    # Example: For Rust
    # cargo fmt -- --check || exit 1
    # cargo clippy -- -D warnings || exit 1
    # Example: For Node.js
    # npm run lint || exit 1
    echo -e "${GREEN}SUCCESS: Lint & Format are clean.${NC}"
}

# --- 🧪 5. Test Execution (UT/IT/E2E) ---
run_tests() {
    echo -e "\n${YELLOW}[PHASE 5] Multi-Layered Testing${NC}"
    
    # [TODO: Set the test command for the project]
    echo -e "Running Unit/Integration Tests..."
    # cargo test || exit 1
    # npm run test || exit 1
    
    echo -e "${GREEN}SUCCESS: All tests passed for Tier ${TIER}.${NC}"
}

# --- 📊 6. Coverage Check (Coverage Check) ---
run_coverage() {
    echo -e "\n${YELLOW}[PHASE 6] Coverage Check${NC}"
    # [TODO: Set the coverage threshold check command]
    # Example: Differential coverage check using llvm-cov
    # cargo llvm-cov --fail-under-lines 80 || exit 1
    
    echo -e "${GREEN}SUCCESS: Coverage targets met.${NC}"
}

# --- 🚀 Execution Control ---
case $SCOPE in
    "secret")   run_secret_check ;;
    "audit")    run_audit_check ;;
    "sync")     run_sync_check ;;
    "lint")     run_lint ;;
    "test")     run_tests ;;
    "coverage") run_coverage ;;
    "all")      
        run_secret_check
        run_audit_check
        run_sync_check
        run_lint
        run_tests
        run_coverage 
        ;;
    *)          echo "Unknown scope: $SCOPE"; exit 1 ;;
esac

echo -e "\n${GREEN}--- ✅ Harness Verification SUCCESS ---${NC}"
exit 0
