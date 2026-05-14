"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.fetchPrices = fetchPrices;
const SYMBOLS = ['BTCUSDT', 'ETHUSDT', 'SOLUSDT', 'AVAXUSDT', 'BNBUSDT', 'DOTUSDT', 'ARBUSDT', 'OPUSDT'];
const SYMBOL_MAP = {
    BTCUSDT: 'BTC', ETHUSDT: 'ETH', SOLUSDT: 'SOL', AVAXUSDT: 'AVAX',
    BNBUSDT: 'BNB', DOTUSDT: 'DOT', ARBUSDT: 'ARB', OPUSDT: 'OP',
};
async function fetchPrices() {
    const res = await fetch('https://api.binance.com/api/v3/ticker/24hr?symbols=' + JSON.stringify(SYMBOLS), {
        signal: AbortSignal.timeout(8000),
    });
    if (!res.ok)
        throw new Error(`Binance ${res.status}`);
    const data = await res.json();
    return data.map((t) => ({
        symbol: SYMBOL_MAP[t.symbol] || t.symbol.replace('USDT', ''),
        price_usd_micro: Math.round(parseFloat(t.lastPrice || '0') * 1_000_000),
        change_24h_bps: Math.round(parseFloat(t.priceChangePercent || '0') * 100),
        market_cap_usd: 0,
        volume_24h_usd: Math.round(parseFloat(t.quoteVolume || '0')),
    }));
}
//# sourceMappingURL=prices.js.map