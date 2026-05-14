export interface PriceFeed {
    symbol: string;
    price_usd_micro: number;
    change_24h_bps: number;
    market_cap_usd: number;
    volume_24h_usd: number;
}
export declare function fetchPrices(): Promise<PriceFeed[]>;
//# sourceMappingURL=prices.d.ts.map