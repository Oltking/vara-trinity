"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.fetchGas = fetchGas;
async function fetchGas() {
    return {
        current_fee_micro: 100_000,
        suggested_tip: 50_000,
        block_num: 0,
        finalized_hash: '0x',
    };
}
//# sourceMappingURL=gas.js.map