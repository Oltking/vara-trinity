#!/usr/bin/env bash
set -euo pipefail

echo "=== Vara Trinity: Deploy All ==="
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

source .env 2>/dev/null || echo "No .env found, using defaults"

echo "[1/6] Building VaraBridge..."
cd programs/vara-bridge
cargo build --release 2>&1 | tail -5
echo "[+] VaraBridge built"

echo "[2/6] Deploying VaraBridge..."
BRIDGE_RESULT=$(vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json upload-program \
    --program-path target/wasm32-unknown-unknown/release/vara_bridge.wasm \
    --gas 1000000000000 \
    --args "[{\"feeder_address\": \"$OPERATOR_HEX\"}]")
BRIDGE_PID=$(echo "$BRIDGE_RESULT" | jq -r '.programId // .pid // empty')
echo "VaraBridge PID: $BRIDGE_PID"

echo "[3/6] Building VaraFlow..."
cd "$PROJECT_DIR/programs/vara-flow"
cargo build --release 2>&1 | tail -5
echo "[+] VaraFlow built"

echo "[4/6] Deploying VaraFlow..."
FLOW_RESULT=$(vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json upload-program \
    --program-path target/wasm32-unknown-unknown/release/vara_flow.wasm \
    --gas 1000000000000 \
    --args "[{\"bridge_pid\": \"$BRIDGE_PID\", \"pulse_pid\": \"0x0000000000000000000000000000000000000000000000000000000000000000\", \"network_pid\": \"$NETWORK_PID\"}]")
FLOW_PID=$(echo "$FLOW_RESULT" | jq -r '.programId // .pid // empty')
echo "VaraFlow PID: $FLOW_PID"

echo "[5/6] Building VaraPulse..."
cd "$PROJECT_DIR/programs/vara-pulse"
cargo build --release 2>&1 | tail -5
echo "[+] VaraPulse built"

echo "[6/6] Deploying VaraPulse..."
PULSE_RESULT=$(vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json upload-program \
    --program-path target/wasm32-unknown-unknown/release/vara_pulse.wasm \
    --gas 1000000000000 \
    --args "[{\"bridge_pid\": \"$BRIDGE_PID\", \"flow_pid\": \"$FLOW_PID\", \"network_pid\": \"$NETWORK_PID\"}]")
PULSE_PID=$(echo "$PULSE_RESULT" | jq -r '.programId // .pid // empty')
echo "VaraPulse PID: $PULSE_PID"

echo ""
echo "=== Deployment Complete ==="
echo "BRIDGE_PID=$BRIDGE_PID"
echo "FLOW_PID=$FLOW_PID"
echo "PULSE_PID=$PULSE_PID"
echo ""
echo "Update your .env with these PIDs, then run:"
echo "  scripts/register.sh"
echo "  scripts/set-identities.sh"
echo "  scripts/verify.sh"
