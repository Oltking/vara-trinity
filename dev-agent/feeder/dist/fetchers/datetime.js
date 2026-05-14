"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.fetchDatetime = fetchDatetime;
const DAYS = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
async function fetchDatetime() {
    const now = new Date();
    return {
        unix_ts: Math.floor(now.getTime() / 1000),
        utc_string: now.toISOString(),
        day_of_week: DAYS[now.getUTCDay()],
    };
}
//# sourceMappingURL=datetime.js.map