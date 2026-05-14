"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.nonceManager = exports.NonceManager = void 0;
let nonce = null;
let pending = false;
async function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}
async function fetchNonceFromChain() {
    // TODO: fetch actual nonce from Vara chain via RPC
    return 0;
}
class NonceManager {
    async acquire() {
        while (pending)
            await sleep(50);
        pending = true;
        if (nonce === null) {
            nonce = await fetchNonceFromChain();
        }
        const n = nonce;
        nonce++;
        pending = false;
        return n;
    }
    reset() {
        nonce = null;
    }
}
exports.NonceManager = NonceManager;
exports.nonceManager = new NonceManager();
//# sourceMappingURL=nonce.js.map