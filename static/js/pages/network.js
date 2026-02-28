import { api } from '../api.js';
import { formatDashShort, formatNumber, formatTime, blockLink, hashLink, showLoading, escapeHtml } from '../components.js';

export async function renderNetwork() {
    const app = document.getElementById('app');
    showLoading(app);

    try {
        const data = await api.get('/api/network');

        app.innerHTML = `
            <div class="page-title"><h1>Network</h1></div>

            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-content">
                        <div class="stat-value">${data.core_version}</div>
                        <div class="stat-label">Core Version</div>
                    </div>
                </div>
                <div class="stat-card">
                    <div class="stat-content">
                        <div class="stat-value">${data.protocol_version}</div>
                        <div class="stat-label">Protocol Version</div>
                    </div>
                </div>
                <div class="stat-card">
                    <div class="stat-content">
                        <div class="stat-value">${data.connections}</div>
                        <div class="stat-label">Connections</div>
                        <div class="stat-sub">MN: ${data.connections_mn}</div>
                    </div>
                </div>
                <div class="stat-card">
                    <div class="stat-content">
                        <div class="stat-value">${formatNumber(data.tx_count)}</div>
                        <div class="stat-label">Total Transactions</div>
                    </div>
                </div>
            </div>

            <div class="dashboard-columns">
                <div class="card">
                    <div class="card-header"><h3>Blockchain</h3></div>
                    <div class="card-body">
                        <div class="detail-grid">
                            <div class="detail-label">Chain</div>
                            <div class="detail-value normal-font">${data.chain}</div>
                            <div class="detail-label">Block Height</div>
                            <div class="detail-value">${blockLink(data.block_height)}</div>
                            <div class="detail-label">Best Block</div>
                            <div class="detail-value">${hashLink(data.best_block_hash, 'block')}</div>
                            <div class="detail-label">Difficulty</div>
                            <div class="detail-value">${data.difficulty.toFixed(6)}</div>
                            <div class="detail-label">TX Rate</div>
                            <div class="detail-value">${data.tx_rate.toFixed(4)} tx/s</div>
                            <div class="detail-label">Credit Pool</div>
                            <div class="detail-value" style="color:var(--color-purple)">${formatDashShort(data.credit_pool_balance)}</div>
                        </div>
                    </div>
                </div>

                <div class="card">
                    <div class="card-header"><h3>ChainLock</h3></div>
                    <div class="card-body">
                        <div class="detail-grid">
                            <div class="detail-label">Height</div>
                            <div class="detail-value">${blockLink(data.chainlock_height)}</div>
                            <div class="detail-label">Block Hash</div>
                            <div class="detail-value">${hashLink(data.chainlock_hash, 'block')}</div>
                        </div>

                        <h3 style="margin-top:1.5rem;margin-bottom:0.75rem">Masternodes</h3>
                        <div class="detail-grid">
                            <div class="detail-label">Total</div>
                            <div class="detail-value normal-font">${data.masternode_count.total}</div>
                            <div class="detail-label">Enabled</div>
                            <div class="detail-value normal-font">${data.masternode_count.enabled}</div>
                            <div class="detail-label">Regular</div>
                            <div class="detail-value normal-font">${data.masternode_count.regular_enabled} / ${data.masternode_count.regular_total}</div>
                            <div class="detail-label">Evo (HPMN)</div>
                            <div class="detail-value normal-font">${data.masternode_count.evo_enabled} / ${data.masternode_count.evo_total}</div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="card" style="margin-top:1.5rem">
                <div class="card-header"><h3>Mempool</h3></div>
                <div class="card-body">
                    <div class="detail-grid">
                        <div class="detail-label">Size</div>
                        <div class="detail-value">${data.mempool_size} transactions</div>
                        <div class="detail-label">Bytes</div>
                        <div class="detail-value">${formatNumber(data.mempool_bytes)} bytes</div>
                        <div class="detail-label">Total Fee</div>
                        <div class="detail-value">${data.mempool_total_fee} DASH</div>
                    </div>
                </div>
            </div>
        `;
    } catch (e) {
        app.innerHTML = `<div class="error-message"><h2>Error</h2><p>${escapeHtml(e.message)}</p></div>`;
    }
}
