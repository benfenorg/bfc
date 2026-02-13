#!/usr/bin/env bash
# 逐个运行 crates/sui-bridge e2e_tests 下的单测，最后输出失败的单测名称
# 用法: ./run_e2e_tests.sh  或  bash run_e2e_tests.sh
# 可在任意目录执行；如需指定工作区根目录可设置环境变量 CARGO_ROOT

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# 工作区根目录（sui-bridge 的 Cargo.toml 上两级）
CARGO_ROOT="${CARGO_ROOT:-$(cd "$SCRIPT_DIR/../.." && pwd)}"
cd "$CARGO_ROOT"

echo "=== 在 $CARGO_ROOT 下运行 sui-bridge e2e 单测 ==="
echo ""

# 从固定列表文件读取测试（跳过空行和 # 开头的注释）
TESTS_LIST_FILE="${SCRIPT_DIR}/e2e_tests_list.txt"
if [ ! -f "$TESTS_LIST_FILE" ]; then
  echo "未找到测试列表文件: $TESTS_LIST_FILE"
  exit 1
fi
TESTS=$(grep -v '^[[:space:]]*#' "$TESTS_LIST_FILE" | grep -v '^[[:space:]]*$' || true)

if [ -z "$TESTS" ]; then
  echo "测试列表为空（请检查 $TESTS_LIST_FILE）"
  exit 1
fi
echo "共 $(echo "$TESTS" | wc -l | tr -d ' ') 个单测（来源: e2e_tests_list.txt）"
echo ""

TOTAL=0
PASSED=0
FAILED_NAMES=()

while IFS= read -r full_name; do
  [ -z "$full_name" ] && continue
  TOTAL=$((TOTAL + 1))
  # 用完整路径跑单个测试（--lib --exact --nocapture，单测失败不退出脚本）
  if cargo test --package sui-bridge --lib -- "$full_name" --exact --nocapture 2>&1; then
    PASSED=$((PASSED + 1))
    echo "[PASS] $full_name"
  else
    FAILED_NAMES+=("$full_name")
    echo "[FAIL] $full_name"
  fi
  echo "---"
done <<< "$TESTS"

echo ""
echo "=============================================="
echo "  合计: $TOTAL  通过: $PASSED  失败: $((TOTAL - PASSED))"
echo "=============================================="

if [ ${#FAILED_NAMES[@]} -gt 0 ]; then
  echo ""
  echo "失败的单测名称："
  for n in "${FAILED_NAMES[@]}"; do
    echo "  - $n"
  done
  exit 1
fi

exit 0
