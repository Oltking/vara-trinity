"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.discoverAgents = discoverAgents;
const child_process_1 = require("child_process");
const config_1 = require("./config");
let cachedAgents = [];
let lastFetch = 0;
const CACHE_TTL = 3_600_000; // 1 hour
function discoverAgents() {
    if (!config_1.config.NETWORK_PID || !config_1.config.A2A_IDL)
        return cachedAgents;
    const now = Date.now();
    if (cachedAgents.length > 0 && now - lastFetch < CACHE_TTL) {
        return cachedAgents;
    }
    try {
        const result = (0, child_process_1.execSync)(`vara-wallet --account ${config_1.config.ACCT} --network ${config_1.config.VARA_NETWORK} ` +
            `--json call ${config_1.config.NETWORK_PID} Registry/Discover ` +
            `--args '[{"include": null}, null, 50]' --idl "${config_1.config.A2A_IDL}"`, { timeout: 15_000, encoding: 'utf-8' });
        const parsed = JSON.parse(result);
        const items = parsed?.result?.items || [];
        // Filter out our own programs and only keep others
        const ourPids = new Set([
            config_1.config.BRIDGE_PID,
            config_1.config.FLOW_PID,
            config_1.config.PULSE_PID,
            config_1.config.STRATEGY_PID,
            config_1.config.OPERATOR_HEX,
        ].filter(Boolean));
        cachedAgents = items
            .filter((app) => !ourPids.has(app.program_id))
            .map((app) => ({
            program_id: app.program_id,
            handle: app.handle || 'unknown',
            description: (app.description || '').slice(0, 100),
            track: app.track?.kind || 'Unknown',
        }));
        lastFetch = now;
        console.log(`Discovered ${cachedAgents.length} agents on network`);
    }
    catch {
        // return cached or empty
    }
    return cachedAgents;
}
//# sourceMappingURL=discovery.js.map