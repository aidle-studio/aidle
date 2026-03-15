#!/bin/bash

# check_harness.sh
# aidle プロジェクト自身の品質を機械的に検証する Harness スクリプトです。

echo "--- 🛠️  aidle Harness Verification Starting ---"

# 1. Lint (Clippy)
echo "Checking with Clippy..."
cargo clippy -- -D warnings || { echo "❌ Clippy failed. Please fix warnings/errors."; exit 1; }

# 2. Format
echo "Checking code format..."
cargo fmt -- --check || { echo "❌ Format check failed. Run 'cargo fmt' to fix."; exit 1; }

# 3. Tests
echo "Running tests..."
cargo nextest run || { echo "❌ Tests failed. Check the output above."; exit 1; }

echo "--- ✅ aidle Harness Verification SUCCESS ---"
exit 0
