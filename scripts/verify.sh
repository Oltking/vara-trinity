#!/usr/bin/env bash
set -euo pipefail

echo "=== Vara Trinity: Verification ==="
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

source .env

INDEXER_URL="${INDEXER_GRAPHQL_URL:-https://agents-api.vara.network/graphql}"

echo "Indexer: $INDEXER_URL"
echo ""

echo "Checking VaraBridge ($BRIDGE_PID)..."
curl -s -X POST "$INDEXER_URL" \
  -H "Content-Type: application/json" \
  -d "{\"query\": \"{ program(id: \\\"$BRIDGE_PID\\\") { id handle state } }\"}" \
  | jq '.data.program // {error: "not found"}'

echo ""
echo "Checking VaraFlow ($FLOW_PID)..."
curl -s -X POST "$INDEXER_URL" \
  -H "Content-Type: application/json" \
  -d "{\"query\": \"{ program(id: \\\"$FLOW_PID\\\") { id handle state } }\"}" \
  | jq '.data.program // {error: "not found"}'

echo ""
echo "Checking VaraPulse ($PULSE_PID)..."
curl -s -X POST "$INDEXER_URL" \
  -H "Content-Type: application/json" \
  -d "{\"query\": \"{ program(id: \\\"$PULSE_PID\\\") { id handle state } }\"}" \
  | jq '.data.program // {error: "not found"}'

echo ""
echo "All checks complete"
