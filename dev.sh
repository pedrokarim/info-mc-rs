#!/usr/bin/env bash
# Launch API + Web + Storybook in a tmux session with split panes.
# Usage: ./dev.sh
# Stop:  tmux kill-session -t mcinfo

SESSION="mcinfo"
ROOT="$(cd "$(dirname "$0")" && pwd)"
WEB="$ROOT/apps/web"

# Kill existing session if any
tmux kill-session -t "$SESSION" >/dev/null 2>&1 || true

# Helper: run command, keep pane open on exit
run() {
  echo "$1"
  shift
  eval "$@"
  echo ""
  echo "⛔ Process exited. Press ENTER to close."
  read
}
export -f run

# Create session with first pane: API
tmux new-session -d -s "$SESSION" -n dev -c "$ROOT" \
  "bash -c 'run \"🔧 API (port 3001)\" cargo run --bin mc-api'"

# Split horizontally: Web (Vite dev)
tmux split-window -h -t "$SESSION:dev" -c "$WEB" \
  "bash -c 'run \"🌐 Web (Vite dev)\" bun run dev'"

# Split the right pane vertically: Storybook
tmux split-window -v -t "$SESSION:dev.1" -c "$WEB" \
  "bash -c 'run \"📖 Storybook (port 6006)\" bunx storybook dev -p 6006 --no-open'"

# Even out the layout
tmux select-layout -t "$SESSION:dev" main-vertical

# Attach
tmux attach -t "$SESSION"
