export const api = {
    async get(path) {
        const resp = await fetch(path);
        if (!resp.ok) {
            const err = await resp.json().catch(() => ({ error: resp.statusText }));
            throw new Error(err.error || 'API request failed');
        }
        return resp.json();
    }
};
