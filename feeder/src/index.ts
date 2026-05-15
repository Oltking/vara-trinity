import { config, validateConfig } from './config';
import { fetchPrices, PriceFeed } from './fetchers/prices';
import { fetchGas, GasFeed } from './fetchers/gas';
import { fetchNews, NewsSummary } from './fetchers/news';
import { fetchMarkets, MarketFeed } from './fetchers/markets';
import { fetchDatetime, DatetimeFeed } from './fetchers/datetime';
import { submitUpdate } from './chain/sender';
import { execSync } from 'child_process';
import { runPulseDao } from './pulse-dao';
import { runSwapCycle } from './swap';
let lastFlowTick = 0;
import { writeFileSync, unlinkSync } from 'fs';
import { join } from 'path';
import { tmpdir } from 'os';

interface FullUpdatePayload {
    prices: PriceFeed[] | null;
    gas: GasFeed | null;
    news: NewsSummary[] | null;
    markets: MarketFeed[] | null;
    datetime: DatetimeFeed | null;
}

async function sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
}

function withTimeout<T>(promise: Promise<T>, ms: number, label: string): Promise<T> {
    return Promise.race([
        promise,
        new Promise<never>((_, reject) =>
            setTimeout(() => reject(new Error(`${label} timeout after ${ms}ms`)), ms)
        ),
    ]);
}

let lastStrategyPost = 0; // fire on first eligible cycle
let lastPulseDao = 0;
let lastIdentityPost = 0;
let lastSwapPost = 0;

function postStrategyToBoard(prices: PriceFeed[]): void {
    if (!config.STRATEGY_PID || !config.NETWORK_PID || !config.A2A_IDL) return;

    // Build price data for VaraStrategy
    const entries = prices.map(p => ({
        symbol: p.symbol,
        price_usd_micro: p.price_usd_micro,
        change_24h_bps: p.change_24h_bps,
        volume_24h_usd: p.volume_24h_usd,
    }));

    // Call VaraStrategy/Analyze
    const argsFile = join(tmpdir(), `strategy-${Date.now()}.json`);
    writeFileSync(argsFile, JSON.stringify([entries]), 'utf-8');

    try {
        const result = execSync(
            `vara-wallet --account ${config.ACCT} --network ${config.VARA_NETWORK} ` +
            `--json call ${config.STRATEGY_PID} VaraStrategy/Analyze ` +
            `--args-file ${argsFile} --idl ${join(config.IDL_DIR, 'vara_strategy.idl')}`,
            { timeout: 60_000, encoding: 'utf-8' }
        );
        const recs = JSON.parse(result as string)?.result || [];
        if (recs.length === 0) return;

        // Post top recommendation to Board as VaraStrategy announcement
        const top = recs[0];
        const body = `${top.title}\n\n${top.body}\n\nConfidence: ${top.confidence}%\nData: VaraBridge on-chain oracle.`;
        const boardFile = join(tmpdir(), `board-${Date.now()}.json`);
        writeFileSync(boardFile, JSON.stringify([config.STRATEGY_PID, { title: top.title, body, tags: ['strategy', 'analysis'] }]), 'utf-8');

        execSync(
            `vara-wallet --account ${config.ACCT} --network ${config.VARA_NETWORK} ` +
            `--json call ${config.NETWORK_PID} Board/PostAnnouncement ` +
            `--args-file ${boardFile} --idl "${config.A2A_IDL}"`,
            { timeout: 60_000, encoding: 'utf-8' }
        );

        lastStrategyPost = Date.now();
        console.log(`Strategy posted: ${top.title}`);
    } catch (err: any) {
        console.log(`Strategy: ${err.stderr?.slice(0,80) || err.message?.slice(0,80)}`);
    } finally {
        try { unlinkSync(argsFile); } catch {}
    }
}

async function runFeedCycle(): Promise<void> {
    const start = Date.now();

    const [pricesResult, gasResult, newsResult, marketsResult, datetimeResult] =
        await Promise.allSettled([
            withTimeout(fetchPrices(), 8000, 'prices'),
            withTimeout(fetchGas(), 5000, 'gas'),
            withTimeout(fetchNews(), 8000, 'news'),
            withTimeout(fetchMarkets(), 8000, 'markets'),
            fetchDatetime(),
        ]);

    const payload: FullUpdatePayload = {
        prices: pricesResult.status === 'fulfilled' ? pricesResult.value : null,
        gas: gasResult.status === 'fulfilled' ? gasResult.value : null,
        news: newsResult.status === 'fulfilled' ? newsResult.value : null,
        markets: marketsResult.status === 'fulfilled' ? marketsResult.value : null,
        datetime: datetimeResult.status === 'fulfilled' ? datetimeResult.value : null,
    };

    const successCount = Object.values(payload).filter(Boolean).length;
    if (successCount === 0) throw new Error('All fetchers failed');

    await submitUpdate(payload);

    // VaraFlow/Tick every ~75s (≈50 blocks) — cross-program call
    if (config.FLOW_PID && Date.now() - lastFlowTick > 75_000) {
        const f = join(tmpdir(), `tick-${Date.now()}.json`);
        writeFileSync(f, JSON.stringify([]), 'utf-8');
        try {
            execSync(`vara-wallet --account ${config.ACCT} --network ${config.VARA_NETWORK} --json call ${config.FLOW_PID} VaraFlow/Tick --args-file ${f} --idl ${join(config.IDL_DIR, 'vara_flow.idl')}`, { timeout: 30_000, encoding: 'utf-8', stdio: 'pipe' });
            lastFlowTick = Date.now();
        } catch {} finally { try { unlinkSync(f); } catch {} }
    }

    // Swap report every ~2 hours
    if (Date.now() - lastSwapPost > 7_200_000) {
        await runSwapCycle();
        lastSwapPost = Date.now();
    }

    // VaraStrategy every ~2 hours (disabled — new programs not registered yet)
    // if (payload.prices && payload.prices.length > 0 && Date.now() - lastStrategyPost > 7_200_000) {
    //     postStrategyToBoard(payload.prices);
    // }

    // Pulse DAO every ~3 hours
    if (Date.now() - lastPulseDao > 10_800_000) {
        runPulseDao();
        lastPulseDao = Date.now();
    }

    // Identity post every ~12 hours
    if (Date.now() - lastIdentityPost > 43_200_000 && config.PULSE_PID && config.NETWORK_PID && config.A2A_IDL) {
        const body = [
            `Pulse DAO by Oltking`,
            ` What it does: Analyzes all registered agents on the Vara A2A Network, rates them by track and compatibility, and matchmakes optimal agent pairs.`,
            ` Every 3h: scans Registry → computes match scores → posts pair recommendations to Board.`,
            ` Every 12h: publishes network health report with agent stats and top connections.`,
            ` Powered by VaraBridge data + VaraStrategy analysis.`,
            ` Built for Vara A2A Agents Arena Season 1.`,
            ` All agents are welcome. Integrate via the Hub.`,
        ].join('\n');
        const f = join(tmpdir(), `identity-${Date.now()}.json`);
        writeFileSync(f, JSON.stringify([config.PULSE_PID, { title: 'Pulse DAO — Network Matchmaker', body, tags: ['pulse-dao', 'about'] }]), 'utf-8');
        try { execSync(`vara-wallet --account ${config.ACCT} --network ${config.VARA_NETWORK} --json call ${config.NETWORK_PID} Board/PostAnnouncement --args-file ${f} --idl "${config.A2A_IDL}"`, { timeout: 15_000, encoding: 'utf-8' }); lastIdentityPost = Date.now(); console.log('Identity post: OK'); } catch {} finally { try { unlinkSync(f); } catch {} }
    }

    console.log(`Cycle: ${successCount}/5 sources | ${Date.now() - start}ms`);
}

async function main(): Promise<void> {
    console.log('VaraBridge Feeder starting...');
    validateConfig();

    while (true) {
        const cycleStart = Date.now();
        try {
            await runFeedCycle();
        } catch (err) {
            console.error(`Cycle failed: ${err}`);
            await sleep(config.RETRY_DELAY_MS);
        }
        const elapsed = Date.now() - cycleStart;
        await sleep(Math.max(0, config.FEED_INTERVAL_MS - elapsed));
    }
}

main().catch(err => { console.error('Fatal:', err); process.exit(1); });
