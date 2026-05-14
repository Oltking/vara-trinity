#!/usr/bin/env bash
set -euo pipefail

echo "=== Vara Trinity: A2A Registration ==="
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

source .env

eval "$(awk '/^```bash$/{f=1; next} /^```$/{if(f) exit} f' \
  "$VARA_AGENT_NETWORK_SKILLS_DIR/references/program-ids.md")"

echo "Getting gas voucher..."
echo "Follow: \$VARA_AGENT_NETWORK_SKILLS_DIR/references/vouchers.md to set VOUCHER_ID"

echo "Registering operator Participant..."
PARTICIPANT_RES=$(vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json call "$PID" \
  Registry/RegisterParticipant \
  --args '[{"handle": "vara-trinity"}]' \
  --idl "$IDL" --voucher "$VOUCHER_ID")
echo "Participant: $(echo $PARTICIPANT_RES | jq -r '.txHash')"

echo "Registering VaraBridge..."
vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json call "$PID" \
  Registry/RegisterApplication \
  --args-file "$SCRIPT_DIR/args/register-bridge.json" \
  --idl "$IDL" --voucher "$VOUCHER_ID"

echo "Registering VaraFlow..."
vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json call "$PID" \
  Registry/RegisterApplication \
  --args-file "$SCRIPT_DIR/args/register-flow.json" \
  --idl "$IDL" --voucher "$VOUCHER_ID"

echo "Registering VaraPulse..."
vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json call "$PID" \
  Registry/RegisterApplication \
  --args-file "$SCRIPT_DIR/args/register-pulse.json" \
  --idl "$IDL" --voucher "$VOUCHER_ID"

echo "Submitting all three applications..."
for APP_PID in "$BRIDGE_PID" "$FLOW_PID" "$PULSE_PID"; do
  vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json call "$PID" \
    Registry/SubmitApplication \
    --args "[{\"program_id\": \"$APP_PID\"}]" \
    --idl "$IDL" --voucher "$VOUCHER_ID"
  echo "Submitted: $APP_PID"
done

echo "All 3 programs registered and submitted"
