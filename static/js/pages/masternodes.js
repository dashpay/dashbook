import { api } from '../api.js';
import { mnStatusBadge, mnTypeBadge, formatNumber, hashLink, blockLink, addressLink, showLoading, renderPagination, bindPagination } from '../components.js';

let currentType = 'all';
let currentStatus = 'all';
let currentPage = 1;

export async function renderMasternodes() {
    currentPage = 1;
    currentType = 'all';
    currentStatus = 'all';

    const app = document.getElementById('app');
    app.innerHTML = `
        <div class="page-title"><h1>Masternodes</h1></div>
        <div class="filter-bar" id="mn-filters">
            <button class="filter-btn active" data-type="all">All</button>
            <button class="filter-btn" data-type="evo">Evo</button>
            <button class="filter-btn" data-type="regular">Regular</button>
            <span style="width:1px;background:var(--border-color);margin:0 0.5rem"></span>
            <button class="filter-btn active" data-status="all">All Status</button>
            <button class="filter-btn" data-status="enabled">Enabled</button>
            <button class="filter-btn" data-status="pose_banned">Banned</button>
        </div>
        <div class="card">
            <div class="card-body" id="mn-content"><div class="spinner"></div></div>
        </div>
        <div id="mn-pagination"></div>
    `;

    // Filter handlers
    document.getElementById('mn-filters').addEventListener('click', (e) => {
        const btn = e.target.closest('.filter-btn');
        if (!btn) return;

        if (btn.dataset.type) {
            currentType = btn.dataset.type;
            document.querySelectorAll('[data-type]').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
        }
        if (btn.dataset.status) {
            currentStatus = btn.dataset.status;
            document.querySelectorAll('[data-status]').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
        }
        currentPage = 1;
        loadMasternodes();
    });

    await loadMasternodes();
}

async function loadMasternodes() {
    showLoading('mn-content');
    try {
        const data = await api.get(`/api/masternodes?type=${currentType}&status=${currentStatus}&page=${currentPage}&limit=50`);

        const rows = data.masternodes.map(mn => `
            <tr>
                <td><a href="#/masternode/${mn.pro_tx_hash}" class="hash-link mono">${mn.pro_tx_hash.slice(0, 12)}...</a></td>
                <td>${mnTypeBadge(mn.mn_type)}</td>
                <td>${mnStatusBadge(mn.status)}</td>
                <td class="mono" style="font-size:0.85rem">${mn.service}</td>
                <td>${mn.pose_penalty > 0 ? `<span class="text-warning">${mn.pose_penalty}</span>` : '<span class="text-muted">0</span>'}</td>
                <td>${mn.last_paid_block > 0 ? blockLink(mn.last_paid_block) : '<span class="text-muted">Never</span>'}</td>
                ${mn.platform_node_id ? `<td class="mono" style="font-size:0.85rem">${mn.platform_node_id.slice(0, 10)}...</td>` : '<td class="text-muted">-</td>'}
            </tr>
        `).join('');

        document.getElementById('mn-content').innerHTML = `
            <table class="data-table">
                <thead><tr><th>ProTx Hash</th><th>Type</th><th>Status</th><th>Service</th><th>PoSe</th><th>Last Paid</th><th>Platform ID</th></tr></thead>
                <tbody>${rows}</tbody>
            </table>
        `;

        document.getElementById('mn-pagination').innerHTML = renderPagination(data.page, data.pages);
        bindPagination(document.getElementById('mn-pagination'), (p) => {
            currentPage = p;
            loadMasternodes();
        });
    } catch (e) {
        document.getElementById('mn-content').innerHTML = `<div class="error-message">${e.message}</div>`;
    }
}
