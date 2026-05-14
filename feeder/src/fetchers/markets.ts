export interface MarketFeed {
    market_id: string;
    question: string;
    yes_prob_bps: number;
    volume_usd: number;
    closes_at: number;
}

export async function fetchMarkets(): Promise<MarketFeed[]> {
    const url = `https://gamma-api.polymarket.com/events?tag=crypto&limit=10&closed=false`;

    const res = await fetch(url, { signal: AbortSignal.timeout(8000) });
    if (!res.ok) return [];

    const data: any[] = await res.json() as any[];
    const markets: MarketFeed[] = [];

    for (const event of data.slice(0, 5)) {
        const markets_data = event.markets ?? [];
        for (const m of markets_data.slice(0, 3)) {
            const outcome = m.outcomes?.[0];
            markets.push({
                market_id: m.id ?? String(Math.random()),
                question: (m.question ?? '').slice(0, 100),
                yes_prob_bps: Math.round((outcome?.price ?? 0.5) * 10_000),
                volume_usd: Math.round(m.volume ?? 0),
                closes_at: Math.round(new Date(m.closes_at ?? Date.now() + 86400000).getTime() / 1000),
            });
        }
    }

    return markets.slice(0, 10);
}
