import { ApiPromise, WsProvider } from '@polkadot/api';

const PROGRAM_ID = '0xe22382cbfff944b092ffc8fb5658c527fd9f0ffaa4995eac0930e026418ed086';

async function main() {
  const ws = new WsProvider('wss://rpc.vara.network');
  const api = await ApiPromise.create({ provider: ws });

  // Get program details to find codeId
  const codeId = await api.query.gear.programStorage(PROGRAM_ID);
  console.log('Code ID (raw):', codeId?.toHuman?.() || codeId?.toString?.() || 'unknown');

  // Try code storage
  const codeStorage = await api.query.gear.codeStorage(PROGRAM_ID);
  console.log('Code storage (raw):', codeStorage?.toHuman?.() || codeStorage?.toString?.() || 'N/A');

  // Try to get the code
  try {
    // Using programId as the key
    const status = await api.query.gear.programStorage(PROGRAM_ID);
    console.log('Program storage:', JSON.stringify(status?.toHuman?.() ?? status, null, 2));
  } catch (e) {
    console.log('Program storage error:', e.message);
  }

  // Get program info
  try {
    const info = await api.query.gear.program(PROGRAM_ID);
    console.log('Program info:', JSON.stringify(info?.toHuman?.() ?? info, null, 2));
  } catch (e) {
    console.log('Program info error:', e.message);
  }

  await api.disconnect();
}

main().catch(err => { console.error(err); process.exit(1); });
