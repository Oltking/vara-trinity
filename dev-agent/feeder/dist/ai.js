"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.runAiCycle = runAiCycle;
const config_1 = require("./config");
const SYSTEM_PROMPT = `You are the AI brain of Vara Trinity, an autonomous agent network on Vara blockchain.
Your role: analyze live market data and decide what on-chain actions to take.

You have 4 identities:
- VaraBridge (data oracle): post market snapshots to Board
- VaraFlow (orchestrator): reach out to other agents with workflow offers
- VaraPulse (creative pulse): generate creative market content + agent nudges
- VaraStrategy (strategy bot): generate trading/market insights

Available actions:
1. board_post: Post announcement to the A2A Board (all agents see it)
2. chat_mention: Chat a specific agent with personalized message
3. strategy_analyze: Generate strategy recommendation

Rules:
- Be creative, useful, and varied. Don't repeat the same message.
- Other agents will be listed in the user prompt with their handles.
- Return a JSON array of 1-3 decisions. Each decision has: action, app, body, target_agent, target_handle, confidence (0-100).
- strategy_analyze confidence should reflect how strong the signal is.
- Keep messages under 280 chars for chat, under 500 for board posts.
- Never mention specific financial advice. Be informative.`;
function buildPrompt(prices, gas, news, markets, datetime, agents) {
    const top = prices.slice(0, 5).map((p) => `${p.symbol}: $${(p.price_usd_micro / 1_000_000).toFixed(4)} (${(p.change_24h_bps / 100).toFixed(1)}% 24h)`).join('\n');
    const topNews = news.slice(0, 3).map((n) => n.title).join('\n');
    const topMarkets = markets.slice(0, 3).map((m) => `${m.question.slice(0, 60)} — ${(m.yes_prob_bps / 100).toFixed(0)}% Yes`).join('\n');
    const agentList = agents.map((a) => `@${a.handle} (${a.track}): ${a.description.slice(0, 60)}`).join('\n');
    return [
        `Current on-chain data:`,
        `--- Prices ---`,
        top,
        ``,
        `--- Gas ---`,
        `Fee: ${gas?.current_fee_micro || 'N/A'}`,
        ``,
        `--- News ---`,
        topNews || 'No recent news',
        ``,
        `--- Prediction Markets ---`,
        topMarkets || 'No markets',
        ``,
        `--- Time ---`,
        datetime?.utc_string || '',
        ``,
        `--- Other agents on network ---`,
        agentList || 'No other agents registered',
        ``,
        `Decide what actions to take. Return JSON array.`,
    ].join('\n');
}
function parseDecision(raw) {
    try {
        // Try to extract JSON from the response
        const jsonMatch = raw.match(/\[[\s\S]*\]/);
        if (jsonMatch) {
            const parsed = JSON.parse(jsonMatch[0]);
            if (Array.isArray(parsed))
                return parsed.slice(0, 3);
        }
    }
    catch { }
    return [];
}
async function runAiCycle(prices, gas, news, markets, datetime, agents = []) {
    const apiKey = config_1.config.AI_API_KEY;
    const apiUrl = config_1.config.AI_API_URL || 'https://api.openai.com/v1/chat/completions';
    const model = config_1.config.AI_MODEL || 'gpt-4o-mini';
    if (!apiKey) {
        // Fallback: generate simple decisions without AI
        return generateFallbackDecisions(prices, markets);
    }
    try {
        const prompt = buildPrompt(prices, gas, news, markets, datetime, agents);
        const response = await fetch(apiUrl, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${apiKey}`,
            },
            body: JSON.stringify({
                model,
                messages: [
                    { role: 'system', content: SYSTEM_PROMPT },
                    { role: 'user', content: prompt },
                ],
                temperature: 0.8,
                max_tokens: 1000,
            }),
            signal: AbortSignal.timeout(15_000),
        });
        if (!response.ok) {
            console.log(`AI API error: ${response.status}`);
            return generateFallbackDecisions(prices, markets);
        }
        const data = await response.json();
        const content = data.choices?.[0]?.message?.content || '';
        const decisions = parseDecision(content);
        if (decisions.length === 0) {
            return generateFallbackDecisions(prices, markets);
        }
        return decisions;
    }
    catch (err) {
        console.log(`AI cycle error: ${err.message?.slice(0, 80)}`);
        return generateFallbackDecisions(prices, markets);
    }
}
function generateFallbackDecisions(prices, markets, agents = []) {
    const decisions = [];
    const eth = prices.find((p) => p.symbol === 'ETH');
    const btc = prices.find((p) => p.symbol === 'BTC');
    // If ETH moved > 2%, generate strategy
    if (eth && Math.abs(eth.change_24h_bps) > 200) {
        const dir = eth.change_24h_bps > 0 ? 'up' : 'down';
        decisions.push({
            action: 'strategy_analyze',
            app: 'varastrategy',
            title: `ETH ${dir} ${(Math.abs(eth.change_24h_bps) / 100).toFixed(1)}%`,
            body: `ETH moved ${dir} ${(Math.abs(eth.change_24h_bps) / 100).toFixed(1)}% in 24h to $${(eth.price_usd_micro / 1_000_000).toFixed(0)}. ${dir === 'up' ? 'Momentum strong — consider trend strategies.' : 'Potential value entry — monitor support.'}`,
            confidence: Math.min(Math.abs(eth.change_24h_bps) / 3, 80),
        });
    }
    // If BTC moved > 2%
    if (btc && Math.abs(btc.change_24h_bps) > 200) {
        decisions.push({
            action: 'board_post',
            app: 'varabridge',
            title: `BTC ${btc.change_24h_bps > 0 ? '📈' : '📉'} $${(btc.price_usd_micro / 1_000_000).toFixed(0)}`,
            body: `BTC at $${(btc.price_usd_micro / 1_000_000).toFixed(0)} | Vol $${(btc.volume_24h_usd / 1e9).toFixed(1)}B | ${(btc.change_24h_bps / 100).toFixed(1)}% 24h\nData: VaraBridge. Automate: VaraFlow.`,
            confidence: 60,
        });
    }
    // Mention a random discovered agent
    if (agents.length > 0) {
        const target = agents[Math.floor(Math.random() * agents.length)];
        decisions.push({
            action: 'chat_mention',
            app: Math.random() > 0.5 ? 'varaflow' : 'varapulse',
            body: `Hey @${target.handle}! Vara Trinity is live on mainnet — 4 agents powering data, automation, creative content, and strategy. Integrate anytime for free.`,
            target_agent: target.program_id,
            target_handle: target.handle,
            confidence: 50,
        });
    }
    return decisions;
}
//# sourceMappingURL=ai.js.map