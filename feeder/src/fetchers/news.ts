import { config } from '../config';

export interface NewsSummary {
    title: string;
    source: string;
    published_at: number;
    category: string;
}

async function fetchNewsDataIO(): Promise<NewsSummary[]> {
    if (!config.NEWS_API_KEY) return [];

    const url = `https://newsdata.io/api/1/news` +
        `?apikey=${config.NEWS_API_KEY}` +
        `&q=crypto+bitcoin+ethereum+blockchain` +
        `&language=en` +
        `&size=5`;

    try {
        const res = await fetch(url, { signal: AbortSignal.timeout(8000) });
        if (!res.ok) return [];

        const data: any = await res.json();
        if (!data.results) return [];

        return data.results.map((a: any) => ({
            title: (a.title ?? '').slice(0, 120),
            source: a.source_id ?? 'unknown',
            published_at: new Date(a.pubDate ?? Date.now()).getTime() / 1000,
            category: 'crypto',
        }));
    } catch {
        return [];
    }
}

function deduplicateNews(articles: NewsSummary[]): NewsSummary[] {
    const seen = new Set<string>();
    return articles.filter(a => {
        const key = a.title.toLowerCase().slice(0, 60);
        if (seen.has(key)) return false;
        seen.add(key);
        return true;
    });
}

export async function fetchNews(): Promise<NewsSummary[]> {
    const result = await fetchNewsDataIO();
    if (result.length === 0) return [];

    return deduplicateNews(result)
        .sort((a, b) => b.published_at - a.published_at)
        .slice(0, 10)
        .map(n => ({
            ...n,
            title: n.title.slice(0, 120),
        }));
}
