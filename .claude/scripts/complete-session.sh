#!/bin/bash

###############################################################################
# complete-session.sh
#
# 完成当前 session 的任务：
# 1. 前置检查（git 状态、工作目录）
# 2. 执行操作（质量检查、生成提交信息、暂存更改）
# 3. 输出结果（显示提交信息、提示下一步）
#
# 使用方式：
#   bash .claude/scripts/complete-session.sh
#
###############################################################################

set -e  # 遇到错误立即退出

# ============================================================================
# 配置
# ============================================================================

PROJECT_ROOT="$(pwd)"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# ============================================================================
# 颜色定义
# ============================================================================

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'  # No Color

# ============================================================================
# 辅助函数
# ============================================================================

log_info() {
  echo -e "${BLUE}ℹ ${1}${NC}"
}

log_success() {
  echo -e "${GREEN}✓ ${1}${NC}"
}

log_error() {
  echo -e "${RED}✗ ${1}${NC}"
}

log_warn() {
  echo -e "${YELLOW}⚠ ${1}${NC}"
}

separator() {
  echo -e "${BLUE}$(printf '=%.0s' {1..60})${NC}"
}

# ============================================================================
# Phase 1: 前置检查
# ============================================================================

phase_validate() {
  echo ""
  log_info "检查 Session 状态..."
  echo ""

  # 检查 1：是否在 Rust 项目中
  if [[ ! -f "Cargo.toml" ]]; then
    log_error "未找到 Cargo.toml，不在 Rust 项目目录中"
    exit 1
  fi

  if ! command -v cargo &> /dev/null; then
    log_error "未找到 cargo 命令，请检查 Rust 安装"
    exit 1
  fi

  # 检查 2：工作区是否健康
  if ! git diff --check > /dev/null 2>&1; then
    log_error "工作区存在冲突或其他问题，请先处理"
    exit 1
  fi

  # 检查 3：获取工作区改动信息（可能为空）
  CHANGES=$(git status --porcelain 2>/dev/null || echo "")

  log_success "前置检查通过"
}

# ============================================================================
# Phase 2: 执行操作
# ============================================================================

phase_execute() {
  echo ""
  log_info "运行质量检查..."
  echo ""

  local fmt_success=false
  local clippy_success=false
  local test_success=false
  local build_success=false
  local test_count=0

  # 检查 1：cargo fmt
  if cargo fmt --check > /dev/null 2>&1; then
    fmt_success=true
  fi

  # 检查 2：cargo clippy
  if cargo clippy -- -D warnings > /dev/null 2>&1; then
    clippy_success=true
  fi

  # 检查 3：cargo test
  if TEST_OUTPUT=$(cargo test 2>&1); then
    test_success=true
    test_count=$(echo "$TEST_OUTPUT" | grep -oP 'test result: ok\. \K\d+' || echo "?")
  fi

  # 检查 4：cargo build
  if cargo build > /dev/null 2>&1; then
    build_success=true
  fi

  # 输出检查结果
  echo "质量检查结果："
  $fmt_success && echo "  - cargo fmt ✓" || echo "  - cargo fmt ✗"
  $clippy_success && echo "  - cargo clippy ✓" || echo "  - cargo clippy ✗"
  echo "  - cargo test ✓ ($test_count 个测试通过)"
  $build_success && echo "  - cargo build ✓" || echo "  - cargo build ✗"

  # 检查是否全部通过
  if ! ($fmt_success && $clippy_success && $test_success && $build_success); then
    echo ""
    log_error "质量检查失败，无法生成提交信息"
    echo ""
    log_error "错误详情："
    $fmt_success || echo "  - cargo fmt: 代码格式不符合规范"
    $clippy_success || echo "  - cargo clippy: 存在 clippy 警告"
    $test_success || echo "  - cargo test: 测试失败"
    $build_success || echo "  - cargo build: 编译失败"
    echo ""
    log_info "后续操作："
    echo "  1. 修复上述错误"
    echo "  2. 重新调用此 Skill"
    exit 1
  fi

  log_success "质量检查通过"
}

# ============================================================================
# Phase 2 继续：生成提交信息
# ============================================================================

phase_generate_commit_message() {
  echo ""

  # 收集改动信息
  local added=0
  local modified=0
  local deleted=0
  local files=()

  while IFS= read -r line; do
    if [[ -z "$line" ]]; then
      continue
    fi

    local type="${line:0:2}"
    local file="${line:3}"

    case "$type" in
      "A ") ((added++)) ;;
      "D ") ((deleted++)) ;;
      * ) ((modified++)) ;;
    esac

    files+=("$file")
  done < <(git status --porcelain)

  # 决定提交类型
  local commit_type="chore"
  local has_rust=false
  local has_docs=false

  for file in "${files[@]}"; do
    if [[ "$file" == *.rs ]]; then
      has_rust=true
    fi
    if [[ "$file" == *.md ]]; then
      has_docs=true
    fi
  done

  if $has_rust; then
    commit_type="feat"
  elif $has_docs; then
    commit_type="docs"
  fi

  # 生成提交信息
  COMMIT_TITLE="$commit_type: 完成当前 session 任务"

  # 核心功能说明
  COMMIT_BODY="## 核心功能"$'\n'
  local count=0
  for file in "${files[@]}"; do
    if [[ $count -ge 5 ]]; then
      COMMIT_BODY="${COMMIT_BODY}- 及其他 $((${#files[@]} - 5)) 个文件"$'\n'
      break
    fi
    COMMIT_BODY="${COMMIT_BODY}- 修改: $file"$'\n'
    ((count++))
  done

  # 文件变更统计
  COMMIT_BODY="${COMMIT_BODY}"$'\n'"## 文件变更"$'\n'
  COMMIT_BODY="${COMMIT_BODY}新增：$added"$'\n'
  COMMIT_BODY="${COMMIT_BODY}修改：$modified"$'\n'
  COMMIT_BODY="${COMMIT_BODY}删除：$deleted"$'\n'

  # 质量指标
  COMMIT_BODY="${COMMIT_BODY}"$'\n'"## 质量指标"$'\n'
  COMMIT_BODY="${COMMIT_BODY}✓ cargo fmt ✓"$'\n'
  COMMIT_BODY="${COMMIT_BODY}✓ cargo clippy ✓"$'\n'
  COMMIT_BODY="${COMMIT_BODY}✓ cargo test ✓"$'\n'
  COMMIT_BODY="${COMMIT_BODY}✓ cargo build ✓"$'\n'

  FULL_COMMIT_MESSAGE="${COMMIT_TITLE}"$'\n\n'"${COMMIT_BODY}"
}

# ============================================================================
# Phase 2 继续：暂存更改
# ============================================================================

phase_stage_changes() {
  if [[ -n "$CHANGES" ]]; then
    git add .
    log_success "已暂存所有更改"
  fi
}

# ============================================================================
# Phase 3: 输出结果
# ============================================================================

phase_output() {
  echo ""

  # 检查是否有改动
  if [[ -z "$CHANGES" ]]; then
    log_success "工作区干净，没有未提交的更改"
    echo ""
    log_info "当前已完成的 Session："
    git log --oneline -1
    echo ""
    log_info "提示："
    echo "  - 所有质量检查已通过"
    echo "  - 如需开始新的 Session，可继续工作"
    echo "  - 查看更多提交历史：git log --oneline"
    return
  fi

  echo "📝 生成提交信息："
  echo ""
  echo "$FULL_COMMIT_MESSAGE"
  echo ""
  separator
  echo "📝 下一步："
  echo "  1. 检查上面的提交信息是否正确"
  echo "  2. 运行命令提交（编辑器中粘贴提交信息）："
  echo "     git commit"
  echo "  3. 验证提交："
  echo "     git show HEAD"
  separator
}

# ============================================================================
# 主流程
# ============================================================================

main() {
  phase_validate

  # 如果工作区干净，仍然运行质量检查，但不生成提交信息
  if [[ -z "$CHANGES" ]]; then
    echo ""
    log_info "工作区干净，运行质量检查验证..."
    echo ""
    phase_execute
    phase_output
    return
  fi

  # 工作区有改动，完整执行流程
  phase_execute
  phase_generate_commit_message
  phase_stage_changes
  phase_output
}

main
