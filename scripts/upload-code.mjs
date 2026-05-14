import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { readFileSync } from 'fs';

const WALLET_PATH = process.env.HOME + '/.vara-wallet/wallets/vara-trinity-operator.json';

async function main() {
  const api = await ApiPromise.create({ provider: new WsProvider('wss://rpc.vara.network') });
  const wallet = JSON.parse(readFileSync(WALLET_PATH, 'utf-8'));
  const keyring = new Keyring({ type: 'sr25519' });
  keyring.addFromJson(wallet, '');
  const pair = keyring.getPair(wallet.address);
  pair.unlock('');

  const wasm = readFileSync('target/wasm32-unknown-unknown/release/vara_bridge.wasm');
  console.log('Uploading code, size:', wasm.length);
  const tx = api.tx.gear.uploadCode(wasm);

  const unsub = await tx.signAndSend(pair, ({ events = [], status }) => {
    console.log('Status:', status.type);
    if (status.isInBlock) {
      events.forEach(({ event }) => {
        if (event.method === 'ExtrinsicFailed') {
          const err = event.data[0];
          if (err.isModule) {
            const { name, docs } = api.registry.findMetaError(err.asModule);
            console.log('FAILED:', name, docs.join(' '));
          } else {
            console.log('FAILED:', err.toString());
          }
        } else if (event.section === 'gear') {
          console.log('GEAR:', event.method, event.data.toString());
        }
      });
      unsub();
      process.exit(0);
    }
  });
}

main().catch(e => { console.error('ERR:', e.message); process.exit(1); });
