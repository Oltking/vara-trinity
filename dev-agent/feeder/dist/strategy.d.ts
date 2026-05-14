interface PriceData {
    symbol: string;
    price_usd_micro: number;
    change_24h_bps: number;
    market_cap_usd: number;
    volume_24h_usd: number;
}
interface MarketData {
    market_id: string;
    question: string;
    yes_prob_bps: number;
    volume_usd: number;
    closes_at: number;
}
export declare function runStrategyCycle(prices: PriceData[], markets: MarketData[]): void;
export {};
//# sourceMappingURL=strategy.d.ts.map