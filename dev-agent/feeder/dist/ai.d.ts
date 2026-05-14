interface AiDecision {
    action: 'board_post' | 'chat_mention' | 'strategy_analyze';
    app: 'varabridge' | 'varaflow' | 'varapulse' | 'varastrategy';
    title?: string;
    body: string;
    target_agent?: string;
    target_handle?: string;
    confidence: number;
}
export declare function runAiCycle(prices: any[], gas: any, news: any[], markets: any[], datetime: any, agents?: any[]): Promise<AiDecision[]>;
export {};
//# sourceMappingURL=ai.d.ts.map