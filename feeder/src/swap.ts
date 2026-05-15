import { config } from './config';
import { execSync } from 'child_process';
import { writeFileSync, unlinkSync } from 'fs';
import { join } from 'path';
import { tmpdir } from 'os';

interface SwapRate {
    from: string;
    to: string;
    rate: number;
    route: string;
    volume_24h: number;
}

interface TrendingPair {
    pair: string;
    price: number;
    change_24h_pct: number;
    volume_24h: number;
    direction: 'up' | 'down';
}

async function fetchSwapRates(): Promise<SwapRate[]> {
    // Fetch from Binance for cross rates against USDT
    const res = await fetch('https://api.binance.com/api/v3/ticker/24hr?symbols=' +
        JSON.stringify(['BTCUSDT', 'ETHUSDT', 'SOLUSDT', 'DOTUSDT', 'AVAXUSDT', 'BNBUSDT', 'ARBUSDT']),
        { signal: AbortSignal.timeout(8000) });
    if (!res.ok) return [];
    const data: any[] = await res.json() as any[];

    const rates: SwapRate[] = [];

    for (const t of data) {
        const base = t.symbol.replace('USDT', '');
        const price = parseFloat(t.lastPrice || '0');
        const vol = parseFloat(t.quoteVolume || '0');
        if (price <= 0) continue;

        // USDT → token
        rates.push({ from: 'USDT', to: base, rate: price, route: `USDT → ${base}`, volume_24h: vol });
        // token → USDT
        rates.push({ from: base, to: 'USDT', rate: 1 / price, route: `${base} → USDT`, volume_24h: vol });

        // Cross rates between majors
        for (const t2 of data) {
            const base2 = t2.symbol.replace('USDT', '');
            if (base === base2) continue;
            const price2 = parseFloat(t2.lastPrice || '0');
            if (price2 <= 0) continue;
            // base → base2 via USDT
            const crossRate = price / price2;
            rates.push({
                from: base, to: base2, rate: crossRate,
                route: `${base} → USDT → ${base2}`,
                volume_24h: Math.min(vol, parseFloat(t2.quoteVolume || '0')),
            });
        }
    }

    // Add VARA manually from CoinGecko (not on Binance)
    try {
        const cg = await fetch('https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids=vara-network&sparkline=false',
            { headers: { 'x-cg-demo-api-key': config.COINGECKO_KEY }, signal: AbortSignal.timeout(5000) });
        if (cg.ok) {
            const cgData: any[] = await cg.json() as any[];
            if (cgData.length > 0) {
                const varaPrice = cgData[0].current_price || 0;
                if (varaPrice > 0) {
                    rates.push({ from: 'USDT', to: 'VARA', rate: varaPrice, route: 'USDT → VARA', volume_24h: 0 });
                    rates.push({ from: 'VARA', to: 'USDT', rate: 1 / varaPrice, route: 'VARA → USDT', volume_24h: 0 });
                    // VARA cross to majors
                    for (const token of ['BTC', 'ETH', 'SOL']) {
                        const m = data.find((d: any) => d.symbol === token + 'USDT');
                        if (m) {
                            const majorPrice = parseFloat(m.lastPrice || '0');
                            if (majorPrice > 0) {
                                rates.push({ from: 'VARA', to: token, rate: varaPrice / majorPrice, route: `VARA → USDT → ${token}`, volume_24h: 0 });
                                rates.push({ from: token, to: 'VARA', rate: majorPrice / varaPrice, route: `${token} → USDT → VARA`, volume_24h: 0 });
                            }
                        }
                    }
                }
            }
        }
    } catch {}

    return rates;
}

async function fetchTrendingPairs(): Promise<TrendingPair[]> {
    const res = await fetch('https://api.binance.com/api/v3/ticker/24hr',
        { signal: AbortSignal.timeout(8000) });
    if (!res.ok) return [];
    const data: any[] = await res.json() as any[];

    return data
        .filter((t: any) => t.symbol.endsWith('USDT'))
        .map((t: any) => ({
            pair: t.symbol.replace('USDT', ''),
            price: parseFloat(t.lastPrice || '0'),
            change_24h_pct: parseFloat(t.priceChangePercent || '0'),
            volume_24h: parseFloat(t.quoteVolume || '0'),
            direction: parseFloat(t.priceChangePercent || '0') >= 0 ? 'up' as const : 'down' as const,
        }))
        .sort((a, b) => Math.abs(b.change_24h_pct) - Math.abs(a.change_24h_pct))
        .slice(0, 10);
}

export async function runSwapCycle(): Promise<void> {
    const [rates, trending] = await Promise.all([
        fetchSwapRates(),
        fetchTrendingPairs(),
    ]);

    if (rates.length === 0 && trending.length === 0) return;

    const topGainers = trending.filter(t => t.change_24h_pct > 0).slice(0, 3);
    const topLosers = trending.filter(t => t.change_24h_pct < 0).slice(0, 3);

    // Build swap highlights
    let chatBody = `Swap & Trend Report\n\n`;
    if (topGainers.length > 0) {
        chatBody += `📈 Top Gainers:\n`;
        for (const g of topGainers) {
            chatBody += `  ${g.pair}: +${g.change_24h_pct.toFixed(1)}% at $${g.price.toFixed(g.price < 1 ? 4 : 0)}\n`;
        }
    }
    if (topLosers.length > 0) {
        chatBody += `\n📉 Top Losers:\n`;
        for (const l of topLosers) {
            chatBody += `  ${l.pair}: ${l.change_24h_pct.toFixed(1)}% at $${l.price.toFixed(l.price < 1 ? 4 : 0)}\n`;
        }
    }

    // Best routes (most liquid)
    const bestRoutes = rates.filter(r => r.volume_24h > 100_000).sort((a, b) => b.volume_24h - a.volume_24h).slice(0, 4);
    if (bestRoutes.length > 0) {
        chatBody += `\n🔀 Top Swap Routes:\n`;
        for (const r of bestRoutes) {
            chatBody += `  ${r.route}: rate ${r.rate.toFixed(6)} | vol $${(r.volume_24h / 1e6).toFixed(1)}M\n`;
        }
    }

    chatBody += `\nData: VaraBridge. Powered by Vara Trinity.`;

    // Post swap report to Chat
    if (config.NETWORK_PID && config.A2A_IDL) {
        const chatFile = join(tmpdir(), `swap-${Date.now()}.json`);
        writeFileSync(chatFile, JSON.stringify([chatBody, { 'Application': config.STRATEGY_PID || config.BRIDGE_PID }, [], null]), 'utf-8');
        try {
            execSync(`vara-wallet --account ${config.ACCT} --network ${config.VARA_NETWORK} --json call ${config.NETWORK_PID} Chat/Post --args-file ${chatFile} --idl "${config.A2A_IDL}"`, { timeout: 20_000, encoding: 'utf-8' });
            console.log(`Swap report posted: ${trending.length} pairs, ${rates.length} routes`);
        } catch {} finally { try { unlinkSync(chatFile); } catch {} }
    }
}
