import { api } from '../api.js';
import { formatDashShort, formatDashValue, formatNumber, hashLink, blockLink, showLoading, renderPagination, bindPagination } from '../components.js';

let currentPage = 1;
let currentAddress = '';

export async function renderAddress({ addr }) {
    currentAddress = addr;
    currentPage = 1;
    const app = document.getElementById('app');
    showLoading(app);

    await loadAddress(addr, 1);
}

async function loadAddress(addr, page) {
    const app = document.getElementById('app');
    try {
        const data = await api.get(`/api/address/${addr}?page=${page}&limit=50`);

        app.innerHTML = `
            <div class="breadcrumb"><a href="#/">Home</a> / Address</div>
            <div class="page-title"><h1>Address</h1></div>
            <p class="mono" style="margin-bottom:1.5rem;word-break:break-all;color:var(--text-accent)">${data.address}</p>

            <div class="address-balance">
                <div class="stat-card">
                    <div class="stat-content">
                        <div class="stat-value">${formatDashShort(data.balance)}</div>
                        <div class="stat-label">Balance</div>
                    </div>
                </div>
                <div class="stat-card">
                    <div class="stat-content">
                        <div class="stat-value">${formatDashShort(data.total_received)}</div>
                        <div class="stat-label">Total Received</div>
                    </div>
                </div>
                <div class="stat-card">
                    <div class="stat-content">
                        <div class="stat-value">${formatNumber(data.tx_count)}</div>
                        <div class="stat-label">Transactions</div>
                    </div>
                </div>
                <div class="stat-card">
                    <div class="stat-content">
                        <div class="stat-value">${formatNumber(data.utxos.length)}</div>
                        <div class="stat-label">UTXOs</div>
                    </div>
                </div>
            </div>

            <div class="card" style="margin-bottom:1.5rem">
                <div class="card-header"><h3>Transactions</h3></div>
                <div class="card-body">
                    <table class="data-table">
                        <thead><tr><th>TxID</th><th>Block</th><th>Amount</th></tr></thead>
                        <tbody>
                            ${data.transactions.map(tx => {
                                const cls = tx.delta_sat > 0 ? 'positive' : tx.delta_sat < 0 ? 'negative' : 'neutral';
                                const prefix = tx.delta_sat > 0 ? '+' : '';
                                return `
                                    <tr>
                                        <td>${hashLink(tx.txid)}</td>
                                        <td>${blockLink(tx.height)}</td>
                                        <td class="tx-io-amount ${cls} mono">${prefix}${formatDashValue(tx.delta)} DASH</td>
                                    </tr>
                                `;
                            }).join('')}
                        </tbody>
                    </table>
                </div>
            </div>
            <div id="addr-pagination"></div>
        `;

        const totalPages = Math.ceil(data.tx_count / 50);
        document.getElementById('addr-pagination').innerHTML = renderPagination(page, Math.min(totalPages, 200));
        bindPagination(document.getElementById('addr-pagination'), (p) => loadAddress(addr, p));
    } catch (e) {
        app.innerHTML = `<div class="error-message"><h2>Error</h2><p>${e.message}</p></div>`;
    }
}
