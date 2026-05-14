export interface GasFeed {
    current_fee_micro: number;
    suggested_tip: number;
    block_num: number;
    finalized_hash: string;
}
export declare function fetchGas(): Promise<GasFeed>;
//# sourceMappingURL=gas.d.ts.map