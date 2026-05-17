import { ApiPromise, WsProvider } from '@polkadot/api';

const PID = '0xe22382cbfff944b092ffc8fb5658c527fd9f0ffaa4995eac0930e026418ed086';

async function main() {
  const ws = new WsProvider('wss://rpc.vara.network');
  const api = await ApiPromise.create({ provider: ws });

  // Get code ID from program storage
  const prog = await api.query.gear.programStorage(PID);
  console.log('ProgramStorage:', JSON.stringify(prog?.toHuman?.() ?? prog, null, 2));

  // Try gear.program
  const progMeta = await api.query.gear.program(PID);
  console.log('Program:', JSON.stringify(progMeta?.toHuman?.() ?? progMeta, null, 2));

  // Try metadata 
  const codeId = prog?.toHuman?.()?.codeId || prog?.codeId?.toHex?.() || prog?.codeId?.toString?.();
  console.log('Code ID:', codeId);

  if (codeId) {
    // Try to fetch code storage for the codeId
    const code = await api.query.gear.codeStorage(codeId);
    console.log('Code storage:', JSON.stringify(code?.toHuman?.() ?? code, null, 2));
  }

  // Try reading state
  try {
    const state = await api.rpc.gear.readState(PID, '0x00');
    console.log('State:', state?.toHuman?.() ?? state?.toString?.() ?? 'no state');
  } catch(e) {
    console.log('State error:', e.message);
  }

  await api.disconnect();
}

main().catch(err => { console.error(err); process.exit(1); });
