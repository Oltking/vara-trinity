interface FullUpdatePayload {
    prices: any[] | null;
    gas: any | null;
    news: any[] | null;
    markets: any[] | null;
    datetime: any | null;
}
export declare function submitUpdate(payload: FullUpdatePayload): Promise<void>;
export {};
//# sourceMappingURL=sender.d.ts.map