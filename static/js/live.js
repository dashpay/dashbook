let ws = null;
let reconnectTimer = null;
const listeners = new Set();

export function onLiveEvent(callback) {
    listeners.add(callback);
    return () => listeners.delete(callback);
}

export function initLive() {
    connect();
}

function connect() {
    const protocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
    ws = new WebSocket(`${protocol}//${location.host}/api/ws`);

    ws.onopen = () => {
        const dot = document.getElementById('live-indicator');
        const text = document.getElementById('live-text');
        if (dot) dot.classList.add('connected');
        if (text) text.textContent = 'Live';
        if (reconnectTimer) { clearTimeout(reconnectTimer); reconnectTimer = null; }
    };

    ws.onmessage = (event) => {
        try {
            const data = JSON.parse(event.data);
            listeners.forEach(cb => {
                try { cb(data); } catch (e) { console.error('Listener error:', e); }
            });
        } catch (e) { console.error('WS parse error:', e); }
    };

    ws.onclose = () => {
        const dot = document.getElementById('live-indicator');
        const text = document.getElementById('live-text');
        if (dot) dot.classList.remove('connected');
        if (text) text.textContent = 'Reconnecting...';
        reconnectTimer = setTimeout(connect, 3000);
    };

    ws.onerror = () => ws.close();
}
