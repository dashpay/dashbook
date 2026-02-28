import { api } from '../api.js';
import { txTypeBadge, instantSendBadge, formatDashValue, formatNumber, hashLink, timeAgo, showLoading, escapeHtml } from '../components.js';
import { onLiveEvent } from '../live.js';

export async function renderMempool() {
    const app = document.getElementById('app');
    showLoading(app);

    try {
        const mempool = await api.get('/api/mempool');

        app.innerHTML = `
            <div class="breadcrumb"><a href="#/">Home</a> / Mempool</div>
            <div class="page-title">
                <h1>Mempool</h1>
                <span class="badge badge-standard">${mempool.size} transaction${mempool.size !== 1 ? 's' : ''}</span>
            </div>

            <div class="stats-grid" style="margin-bottom:1.5rem">
                <div class="stat-card">
                    <div class="stat-content">
                        <div class="stat-value">${mempool.size}</div>
                        <div class="stat-label">Transactions</div>
                    </div>
                </div>
                <div class="stat-card">
                    <div class="stat-content">
                        <div class="stat-value">${formatNumber(mempool.bytes)}</div>
                        <div class="stat-label">Size (bytes)</div>
                    </div>
                </div>
                <div class="stat-card">
                    <div class="stat-content">
                        <div class="stat-value">${formatDashValue(mempool.total_fee)} DASH</div>
                        <div class="stat-label">Total Fees</div>
                    </div>
                </div>
            </div>

            <div class="card">
                <div class="card-header"><h3>Pending Transactions</h3></div>
                <div class="card-body" id="mempool-txs">
                    ${renderTxList(mempool.transactions)}
                </div>
            </div>
        `;
    } catch (e) {
        app.innerHTML = `<div class="error-message"><h2>Error</h2><p>${escapeHtml(e.message)}</p></div>`;
    }

    const unsub = onLiveEvent(event => {
        if (event.type === 'MempoolUpdate') {
            renderMempool();
        }
    });

    return unsub;
}

function renderTxList(txids) {
    if (!txids || txids.length === 0) {
        return '<p class="text-muted" style="padding:1rem">No pending transactions</p>';
    }

    const rows = txids.map(txid => `
        <tr>
            <td>${hashLink(txid)}</td>
        </tr>
    `).join('');

    return `
        <table class="data-table">
            <thead><tr><th>TxID</th></tr></thead>
            <tbody>${rows}</tbody>
        </table>
    `;
}
