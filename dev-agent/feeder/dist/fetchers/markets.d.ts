export interface MarketFeed {
    market_id: string;
    question: string;
    yes_prob_bps: number;
    volume_usd: number;
    closes_at: number;
}
export declare function fetchMarkets(): Promise<MarketFeed[]>;
//# sourceMappingURL=markets.d.ts.map