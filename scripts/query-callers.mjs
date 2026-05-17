import { ApiPromise, WsProvider } from '@polkadot/api';
import { options } from '@polkadot/api-augment';

const PROGRAMS = {
  Bridge: '0xfb7ed5a79dc2ff15283a524a4489321b5e1f6341db2b9892be83b9568cc1fcb4',
  Flow: '0x19d4b1778cfdf64c732e10640ccff923c4137a7fbed4f1a291e241d3e6361175',
  Pulse: '0xc7b3ef94a77646110671a2c03e6c508ae982dec9db4f29a980e4dec04c0a10e5',
  Strategy: '0xe6483fe2fc8fea2dc3e2ee848e0372b9b486e023bb4cb21247a914e8f074aaa7',
};

const OUR_ACCT = '0xb21266daf5b67e4e7a2f2c782f57e14b39c9145d2574637340f08a68f689f8f6d';

const pidSet = new Set(Object.values(PROGRAMS));

async function main() {
  const ws = new WsProvider('wss://rpc.vara.network');
  const api = await ApiPromise.create({ provider: ws });
  const lastBlock = (await api.rpc.chain.getHeader()).number.toNumber();
  const startBlock = Math.max(lastBlock - 14400, 1); // ~24h if 6s block time

  console.log(`Scanning blocks ${startBlock}..${lastBlock} (${lastBlock - startBlock + 1} blocks)`);

  const callers = {};
  for (const name of Object.keys(PROGRAMS)) callers[name] = {};

  let total = 0;
  const BATCH = 100;
  for (let b = startBlock; b <= lastBlock; b += BATCH) {
    const end = Math.min(b + BATCH - 1, lastBlock);
    const blockHashes = await Promise.all(
      Array.from({ length: end - b + 1 }, (_, i) => api.rpc.chain.getBlockHash(b + i))
    );
    const events = await Promise.all(
      blockHashes.map(hash => api.query.system.events.at(hash))
    );
    for (let i = 0; i < events.length; i++) {
      for (const ev of events[i]) {
        const e = ev.event;
        if (e.section === 'gear' && e.method === 'MessageQueued') {
          const dest = e.data.destination?.toHex?.()?.toLowerCase();
          if (dest && pidSet.has(dest)) {
            const source = e.data.source?.toHex?.()?.toLowerCase() || 'unknown';
            const name = Object.entries(PROGRAMS).find(([,p]) => p === dest)?.[0] || dest;
            if (!callers[name][source]) callers[name][source] = 0;
            callers[name][source]++;
            total++;
          }
        }
      }
    }
  }

  console.log(`\n=== Caller Stats (last ~24h, ${total} total msgs) ===\n`);
  for (const [name, srcs] of Object.entries(callers)) {
    const entries = Object.entries(srcs).sort((a,b) => b[1] - a[1]);
    if (entries.length === 0) {
      console.log(`${name}: no messages`);
      continue;
    }
    console.log(`${name} (${entries.reduce((s, [,c]) => s + c, 0)} messages):`);
    for (const [src, count] of entries) {
      const isUs = src.includes(OUR_ACCT.toLowerCase().slice(2, 10));
      console.log(`  ${src.slice(0, 20)}... ${count}x${isUs ? ' (us/feeder)' : ''}`);
    }
    console.log('');
  }

  await api.disconnect();
}

main().catch(err => { console.error(err); process.exit(1); });
