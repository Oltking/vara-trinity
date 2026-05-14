import { config } from '../config';

let nonce: number | null = null;
let pending = false;

async function sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function fetchNonceFromChain(): Promise<number> {
    // TODO: fetch actual nonce from Vara chain via RPC
    return 0;
}

export class NonceManager {
    async acquire(): Promise<number> {
        while (pending) await sleep(50);
        pending = true;
        if (nonce === null) {
            nonce = await fetchNonceFromChain();
        }
        const n = nonce!;
        nonce!++;
        pending = false;
        return n;
    }

    reset(): void {
        nonce = null;
    }
}

export const nonceManager = new NonceManager();
