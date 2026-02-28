// ============ Badges ============

export function chainlockBadge(locked) {
    if (locked) {
        return `<span class="badge badge-chainlock">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><rect x="3" y="11" width="18" height="11" rx="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>
            ChainLocked</span>`;
    }
    return '<span class="badge badge-pending">Pending</span>';
}

export function instantSendBadge(locked, internal) {
    if (!locked) return '';
    const label = internal ? 'IS (LLMQ)' : 'InstantSend';
    return `<span class="badge badge-instantsend">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10"/></svg>
        ${label}</span>`;
}

export function txTypeBadge(type) {
    const types = {
        0: { label: 'Standard', cls: 'badge-standard' },
        1: { label: 'ProRegTx', cls: 'badge-proreg' },
        2: { label: 'ProUpServTx', cls: 'badge-proupserv' },
        3: { label: 'ProUpRegTx', cls: 'badge-proupreg' },
        4: { label: 'ProUpRevTx', cls: 'badge-prouprev' },
        5: { label: 'CoinBase', cls: 'badge-coinbase' },
        6: { label: 'QcTx', cls: 'badge-qc' },
        8: { label: 'AssetLock', cls: 'badge-assetlock' },
        9: { label: 'AssetUnlock', cls: 'badge-assetunlock' },
    };
    const t = types[type] || { label: `Type ${type}`, cls: 'badge-standard' };
    return `<span class="badge ${t.cls}">${t.label}</span>`;
}

export function mnStatusBadge(status) {
    if (status === 'ENABLED') return '<span class="badge badge-enabled">ENABLED</span>';
    if (status === 'POSE_BANNED') return '<span class="badge badge-banned">POSE_BANNED</span>';
    return `<span class="badge badge-standard">${status}</span>`;
}

export function mnTypeBadge(type) {
    if (type === 'Evo') return '<span class="badge badge-evo">Evo</span>';
    return '<span class="badge badge-regular">Regular</span>';
}

// ============ Formatting ============

export function formatDash(satoshis) {
    return (satoshis / 100000000).toFixed(8) + ' DASH';
}

export function formatDashShort(value) {
    const n = parseFloat(value);
    if (n >= 1000) return n.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 }) + ' DASH';
    return n.toLocaleString(undefined, { minimumFractionDigits: 4, maximumFractionDigits: 4 }) + ' DASH';
}

export function formatDashValue(value) {
    return parseFloat(value).toFixed(8);
}

export function formatNumber(n) {
    return Number(n).toLocaleString();
}

export function formatTime(ts) {
    return new Date(ts * 1000).toLocaleString();
}

export function timeAgo(ts) {
    const s = Math.floor(Date.now() / 1000 - ts);
    if (s < 5) return 'just now';
    if (s < 60) return `${s}s ago`;
    if (s < 3600) return `${Math.floor(s / 60)}m ago`;
    if (s < 86400) return `${Math.floor(s / 3600)}h ago`;
    return `${Math.floor(s / 86400)}d ago`;
}

export function truncHash(hash, n = 8) {
    if (!hash) return '';
    if (hash.length <= n * 2 + 3) return hash;
    return hash.slice(0, n) + '...' + hash.slice(-n);
}

// ============ Links ============

export function hashLink(hash, type = 'tx') {
    return `<a href="#/${type}/${hash}" class="hash-link mono" title="${hash}">${truncHash(hash)}</a>`;
}

export function blockLink(heightOrHash) {
    return `<a href="#/block/${heightOrHash}" class="hash-link">${heightOrHash}</a>`;
}

export function addressLink(addr) {
    if (!addr) return '<span class="text-muted">N/A</span>';
    return `<a href="#/address/${addr}" class="address-link mono" title="${addr}">${truncHash(addr, 10)}</a>`;
}

export function fullAddressLink(addr) {
    if (!addr) return '<span class="text-muted">N/A</span>';
    return `<a href="#/address/${addr}" class="address-link mono">${addr}</a>`;
}

// ============ Pagination ============

export function renderPagination(page, totalPages, onChange) {
    if (totalPages <= 1) return '';

    const items = [];
    items.push(`<button ${page <= 1 ? 'disabled' : ''} data-page="${page - 1}">&laquo; Prev</button>`);

    const start = Math.max(1, page - 2);
    const end = Math.min(totalPages, page + 2);

    if (start > 1) items.push(`<span class="page-num" data-page="1">1</span>`);
    if (start > 2) items.push(`<span class="page-info">...</span>`);

    for (let i = start; i <= end; i++) {
        items.push(`<span class="page-num ${i === page ? 'active' : ''}" data-page="${i}">${i}</span>`);
    }

    if (end < totalPages - 1) items.push(`<span class="page-info">...</span>`);
    if (end < totalPages) items.push(`<span class="page-num" data-page="${totalPages}">${totalPages}</span>`);

    items.push(`<button ${page >= totalPages ? 'disabled' : ''} data-page="${page + 1}">Next &raquo;</button>`);

    const html = `<div class="pagination">${items.join('')}</div>`;
    return html;
}

export function bindPagination(container, onChange) {
    container.querySelectorAll('.pagination [data-page]').forEach(el => {
        el.addEventListener('click', () => {
            const p = parseInt(el.dataset.page);
            if (!isNaN(p)) onChange(p);
        });
    });
}

// ============ Escape ============

export function escapeHtml(str) {
    const d = document.createElement('div');
    d.textContent = str;
    return d.innerHTML;
}

// ============ Loading ============

export function showLoading(container) {
    if (typeof container === 'string') container = document.getElementById(container);
    if (container) container.innerHTML = '<div class="loading-page"><div class="spinner"></div></div>';
}

// ============ Search ============

export function initSearch() {
    const doSearch = async (input) => {
        const q = input.value.trim();
        if (!q) return;
        try {
            const { api: apiClient } = await import('./api.js');
            const result = await apiClient.get(`/api/search?q=${encodeURIComponent(q)}`);
            if (result.type === 'block') location.hash = `#/block/${result.value}`;
            else if (result.type === 'tx') location.hash = `#/tx/${result.value}`;
            else if (result.type === 'address') location.hash = `#/address/${result.value}`;
            else if (result.type === 'masternode') location.hash = `#/masternode/${result.value}`;
            else alert('Nothing found for: ' + q);
            input.value = '';
        } catch (e) {
            console.error('Search error:', e);
        }
    };

    // Header search
    const globalInput = document.getElementById('global-search');
    const searchBtn = document.getElementById('search-btn');

    if (globalInput) {
        globalInput.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') doSearch(globalInput);
        });
    }
    if (searchBtn) {
        searchBtn.addEventListener('click', () => doSearch(globalInput));
    }
}
