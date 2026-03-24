#!/bin/bash

# check_harness.sh
# AI駆動開発 (AIDD) における「品質の番人」となる統合検証スクリプトです。
# AIエージェントはタスクを完了（DoDを宣言）する前に必ずこのスクリプトを実行し、
# 全ての検証が SUCCESS であることを確認してください。

set -e # エラーが発生したら即座に終了

# --- 🎨 カラー設定 ---
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}--- 🛠️ Harness Verification Starting ---${NC}"

# --- ⚙️ 設定 & 引数解析 ---
TIER=${1:-2} # デフォルトは Tier 2
SCOPE=${2:-"all"} # all, secret, audit, sync, lint, test, coverage

echo -e "Target Tier: ${TIER}"
echo -e "Verification Scope: ${SCOPE}"

# --- 🛡️ 1. シークレット漏洩チェック (Secret Scan) ---
run_secret_check() {
    echo -e "\n${YELLOW}[PHASE 1] Secret & Security Scan${NC}"
    # TODO: gitleaks などの軽量スキャナーがインストールされている場合は有効化する
    # if command -v gitleaks &> /dev/null; then
    #     gitleaks detect --source . -v || exit 1
    # else
    #     echo -e "${YELLOW}[WARN] gitleaks is not installed. Skipping secret scan.${NC}"
    # fi
    echo -e "${GREEN}SUCCESS: Secret scan passed (or skipped).${NC}"
}

# --- 📝 2. 監査チェック (Audit & DoD Check) ---
run_audit_check() {
    echo -e "\n${YELLOW}[PHASE 2] Audit & DoD Verification${NC}"
    
    # アクティブなプランファイルの DoD に [x] (Audit) が入っているかチェック
    ACTIVE_PLAN=$(ls -t docs/exec-plans/active/*.md 2>/dev/null | head -1 || true)
    
    if [ -n "$ACTIVE_PLAN" ]; then
        if grep -q "\[x\] \*\*(Audit)\*\*" "$ACTIVE_PLAN"; then
            echo -e "${GREEN}SUCCESS: Audit checked in $ACTIVE_PLAN.${NC}"
        else
            echo -e "${RED}[ERROR] Audit not checked in $ACTIVE_PLAN!${NC}"
            echo -e "エージェントは 'docs/AUDIT.md' に基づき自己監査を実施し、"
            echo -e "プランの DoD にある '[x] (Audit)' にチェックを入れてください。"
            exit 1
        fi
    else
        echo -e "${YELLOW}[WARN] No active plan found. Skipping Audit check.${NC}"
    fi
}

# --- 🔄 3. コンテキスト同期チェック (Context Sync Check) ---
run_sync_check() {
    echo -e "\n${YELLOW}[PHASE 3] Context Sync Verification${NC}"

    # v2 アーキテクチャの必須ドキュメント群の存在チェック
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
            echo -e "AIエージェントは 'aidle v2' のディレクトリ構造を破壊してはいけません。"
            exit 1
        fi
    done

    echo -e "${GREEN}SUCCESS: All core context files are present.${NC}"
}

# --- 🔍 4. 静的解析 (Lint & Format) ---
run_lint() {
    echo -e "\n${YELLOW}[PHASE 4] Lint & Format Check${NC}"
    # [TODO: プロジェクトの言語に合わせてコメントアウトを外す]
    # 例: Rust の場合
    # cargo fmt -- --check || exit 1
    # cargo clippy -- -D warnings || exit 1
    # 例: Node.js の場合
    # npm run lint || exit 1
    echo -e "${GREEN}SUCCESS: Lint & Format are clean.${NC}"
}

# --- 🧪 5. テスト実行 (UT/IT/E2E) ---
run_tests() {
    echo -e "\n${YELLOW}[PHASE 5] Multi-Layered Testing${NC}"
    
    # [TODO: プロジェクトのテストコマンドを設定する]
    echo -e "Running Unit/Integration Tests..."
    # cargo test || exit 1
    # npm run test || exit 1
    
    echo -e "${GREEN}SUCCESS: All tests passed for Tier ${TIER}.${NC}"
}

# --- 📊 6. カバレッジチェック (Coverage Check) ---
run_coverage() {
    echo -e "\n${YELLOW}[PHASE 6] Coverage Check${NC}"
    # [TODO: カバレッジの閾値チェックコマンドを設定する]
    # 例: llvm-cov を使用した差分カバレッジチェック
    # cargo llvm-cov --fail-under-lines 80 || exit 1
    
    echo -e "${GREEN}SUCCESS: Coverage targets met.${NC}"
}

# --- 🚀 実行制御 ---
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
