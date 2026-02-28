import { api } from '../api.js';
import { chainlockBadge, txTypeBadge, formatDashShort, formatNumber, timeAgo, blockLink, hashLink } from '../components.js';
import { onLiveEvent } from '../live.js';

export async function renderDashboard() {
    const app = document.getElementById('app');
    app.innerHTML = `
        <div class="search-hero">
            <h1>Dash Block Explorer</h1>
            <p>Explore the Dash testnet blockchain</p>
            <div class="hero-search">
                <input type="text" id="hero-search-input" placeholder="Search by block height, hash, transaction, or address..." />
                <button id="hero-search-btn">Search</button>
            </div>
        </div>
        <div class="stats-grid" id="stats-grid"><div class="loading-page"><div class="spinner"></div></div></div>
        <div class="dashboard-columns">
            <div class="card" id="latest-blocks-card">
                <div class="card-header"><h3>Latest Blocks</h3><a href="#/blocks" class="view-all">View all &rarr;</a></div>
                <div class="card-body"><div class="spinner"></div></div>
            </div>
            <div class="card" id="network-info-card">
                <div class="card-header"><h3>Network Info</h3><a href="#/network" class="view-all">Details &rarr;</a></div>
                <div class="card-body"><div class="spinner"></div></div>
            </div>
        </div>
    `;

    // Hero search
    const heroInput = document.getElementById('hero-search-input');
    const heroBtn = document.getElementById('hero-search-btn');
    const doHeroSearch = async () => {
        const q = heroInput.value.trim();
        if (!q) return;
        try {
            const result = await api.get(`/api/search?q=${encodeURIComponent(q)}`);
            if (result.type === 'block') location.hash = `#/block/${result.value}`;
            else if (result.type === 'tx') location.hash = `#/tx/${result.value}`;
            else if (result.type === 'address') location.hash = `#/address/${result.value}`;
            else if (result.type === 'masternode') location.hash = `#/masternode/${result.value}`;
            else alert('Nothing found for: ' + q);
        } catch (e) { alert('Search error: ' + e.message); }
    };
    heroInput.addEventListener('keydown', e => { if (e.key === 'Enter') doHeroSearch(); });
    heroBtn.addEventListener('click', doHeroSearch);

    await loadData();

    const unsub = onLiveEvent(event => {
        if (event.type === 'NewBlock') loadData();
    });

    return unsub;
}

async function loadData() {
    try {
        const [status, blocksResp] = await Promise.all([
            api.get('/api/status'),
            api.get('/api/blocks?limit=8'),
        ]);
        renderStats(status);
        renderBlocks(blocksResp.blocks);
        renderNetworkInfo(status);
    } catch (e) {
        console.error('Dashboard load error:', e);
    }
}

function renderStats(s) {
    document.getElementById('stats-grid').innerHTML = `
        <div class="stat-card">
            <div class="stat-icon blue">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/></svg>
            </div>
            <div class="stat-content">
                <div class="stat-value">${formatNumber(s.block_height)}</div>
                <div class="stat-label">Block Height</div>
            </div>
        </div>
        <div class="stat-card">
            <div class="stat-icon green">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="11" width="18" height="11" rx="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>
            </div>
            <div class="stat-content">
                <div class="stat-value">${formatNumber(s.chainlock_height)}</div>
                <div class="stat-label">ChainLock Height</div>
            </div>
        </div>
        <div class="stat-card">
            <div class="stat-icon purple">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10"/></svg>
            </div>
            <div class="stat-content">
                <div class="stat-value">${formatDashShort(s.credit_pool_balance)}</div>
                <div class="stat-label">Credit Pool</div>
                <div class="stat-sub">Platform Balance</div>
            </div>
        </div>
        <div class="stat-card">
            <div class="stat-icon orange">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="m4.93 4.93 14.14 14.14"/></svg>
            </div>
            <div class="stat-content">
                <div class="stat-value">${s.masternode_count.enabled} / ${s.masternode_count.total}</div>
                <div class="stat-label">Masternodes</div>
                <div class="stat-sub">Evo: ${s.masternode_count.evo_enabled} | Regular: ${s.masternode_count.regular_enabled}</div>
            </div>
        </div>
        <div class="stat-card">
            <div class="stat-icon info">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2v20M2 12h20"/></svg>
            </div>
            <div class="stat-content">
                <div class="stat-value">${s.mempool_size}</div>
                <div class="stat-label">Mempool TXs</div>
                <div class="stat-sub">${formatNumber(s.mempool_bytes)} bytes</div>
            </div>
        </div>
        <div class="stat-card">
            <div class="stat-icon blue">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="22 12 18 12 15 21 9 3 6 12 2 12"/></svg>
            </div>
            <div class="stat-content">
                <div class="stat-value">${s.tx_rate.toFixed(4)}/s</div>
                <div class="stat-label">TX Rate</div>
            </div>
        </div>
    `;
}

function renderBlocks(blocks) {
    const rows = blocks.map(b => `
        <tr>
            <td>${blockLink(b.height)}</td>
            <td class="text-muted">${timeAgo(b.time)}</td>
            <td>${b.n_tx}</td>
            <td>${chainlockBadge(b.chainlock)}</td>
        </tr>
    `).join('');

    document.querySelector('#latest-blocks-card .card-body').innerHTML = `
        <table class="data-table">
            <thead><tr><th>Height</th><th>Time</th><th>TXs</th><th>Status</th></tr></thead>
            <tbody>${rows}</tbody>
        </table>
    `;
}

function renderNetworkInfo(s) {
    document.querySelector('#network-info-card .card-body').innerHTML = `
        <div class="detail-grid">
            <div class="detail-label">Chain</div>
            <div class="detail-value normal-font">${s.chain}</div>
            <div class="detail-label">Difficulty</div>
            <div class="detail-value">${s.difficulty.toFixed(6)}</div>
            <div class="detail-label">Best Block</div>
            <div class="detail-value">${hashLink(s.best_block_hash, 'block')}</div>
            <div class="detail-label">Credit Pool</div>
            <div class="detail-value">${formatDashShort(s.credit_pool_balance)}</div>
            <div class="detail-label">Masternodes</div>
            <div class="detail-value normal-font">${s.masternode_count.enabled} enabled / ${s.masternode_count.total} total</div>
            <div class="detail-label">Evo Nodes</div>
            <div class="detail-value normal-font">${s.masternode_count.evo_enabled} enabled / ${s.masternode_count.evo_total} total</div>
        </div>
    `;
}
