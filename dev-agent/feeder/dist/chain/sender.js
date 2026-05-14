"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.submitUpdate = submitUpdate;
const child_process_1 = require("child_process");
const fs_1 = require("fs");
const path_1 = require("path");
const os_1 = require("os");
const config_1 = require("../config");
function writeTempJson(data) {
    const filePath = (0, path_1.join)((0, os_1.tmpdir)(), `bridge-update-${Date.now()}.json`);
    (0, fs_1.writeFileSync)(filePath, JSON.stringify([data]), 'utf-8');
    return filePath;
}
async function submitUpdate(payload) {
    const argsFile = writeTempJson(payload);
    try {
        const cmd = [
            'vara-wallet',
            '--account', config_1.config.ACCT,
            '--network', config_1.config.VARA_NETWORK,
            '--json', 'call', config_1.config.BRIDGE_PID,
            'VaraBridge/UpdateAll',
            '--args-file', argsFile,
            '--idl', (0, path_1.join)(config_1.config.IDL_DIR, 'vara_bridge.idl'),
        ].join(' ');
        const result = (0, child_process_1.execSync)(cmd, { timeout: 60_000, encoding: 'utf-8' });
        const res = JSON.parse(result);
        if (!res.txHash) {
            throw new Error(`No txHash: ${JSON.stringify(res)}`);
        }
        console.log(`tx: ${res.txHash} | block: ${res.blockNumber}`);
    }
    finally {
        try {
            (0, fs_1.unlinkSync)(argsFile);
        }
        catch {
            // ignore cleanup errors
        }
    }
}
//# sourceMappingURL=sender.js.map