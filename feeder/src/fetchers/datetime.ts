export interface DatetimeFeed {
    unix_ts: number;
    utc_string: string;
    day_of_week: string;
}

const DAYS = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];

export async function fetchDatetime(): Promise<DatetimeFeed> {
    const now = new Date();
    return {
        unix_ts: Math.floor(now.getTime() / 1000),
        utc_string: now.toISOString(),
        day_of_week: DAYS[now.getUTCDay()],
    };
}
