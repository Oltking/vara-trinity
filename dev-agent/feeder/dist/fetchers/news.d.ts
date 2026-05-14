export interface NewsSummary {
    title: string;
    source: string;
    published_at: number;
    category: string;
}
export declare function fetchNews(): Promise<NewsSummary[]>;
//# sourceMappingURL=news.d.ts.map