import { execSync } from 'child_process';
import { writeFileSync, unlinkSync } from 'fs';
import { join } from 'path';
import { tmpdir } from 'os';

import { config } from '../config';

interface FullUpdatePayload {
    prices: any[] | null;
    gas: any | null;
    news: any[] | null;
    markets: any[] | null;
    datetime: any | null;
}

function writeTempJson(data: any): string {
    const filePath = join(tmpdir(), `bridge-update-${Date.now()}.json`);
    writeFileSync(filePath, JSON.stringify([data]), 'utf-8');
    return filePath;
}

export async function submitUpdate(payload: FullUpdatePayload): Promise<void> {
    const argsFile = writeTempJson(payload);

    try {
        const cmd = [
            'vara-wallet',
            '--account', config.ACCT,
            '--network', config.VARA_NETWORK,
            '--json', 'call', config.BRIDGE_PID,
            'VaraBridge/UpdateAll',
            '--args-file', argsFile,
            '--idl', join(config.IDL_DIR, 'vara_bridge.idl'),
        ].join(' ');

        const result = execSync(cmd, { timeout: 60_000, encoding: 'utf-8' });

        const res = JSON.parse(result as string);
        if (!res.txHash) {
            throw new Error(`No txHash: ${JSON.stringify(res)}`);
        }

        console.log(`tx: ${res.txHash} | block: ${res.blockNumber}`);
    } finally {
        try {
            unlinkSync(argsFile);
        } catch {
            // ignore cleanup errors
        }
    }
}
