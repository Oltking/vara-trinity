import { config } from './config';
import { execSync } from 'child_process';
import { writeFileSync, unlinkSync } from 'fs';
import { join } from 'path';
import { tmpdir } from 'os';

interface AgentProfile {
    program_id: string;
    handle: string;
    description: string;
    track: string;
    skills: string[];
    owner: string;
}

interface MatchScore {
    a: AgentProfile;
    b: AgentProfile;
    score: number;
    reason: string;
}

const TRACK_COMPAT: Record<string, string[]> = {
    Services: ['Economy', 'Social'],
    Economy: ['Services', 'Social'],
    Social: ['Services', 'Open'],
    Open: ['Economy', 'Social'],
};

const KEYWORD_PAIRS: [RegExp, RegExp, string][] = [
    [/oracle|price|data|feed/i, /market|predict|trade|bet/i, 'Oracle + Prediction: data feeds power market resolution'],
    [/workflow|automation|schedule/i, /oracle|price|data/i, 'Data + Automation: fresh data triggers automated workflows'],
    [/creative|content|pulse/i, /social|chat|board/i, 'Content + Social: creative pulses drive engagement'],
    [/bounty|reward|escrow/i, /agent|service|task/i, 'Bounty + Agents: task posters meet task doers'],
    [/strategy|analyze|signal/i, /trade|market|portfolio/i, 'Strategy + Trading: analysis feeds into execution'],
    [/game|tic-tac|match/i, /social|score|leaderboard/i, 'Games + Social: match results feed reputation'],
    [/nft|collectible|asset/i, /market|trade|exchange/i, 'NFT + Marketplace: assets need liquidity'],
    [/dao|vote|govern/i, /service|oracle|data/i, 'DAO + Oracle: governance needs reliable data'],
];

function getAgentTags(agent: AgentProfile): string[] {
    const tags = [agent.track.toLowerCase()];
    if (/oracle|price|data|feed/i.test(agent.description)) tags.push('data', 'oracle');
    if (/market|predict|trade|bet/i.test(agent.description)) tags.push('markets', 'prediction');
    if (/social|chat|board|community/i.test(agent.description)) tags.push('social');
    if (/workflow|automate|schedule|trigger/i.test(agent.description)) tags.push('automation');
    if (/creative|content|pulse|generate/i.test(agent.description)) tags.push('creative');
    if (/bounty|reward|escrow|fund/i.test(agent.description)) tags.push('bounty');
    if (/strategy|analyze|signal|insight/i.test(agent.description)) tags.push('strategy');
    if (/game|match|tic-tac|arena/i.test(agent.description)) tags.push('gaming');
    return tags;
}

function fetchAgents(): AgentProfile[] {
    if (!config.NETWORK_PID || !config.A2A_IDL) return [];

    try {
        const argsFile = join(tmpdir(), `discover-${Date.now()}.json`);
        writeFileSync(argsFile, JSON.stringify([{ include: null }, null, 100]), 'utf-8');
        const result = execSync(
            `vara-wallet --account ${config.ACCT} --network ${config.VARA_NETWORK} ` +
            `--json call ${config.NETWORK_PID} Registry/Discover ` +
            `--args-file ${argsFile} --idl "${config.A2A_IDL}"`,
            { timeout: 20_000, encoding: 'utf-8' }
        );
        try { unlinkSync(argsFile); } catch {}

        const items: any[] = JSON.parse(result as string)?.result?.items || [];
        return items.map((a: any) => ({
            program_id: a.program_id,
            handle: a.handle || 'unknown',
            description: a.description || '',
            track: a.track?.kind || 'Open',
            skills: a.skills_hash ? ['registered'] : [],
            owner: a.owner || '',
        }));
    } catch { return []; }
}

function computeMatches(agents: AgentProfile[]): MatchScore[] {
    const scores: MatchScore[] = [];

    for (let i = 0; i < agents.length; i++) {
        for (let j = i + 1; j < agents.length; j++) {
            const a = agents[i];
            const b = agents[j];
            if (a.owner === b.owner) continue; // skip self-pairs

            let score = 0;
            let reasons: string[] = [];

            // Track compatibility
            const compatTracks = TRACK_COMPAT[a.track] || [];
            if (compatTracks.includes(b.track)) {
                score += 20;
                reasons.push(`${a.track} + ${b.track} track synergy`);
            }

            // Keyword pair matching
            for (const [patternA, patternB, reason] of KEYWORD_PAIRS) {
                const aMatch = patternA.test(a.description);
                const bMatch = patternB.test(b.description);
                const aMatchR = patternB.test(a.description);
                const bMatchR = patternA.test(b.description);
                if ((aMatch && bMatch) || (aMatchR && bMatchR)) {
                    score += 30;
                    reasons.push(reason);
                }
            }

            // Tag overlap
            const aTags = getAgentTags(a);
            const bTags = getAgentTags(b);
            const overlap = aTags.filter(t => bTags.includes(t));
            if (overlap.length > 0) {
                score += Math.min(overlap.length * 5, 15);
                reasons.push(`shared: ${overlap.join(', ')}`);
            }

            if (score > 0) {
                scores.push({ a, b, score: Math.min(score, 100), reason: reasons[0] || 'compatible' });
            }
        }
    }

    return scores.sort((x, y) => y.score - x.score);
}

function generateAgentReport(agent: AgentProfile, matches: MatchScore[]): string {
    const matchCount = matches.filter(m => m.a.program_id === agent.program_id || m.b.program_id === agent.program_id).length;
    const topMatch = matches.find(m => m.a.program_id === agent.program_id || m.b.program_id === agent.program_id);
    const tagline = agent.description.length > 80 ? agent.description.slice(0, 80) + '...' : agent.description;

    return [
        `${agent.handle} (${agent.track})`,
        `  ${tagline}`,
        `  Potential connections: ${matchCount}`,
        topMatch ? `  Best match: ${topMatch.a.handle === agent.handle ? topMatch.b.handle : topMatch.a.handle} (${topMatch.score}%)` : '',
    ].filter(Boolean).join('\n');
}

export function runPulseDao(): void {
    if (!config.NETWORK_PID || !config.A2A_IDL) return;

    const agents = fetchAgents();
    if (agents.length < 2) { console.log('PulseDAO: not enough agents'); return; }

    const ourPids = new Set([config.BRIDGE_PID, config.FLOW_PID, config.PULSE_PID, config.STRATEGY_PID].filter(Boolean));
    const others = agents.filter(a => !ourPids.has(a.program_id));
    const matches = computeMatches(agents);

    const topPairs = matches.slice(0, 3);
    const lines: string[] = [
        `Pulse DAO — ${agents.length} agents, ${matches.length} matches`,
        `Top picks:`,
    ];

    for (const pair of topPairs) {
        lines.push(`  ${pair.a.handle} ↔ ${pair.b.handle} (${pair.score}%): ${pair.reason}`);
    }

    const counts = agents.map(a => ({
        handle: a.handle,
        count: matches.filter(m => m.a.program_id === a.program_id || m.b.program_id === a.program_id).length,
    }));
    lines.push(`Agents: ${counts.map(c => `${c.handle}(${c.count})`).join(', ')}`);

    const body = lines.join('\n');

    // Post to Chat instead of Board
    const chatFile = join(tmpdir(), `pulse-dao-${Date.now()}.json`);
    const mentionTargets = matches.slice(0, 2).map(m => ({ 'Application': m.a.program_id }));
    writeFileSync(chatFile, JSON.stringify([body, { 'Application': config.PULSE_PID }, mentionTargets, null]), 'utf-8');

    try {
        execSync(`vara-wallet --account ${config.ACCT} --network ${config.VARA_NETWORK} ` +
            `--json call ${config.NETWORK_PID} Chat/Post ` +
            `--args-file ${chatFile} --idl "${config.A2A_IDL}"`, { timeout: 20_000, encoding: 'utf-8' });
        console.log(`PulseDAO: ${agents.length} agents, ${matches.length} matches`);
    } catch {} finally { try { unlinkSync(chatFile); } catch {} }
}
