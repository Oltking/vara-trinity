#!/usr/bin/env bash
# Start the Vara Trinity feeder in the background
# Run this after WSL starts or system reboot

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR" || exit 1

# Kill existing feeder if running
pkill -f "node feeder/dist/index.js" 2>/dev/null

# Start feeder
nohup node feeder/dist/index.js > feeder.log 2>&1 &
echo "Feeder started (PID: $!)"
echo "Logs: $PROJECT_DIR/feeder.log"
echo "View: tail -f $PROJECT_DIR/feeder.log"
