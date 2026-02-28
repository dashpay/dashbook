import { api } from '../api.js';
import { chainlockBadge, txTypeBadge, instantSendBadge, formatDashValue, formatDashShort, formatTime, formatNumber, hashLink, showLoading, escapeHtml } from '../components.js';

export async function renderBlockDetail({ id }) {
    const app = document.getElementById('app');
    showLoading(app);

    try {
        const block = await api.get(`/api/block/${id}`);

        app.innerHTML = `
            <div class="breadcrumb"><a href="#/blocks">Blocks</a> / ${block.height}</div>
            <div class="block-nav">
                ${block.previous_block_hash ? `<a href="#/block/${block.height - 1}">&larr; Block ${block.height - 1}</a>` : '<span></span>'}
                ${block.next_block_hash ? `<a href="#/block/${block.height + 1}">Block ${block.height + 1} &rarr;</a>` : '<span></span>'}
            </div>
            <div class="page-title">
                <h1>Block ${formatNumber(block.height)}</h1>
                ${chainlockBadge(block.chainlock)}
            </div>

            <div class="card" style="margin-bottom:1.5rem">
                <div class="card-header"><h3>Block Details</h3></div>
                <div class="card-body">
                    <div class="detail-grid">
                        <div class="detail-label">Hash</div>
                        <div class="detail-value">${block.hash}</div>
                        <div class="detail-label">Height</div>
                        <div class="detail-value">${formatNumber(block.height)}</div>
                        <div class="detail-label">Timestamp</div>
                        <div class="detail-value normal-font">${formatTime(block.time)}</div>
                        <div class="detail-label">Confirmations</div>
                        <div class="detail-value">${formatNumber(block.confirmations)}</div>
                        <div class="detail-label">Transactions</div>
                        <div class="detail-value">${block.n_tx}</div>
                        <div class="detail-label">Size</div>
                        <div class="detail-value">${formatNumber(block.size)} bytes</div>
                        <div class="detail-label">Difficulty</div>
                        <div class="detail-value">${block.difficulty.toFixed(6)}</div>
                        <div class="detail-label">Merkle Root</div>
                        <div class="detail-value">${block.merkle_root}</div>
                        <div class="detail-label">Nonce</div>
                        <div class="detail-value">${block.nonce}</div>
                        <div class="detail-label">Bits</div>
                        <div class="detail-value">${block.bits}</div>
                    </div>
                </div>
            </div>

            ${block.cb_tx ? renderCbTx(block.cb_tx) : ''}

            <div class="card">
                <div class="card-header"><h3>Transactions (${block.n_tx})</h3></div>
                <div class="card-body">
                    <table class="data-table">
                        <thead><tr><th>TxID</th><th>Type</th><th>Size</th><th>Output</th><th>IS</th></tr></thead>
                        <tbody>
                            ${block.transactions.map(tx => `
                                <tr>
                                    <td>${hashLink(tx.txid)}</td>
                                    <td>${txTypeBadge(tx.tx_type)}</td>
                                    <td>${tx.size}</td>
                                    <td class="mono">${formatDashValue(tx.total_output)} DASH</td>
                                    <td>${instantSendBadge(tx.instantlock, false)}</td>
                                </tr>
                            `).join('')}
                        </tbody>
                    </table>
                </div>
            </div>
        `;
    } catch (e) {
        app.innerHTML = `<div class="error-message"><h2>Error</h2><p>${escapeHtml(e.message)}</p></div>`;
    }
}

function renderCbTx(cb) {
    return `
        <div class="card" style="margin-bottom:1.5rem">
            <div class="card-header"><h3>Coinbase Transaction (DIP-4)</h3></div>
            <div class="card-body">
                <div class="detail-grid">
                    <div class="detail-label">Credit Pool</div>
                    <div class="detail-value" style="color:var(--color-purple)">${formatDashShort(cb.credit_pool_balance)}</div>
                    <div class="detail-label">MN List Root</div>
                    <div class="detail-value">${cb.merkle_root_mn_list}</div>
                    <div class="detail-label">Quorums Root</div>
                    <div class="detail-value">${cb.merkle_root_quorums}</div>
                    <div class="detail-label">CL Height Diff</div>
                    <div class="detail-value">${cb.best_cl_height_diff}</div>
                </div>
            </div>
        </div>
    `;
}
