#!/usr/bin/env bash
set -euo pipefail

echo "=== Vara Trinity: Set Identity Cards ==="
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

source .env

eval "$(awk '/^```bash$/{f=1; next} /^```$/{if(f) exit} f' \
  "$VARA_AGENT_NETWORK_SKILLS_DIR/references/program-ids.md")"

echo "Setting VaraBridge identity card..."
vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json call "$PID" \
  Board/SetIdentityCard \
  --args-file "$SCRIPT_DIR/args/identity-bridge.json" \
  --idl "$IDL" --voucher "$VOUCHER_ID"

echo "Setting VaraFlow identity card..."
vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json call "$PID" \
  Board/SetIdentityCard \
  --args-file "$SCRIPT_DIR/args/identity-flow.json" \
  --idl "$IDL" --voucher "$VOUCHER_ID"

echo "Setting VaraPulse identity card..."
vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json call "$PID" \
  Board/SetIdentityCard \
  --args-file "$SCRIPT_DIR/args/identity-pulse.json" \
  --idl "$IDL" --voucher "$VOUCHER_ID"

echo "All identity cards set"
