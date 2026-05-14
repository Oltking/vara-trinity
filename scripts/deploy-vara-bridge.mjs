import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { readFileSync } from 'fs';
import { resolve } from 'path';

const WASM_PATH = resolve(process.cwd(), 'target/wasm32-unknown-unknown/release/vara_bridge.wasm');
const WALLET_PATH = resolve(process.env.HOME, '.vara-wallet/wallets/vara-trinity-operator.json');

async function main() {
  const wasmBytes = readFileSync(WASM_PATH);
  const walletJson = JSON.parse(readFileSync(WALLET_PATH, 'utf-8'));

  const provider = new WsProvider('wss://rpc.vara.network');
  const api = await ApiPromise.create({ provider });

  const keyring = new Keyring({ type: 'sr25519' });
  keyring.addFromJson(walletJson, '');
  const pair = keyring.getPair(walletJson.address);
  pair.unlock('');

  const feeder = '0x7c5ab61c9152010551f1f7f57040a221c71da0aef93c2144e601bf9ccece8067';
  const networkPid = '0x19f27f4c906a5ac230be82d907850d44c7a7fff1b4c6903f62e78e09e0b353f3';

  const init_payload = '0x0c' + '4e6577' + feeder.slice(2) + networkPid.slice(2);

  console.log('WASM:', wasmBytes.length, 'bytes');
  console.log('Account:', pair.address);

  const salt = '0x' + Date.now().toString(16);
  const gasLimit = 200_000_000_000;
  const value = 1_000_000_000_000; // 1 VARA to the program

  const tx = api.tx.gear.uploadProgram(wasmBytes, salt, init_payload, gasLimit, value, false);

  console.log('Submitting...');
  const unsub = await tx.signAndSend(pair, ({ events = [], status, txHash }) => {
    console.log('Status:', status.type);
    if (status.isInBlock || status.isFinalized) {
      console.log('Block:', (status.isInBlock ? status.asInBlock : status.asFinalized).toHex());
      console.log('TxHash:', txHash.toHex());
      events.forEach(({ phase, event: { section, method, data } }) => {
        console.log(`  ${section}.${method}: ${data}`);
        if (method === 'ProgramChanged') {
          console.log('✅ Program ID from event:', data.toString());
        }
      });
      unsub();
    }
  });
}

main().catch(err => {
  console.error('FAILED:', err.message);
  process.exit(1);
});
