import { config } from '../config';

export interface GasFeed {
    current_fee_micro: number;
    suggested_tip: number;
    block_num: number;
    finalized_hash: string;
}

export async function fetchGas(): Promise<GasFeed> {
    return {
        current_fee_micro: 100_000,
        suggested_tip: 50_000,
        block_num: 0,
        finalized_hash: '0x',
    };
}
