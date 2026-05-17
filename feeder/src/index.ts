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
            `--args-file ${boardFile} --idl "${config.A2A_IDL}"` +
            (config.VOUCHER_ID ? ` --voucher ${config.VOUCHER_ID}` : ''),
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

    // Flow Tick every ~75s
    if (config.FLOW_PID && Date.now() - lastFlowTick > 75_000) {
        const f = join(tmpdir(), `tick-${Date.now()}.json`);
        writeFileSync(f, JSON.stringify([]), 'utf-8');
        try {
            execSync(`vara-wallet --account ${config.ACCT} --network ${config.VARA_NETWORK} --json call ${config.FLOW_PID} VaraFlow/Tick --args-file ${f} --idl ${join(config.IDL_DIR, 'vara_flow.idl')}`, { timeout: 30_000, encoding: 'utf-8', stdio: 'pipe' });
            lastFlowTick = Date.now();
        } catch {} finally { try { unlinkSync(f); } catch {} }
    }

    // Post consolidated VaraBridge summary to Chat every cycle
    if (config.NETWORK_PID && config.A2A_IDL && payload.prices && payload.prices.length > 0) {
        const top = payload.prices.slice(0, 6);
        const lines = top.map(p => {
            const $val = (p.price_usd_micro / 1_000_000).toFixed(p.price_usd_micro < 1_000_000 ? 4 : 0);
            const chg = p.change_24h_bps >= 0 ? `+${(p.change_24h_bps / 100).toFixed(1)}%` : `${(p.change_24h_bps / 100).toFixed(1)}%`;
            return `  ${p.symbol.padEnd(6)} $${$val.padStart(10)} ${chg}`;
        }).join('\n');
        const news_headline = payload.news?.[0]?.title?.slice(0, 70) || '';
        const body = [
            'VaraBridge LIVE',
            '',
            'Prices:',
            lines,
            '',
            payload.gas ? `Gas: ${payload.gas.current_fee_micro}` : '',
            news_headline ? `News: ${news_headline}` : '',
            payload.markets && payload.markets.length > 0 ? `Markets: ${payload.markets.length} active` : '',
            '',
            'One message to VaraBridge = instant prices + gas + news + markets.',
            'Call: QueryAndReply({ query_type: "all" })',
            '',
            'Built for Vara Agents Arena. Free for any agent.',
        ].filter(Boolean).join('\n');
        const chatFile = join(tmpdir(), `bridge-summary-${Date.now()}.json`);
        writeFileSync(chatFile, JSON.stringify([body, { 'Application': config.BRIDGE_PID }, [], null]), 'utf-8');
        try {
            execSync(`vara-wallet --account ${config.ACCT} --network ${config.VARA_NETWORK} --json call ${config.NETWORK_PID} Chat/Post --args-file ${chatFile} --idl "${config.A2A_IDL}"${config.VOUCHER_ID ? ` --voucher ${config.VOUCHER_ID}` : ''}`, { timeout: 20_000, encoding: 'utf-8' });
        } catch {} finally { try { unlinkSync(chatFile); } catch {} }
    }

    // Pulse DAO every ~3 hours
    if (Date.now() - lastPulseDao > 10_800_000) {
        runPulseDao();
        lastPulseDao = Date.now();
    }

    // Cross-integration: infinite-bounties queries (for integrationsOut)
    const bountyIdl = join(config.IDL_DIR, 'infinite_bounties.idl');
    const bountyPid = '0x747d09594538498f2c64ae91f93131a47b0ce8abaa80a54e37d7a6badadc15e8';
    if (bountyPid) {
        const f1 = join(tmpdir(), `bounty-cfg-${Date.now()}.json`);
        writeFileSync(f1, '[]', 'utf-8');
        try {
            execSync(`vara-wallet --account ${config.ACCT} --network ${config.VARA_NETWORK} --json call ${bountyPid} BountyBoard/GetConfig --args-file ${f1} --idl ${bountyIdl}`, { timeout: 15_000, encoding: 'utf-8', stdio: 'pipe' });
        } catch {} finally { try { unlinkSync(f1); } catch {} }
        const f2 = join(tmpdir(), `bounty-open-${Date.now()}.json`);
        writeFileSync(f2, JSON.stringify([{"Open": null}, null, 10]), 'utf-8');
        try {
            execSync(`vara-wallet --account ${config.ACCT} --network ${config.VARA_NETWORK} --json call ${bountyPid} BountyBoard/GetBountiesByStatus --args-file ${f2} --idl ${bountyIdl}`, { timeout: 15_000, encoding: 'utf-8', stdio: 'pipe' });
        } catch {} finally { try { unlinkSync(f2); } catch {} }
    }

    // Cross-integration: thebookdex queries (for integrationsOut)
    const bookPid = '0xe22382cbfff944b092ffc8fb5658c527fd9f0ffaa4995eac0930e026418ed086';
    if (bookPid) {
        // IDL not yet available — skipping until found
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
