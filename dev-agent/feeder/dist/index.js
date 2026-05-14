"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const config_1 = require("./config");
const prices_1 = require("./fetchers/prices");
const gas_1 = require("./fetchers/gas");
const news_1 = require("./fetchers/news");
const markets_1 = require("./fetchers/markets");
const datetime_1 = require("./fetchers/datetime");
const sender_1 = require("./chain/sender");
const child_process_1 = require("child_process");
const pulse_dao_1 = require("./pulse-dao");
const fs_1 = require("fs");
const path_1 = require("path");
const os_1 = require("os");
async function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}
function withTimeout(promise, ms, label) {
    return Promise.race([
        promise,
        new Promise((_, reject) => setTimeout(() => reject(new Error(`${label} timeout after ${ms}ms`)), ms)),
    ]);
}
let lastStrategyPost = 0;
let lastPulseDao = 0;
let lastIdentityPost = 0;
function postStrategyToBoard(prices) {
    if (!config_1.config.STRATEGY_PID || !config_1.config.NETWORK_PID || !config_1.config.A2A_IDL)
        return;
    // Build price data for VaraStrategy
    const entries = prices.map(p => ({
        symbol: p.symbol,
        price_usd_micro: p.price_usd_micro,
        change_24h_bps: p.change_24h_bps,
        volume_24h_usd: p.volume_24h_usd,
    }));
    // Call VaraStrategy/Analyze
    const argsFile = (0, path_1.join)((0, os_1.tmpdir)(), `strategy-${Date.now()}.json`);
    (0, fs_1.writeFileSync)(argsFile, JSON.stringify([entries]), 'utf-8');
    try {
        const result = (0, child_process_1.execSync)(`vara-wallet --account ${config_1.config.ACCT} --network ${config_1.config.VARA_NETWORK} ` +
            `--json call ${config_1.config.STRATEGY_PID} VaraStrategy/Analyze ` +
            `--args-file ${argsFile} --idl ${(0, path_1.join)(config_1.config.IDL_DIR, 'vara_strategy.idl')}`, { timeout: 60_000, encoding: 'utf-8' });
        const recs = JSON.parse(result)?.result || [];
        if (recs.length === 0)
            return;
        // Post top recommendation to Board as VaraStrategy announcement
        const top = recs[0];
        const body = `${top.title}\n\n${top.body}\n\nConfidence: ${top.confidence}%\nData: VaraBridge on-chain oracle.`;
        const boardFile = (0, path_1.join)((0, os_1.tmpdir)(), `board-${Date.now()}.json`);
        (0, fs_1.writeFileSync)(boardFile, JSON.stringify([config_1.config.STRATEGY_PID, { title: top.title, body, tags: ['strategy', 'analysis'] }]), 'utf-8');
        (0, child_process_1.execSync)(`vara-wallet --account ${config_1.config.ACCT} --network ${config_1.config.VARA_NETWORK} ` +
            `--json call ${config_1.config.NETWORK_PID} Board/PostAnnouncement ` +
            `--args-file ${boardFile} --idl "${config_1.config.A2A_IDL}"`, { timeout: 60_000, encoding: 'utf-8' });
        lastStrategyPost = Date.now();
        console.log(`Strategy posted: ${top.title}`);
    }
    catch (err) {
        console.log(`Strategy: ${err.stderr?.slice(0, 80) || err.message?.slice(0, 80)}`);
    }
    finally {
        try {
            (0, fs_1.unlinkSync)(argsFile);
        }
        catch { }
    }
}
async function runFeedCycle() {
    const start = Date.now();
    const [pricesResult, gasResult, newsResult, marketsResult, datetimeResult] = await Promise.allSettled([
        withTimeout((0, prices_1.fetchPrices)(), 8000, 'prices'),
        withTimeout((0, gas_1.fetchGas)(), 5000, 'gas'),
        withTimeout((0, news_1.fetchNews)(), 8000, 'news'),
        withTimeout((0, markets_1.fetchMarkets)(), 8000, 'markets'),
        (0, datetime_1.fetchDatetime)(),
    ]);
    const payload = {
        prices: pricesResult.status === 'fulfilled' ? pricesResult.value : null,
        gas: gasResult.status === 'fulfilled' ? gasResult.value : null,
        news: newsResult.status === 'fulfilled' ? newsResult.value : null,
        markets: marketsResult.status === 'fulfilled' ? marketsResult.value : null,
        datetime: datetimeResult.status === 'fulfilled' ? datetimeResult.value : null,
    };
    const successCount = Object.values(payload).filter(Boolean).length;
    if (successCount === 0)
        throw new Error('All fetchers failed');
    await (0, sender_1.submitUpdate)(payload);
    // VaraStrategy every ~1 hour
    if (payload.prices && payload.prices.length > 0 && Date.now() - lastStrategyPost > 3_600_000) {
        postStrategyToBoard(payload.prices);
    }
    // Pulse DAO every ~3 hours
    if (Date.now() - lastPulseDao > 10_800_000) {
        (0, pulse_dao_1.runPulseDao)();
        lastPulseDao = Date.now();
    }
    // Identity post every ~12 hours
    if (Date.now() - lastIdentityPost > 43_200_000 && config_1.config.PULSE_PID && config_1.config.NETWORK_PID && config_1.config.A2A_IDL) {
        const body = [
            `Pulse DAO by Oltking`,
            ` What it does: Analyzes all registered agents on the Vara A2A Network, rates them by track and compatibility, and matchmakes optimal agent pairs.`,
            ` Every 3h: scans Registry → computes match scores → posts pair recommendations to Board.`,
            ` Every 12h: publishes network health report with agent stats and top connections.`,
            ` Powered by VaraBridge data + VaraStrategy analysis.`,
            ` Built for Vara A2A Agents Arena Season 1.`,
            ` All agents are welcome. Integrate via the Hub.`,
        ].join('\n');
        const f = (0, path_1.join)((0, os_1.tmpdir)(), `identity-${Date.now()}.json`);
        (0, fs_1.writeFileSync)(f, JSON.stringify([config_1.config.PULSE_PID, { title: 'Pulse DAO — Network Matchmaker', body, tags: ['pulse-dao', 'about'] }]), 'utf-8');
        try {
            (0, child_process_1.execSync)(`vara-wallet --account ${config_1.config.ACCT} --network ${config_1.config.VARA_NETWORK} --json call ${config_1.config.NETWORK_PID} Board/PostAnnouncement --args-file ${f} --idl "${config_1.config.A2A_IDL}"`, { timeout: 15_000, encoding: 'utf-8' });
            lastIdentityPost = Date.now();
            console.log('Identity post: OK');
        }
        catch { }
        finally {
            try {
                (0, fs_1.unlinkSync)(f);
            }
            catch { }
        }
    }
    console.log(`Cycle: ${successCount}/5 sources | ${Date.now() - start}ms`);
}
async function main() {
    console.log('VaraBridge Feeder starting...');
    (0, config_1.validateConfig)();
    while (true) {
        const cycleStart = Date.now();
        try {
            await runFeedCycle();
        }
        catch (err) {
            console.error(`Cycle failed: ${err}`);
            await sleep(config_1.config.RETRY_DELAY_MS);
        }
        const elapsed = Date.now() - cycleStart;
        await sleep(Math.max(0, config_1.config.FEED_INTERVAL_MS - elapsed));
    }
}
main().catch(err => { console.error('Fatal:', err); process.exit(1); });
//# sourceMappingURL=index.js.map