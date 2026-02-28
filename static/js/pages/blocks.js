import { api } from '../api.js';
import { chainlockBadge, formatNumber, timeAgo, blockLink, renderPagination, bindPagination, showLoading, escapeHtml } from '../components.js';

let currentPage = 1;

export async function renderBlocks() {
    currentPage = 1;
    const app = document.getElementById('app');
    app.innerHTML = `
        <div class="page-title"><h1>Blocks</h1></div>
        <div class="card">
            <div class="card-body" id="blocks-content"><div class="spinner"></div></div>
        </div>
        <div id="blocks-pagination"></div>
    `;
    await loadBlocks(1);
}

async function loadBlocks(page) {
    currentPage = page;
    showLoading('blocks-content');
    try {
        const data = await api.get(`/api/blocks?page=${page}&limit=25`);
        const rows = data.blocks.map(b => `
            <tr>
                <td>${blockLink(b.height)}</td>
                <td class="mono">${b.hash.slice(0, 16)}...</td>
                <td class="text-muted">${timeAgo(b.time)}</td>
                <td>${b.n_tx}</td>
                <td>${formatNumber(b.size)}</td>
                <td>${chainlockBadge(b.chainlock)}</td>
            </tr>
        `).join('');

        document.getElementById('blocks-content').innerHTML = `
            <table class="data-table">
                <thead><tr><th>Height</th><th>Hash</th><th>Time</th><th>TXs</th><th>Size</th><th>Status</th></tr></thead>
                <tbody>${rows}</tbody>
            </table>
        `;

        const totalPages = Math.min(data.pages, 1000);
        document.getElementById('blocks-pagination').innerHTML = renderPagination(page, totalPages);
        bindPagination(document.getElementById('blocks-pagination'), loadBlocks);
    } catch (e) {
        document.getElementById('blocks-content').innerHTML = `<div class="error-message">${escapeHtml(e.message)}</div>`;
    }
}
