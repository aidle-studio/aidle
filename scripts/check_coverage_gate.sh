#!/usr/bin/env bash
set -euo pipefail

LCOV_FILE="${1:-coverage.lcov}"
MIN_TOTAL="${MIN_TOTAL:-80}"
MIN_DIFF="${MIN_DIFF:-90}"
BASE_REF="${BASE_REF:-}"

if [[ ! -f "$LCOV_FILE" ]]; then
  echo "error: lcov file not found: $LCOV_FILE" >&2
  exit 2
fi

read -r total_covered total_lines < <(
  awk '
    /^SF:/ {
      file = substr($0, 4)
      in_src = (file ~ /(^|\/)src\/.*\.rs$/)
      next
    }
    /^DA:/ && in_src {
      split(substr($0, 4), a, ",")
      lines += 1
      if ((a[2] + 0) > 0) covered += 1
      next
    }
    END {
      printf "%d %d\n", covered, lines
    }
  ' "$LCOV_FILE"
)

if [[ "$total_lines" -eq 0 ]]; then
  echo "error: no Rust source coverage entries found under src/" >&2
  exit 2
fi

total_percent="$(awk -v c="$total_covered" -v t="$total_lines" 'BEGIN { printf "%.2f", (c * 100.0) / t }')"
echo "total coverage: ${total_percent}% (${total_covered}/${total_lines})"

if ! awk -v v="$total_percent" -v m="$MIN_TOTAL" 'BEGIN { exit !(v + 0 >= m + 0) }'; then
  echo "error: total coverage ${total_percent}% is below threshold ${MIN_TOTAL}%" >&2
  exit 1
fi

if [[ -z "$BASE_REF" ]] || ! git rev-parse --verify "$BASE_REF" >/dev/null 2>&1; then
  echo "diff coverage: skipped (BASE_REF is not set or not found)"
  exit 0
fi

changed_files="$(
  git diff --name-only "${BASE_REF}...HEAD" \
    | grep -E '^src/.*\.rs$' \
    || true
)"

if [[ -z "$changed_files" ]]; then
  echo "diff coverage: skipped (no changed Rust files under src/)"
  exit 0
fi

changed_joined="$(printf '%s\n' "$changed_files" | paste -sd ';' -)"

read -r diff_covered diff_lines < <(
  awk -v changed="$changed_joined" '
    function tracked(file) {
      n = split(changed, list, ";")
      for (i = 1; i <= n; i++) {
        if (file == list[i] || file ~ ("/" list[i] "$")) return 1
      }
      return 0
    }
    /^SF:/ {
      file = substr($0, 4)
      target = tracked(file)
      next
    }
    /^DA:/ && target {
      split(substr($0, 4), a, ",")
      lines += 1
      if ((a[2] + 0) > 0) covered += 1
      next
    }
    END {
      printf "%d %d\n", covered, lines
    }
  ' "$LCOV_FILE"
)

if [[ "$diff_lines" -eq 0 ]]; then
  echo "diff coverage: skipped (no executable lines found for changed files)"
  exit 0
fi

diff_percent="$(awk -v c="$diff_covered" -v t="$diff_lines" 'BEGIN { printf "%.2f", (c * 100.0) / t }')"
echo "diff coverage: ${diff_percent}% (${diff_covered}/${diff_lines})"

if ! awk -v v="$diff_percent" -v m="$MIN_DIFF" 'BEGIN { exit !(v + 0 >= m + 0) }'; then
  echo "error: diff coverage ${diff_percent}% is below threshold ${MIN_DIFF}%" >&2
  exit 1
fi

