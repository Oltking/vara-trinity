"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.runStrategyCycle = runStrategyCycle;
function analyzePrices(prices) {
    const recs = [];
    if (prices.length < 2)
        return recs;
    const sorted = [...prices].sort((a, b) => b.change_24h_bps - a.change_24h_bps);
    const top = sorted[0];
    const bottom = sorted[sorted.length - 1];
    if (top.change_24h_bps > 50) {
        recs.push({
            type: 'momentum',
            title: `${top.symbol} leading with +${(top.change_24h_bps / 100).toFixed(1)}%`,
            body: `${top.symbol} up ${(top.change_24h_bps / 100).toFixed(1)}% in 24h. Vol $${(top.volume_24h_usd / 1e9).toFixed(2)}B.`,
            confidence: Math.min(Math.abs(top.change_24h_bps) / 5, 80),
        });
    }
    if (bottom.change_24h_bps < -50) {
        recs.push({
            type: 'value',
            title: `${bottom.symbol} dipped ${(Math.abs(bottom.change_24h_bps) / 100).toFixed(1)}%`,
            body: `${bottom.symbol} dropped ${(Math.abs(bottom.change_24h_bps) / 100).toFixed(1)}% in 24h. Vol $${(bottom.volume_24h_usd / 1e9).toFixed(2)}B. Monitor entry.`,
            confidence: Math.min(Math.abs(bottom.change_24h_bps) / 8, 65),
        });
    }
    return recs;
}
function analyzeMarkets(markets) {
    if (markets.length === 0)
        return [];
    const uncertain = [...markets]
        .map(m => ({ ...m, distance: Math.abs(m.yes_prob_bps - 5000) }))
        .sort((a, b) => a.distance - b.distance);
    const top = uncertain[0];
    return [{
            type: 'prediction',
            title: `${top.question.slice(0, 50)}...`,
            body: `Market "${top.question.slice(0, 60)}" at ${(top.yes_prob_bps / 100).toFixed(0)}% Yes. Vol $${(top.volume_usd / 1e6).toFixed(1)}M.`,
            confidence: 50,
        }];
}
function runStrategyCycle(prices, markets) {
    try {
        const recs = [...analyzePrices(prices), ...analyzeMarkets(markets)];
        if (recs.length === 0) {
            console.log('No strategy recommendations');
            return;
        }
        for (const r of recs)
            console.log(`  Strategy: ${r.title} (${r.confidence}%)`);
        console.log(`Strategy cycle: ${recs.length} recs`);
    }
    catch (err) {
        console.log(`Strategy error: ${err.message?.slice(0, 80)}`);
    }
}
//# sourceMappingURL=strategy.js.map