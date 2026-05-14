"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.config = void 0;
exports.validateConfig = validateConfig;
const fs_1 = require("fs");
const path_1 = require("path");
function requireEnv(name) {
    const value = process.env[name];
    if (!value)
        throw new Error(`Missing required env var: ${name}`);
    return value;
}
function optionalEnv(name, fallback = '') {
    return process.env[name] || fallback;
}
function loadDotEnv() {
    const envPath = (0, path_1.resolve)(__dirname, '../../.env');
    if ((0, fs_1.existsSync)(envPath)) {
        const content = (0, fs_1.readFileSync)(envPath, 'utf-8');
        for (const line of content.split('\n')) {
            const trimmed = line.trim();
            if (!trimmed || trimmed.startsWith('#'))
                continue;
            const eqIdx = trimmed.indexOf('=');
            if (eqIdx === -1)
                continue;
            const key = trimmed.slice(0, eqIdx).trim();
            const val = trimmed.slice(eqIdx + 1).trim();
            if (!process.env[key])
                process.env[key] = val;
        }
    }
}
loadDotEnv();
exports.config = {
    ACCT: requireEnv('ACCT'),
    OPERATOR_HEX: requireEnv('OPERATOR_HEX'),
    BRIDGE_PID: requireEnv('BRIDGE_PID'),
    FLOW_PID: optionalEnv('FLOW_PID'),
    PULSE_PID: optionalEnv('PULSE_PID'),
    STRATEGY_PID: optionalEnv('STRATEGY_PID'),
    NETWORK_PID: optionalEnv('NETWORK_PID'),
    VARA_NETWORK: requireEnv('VARA_NETWORK'),
    VARA_WS: optionalEnv('VARA_WS', requireEnv('VARA_NETWORK')),
    IDL_DIR: optionalEnv('IDL_DIR', './idl'),
    A2A_IDL: optionalEnv('A2A_IDL'),
    COINGECKO_KEY: optionalEnv('COINGECKO_KEY'),
    NEWS_API_KEY: optionalEnv('NEWS_API_KEY'),
    FEED_INTERVAL_MS: 30_000,
    RETRY_DELAY_MS: 30_000,
};
function validateConfig() {
    const required = ['ACCT', 'BRIDGE_PID', 'VARA_NETWORK'];
    const missing = required.filter(k => !process.env[k]);
    if (missing.length > 0)
        throw new Error(`Missing: ${missing.join(', ')}`);
    console.log(`Config OK: ${exports.config.ACCT} on ${exports.config.VARA_NETWORK}`);
}
//# sourceMappingURL=config.js.map