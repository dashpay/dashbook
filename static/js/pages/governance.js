import { api } from '../api.js';
import { formatDashShort, formatNumber, formatTime, blockLink, fullAddressLink, showLoading, escapeHtml } from '../components.js';

export async function renderGovernance() {
    const app = document.getElementById('app');
    showLoading(app);

    try {
        const data = await api.get('/api/governance');
        const info = data.info;

        app.innerHTML = `
            <div class="page-title"><h1>Governance</h1></div>

            <div class="stats-grid" style="margin-bottom:1.5rem">
                <div class="stat-card">
                    <div class="stat-content">
                        <div class="stat-value">${formatDashShort(info.governance_budget)}</div>
                        <div class="stat-label">Budget Available</div>
                    </div>
                </div>
                <div class="stat-card">
                    <div class="stat-content">
                        <div class="stat-value">${blockLink(info.next_superblock)}</div>
                        <div class="stat-label">Next Superblock</div>
                    </div>
                </div>
                <div class="stat-card">
                    <div class="stat-content">
                        <div class="stat-value">${info.superblock_cycle}</div>
                        <div class="stat-label">Superblock Cycle</div>
                    </div>
                </div>
                <div class="stat-card">
                    <div class="stat-content">
                        <div class="stat-value">${info.funding_threshold}</div>
                        <div class="stat-label">Funding Threshold</div>
                        <div class="stat-sub">Required votes</div>
                    </div>
                </div>
            </div>

            <div class="section-header">
                <h2>Proposals (${data.proposals.length})</h2>
            </div>

            ${data.proposals.length === 0
                ? '<div class="empty-message">No active proposals</div>'
                : data.proposals.map(p => renderProposal(p, info.funding_threshold)).join('')
            }
        `;
    } catch (e) {
        app.innerHTML = `<div class="error-message"><h2>Error</h2><p>${escapeHtml(e.message)}</p></div>`;
    }
}

function renderProposal(p, threshold) {
    const totalVotes = p.yes_count + p.no_count + p.abstain_count;
    const yesPercent = totalVotes > 0 ? (p.yes_count / totalVotes) * 100 : 0;
    const noPercent = totalVotes > 0 ? (p.no_count / totalVotes) * 100 : 0;
    const funded = p.is_funded;

    return `
        <div class="proposal-card">
            <div class="proposal-header">
                <div>
                    <div class="proposal-name">${escapeHtml(p.name)}</div>
                    <div style="margin-top:0.25rem">
                        ${funded ? '<span class="badge badge-enabled">Funded</span>' : '<span class="badge badge-standard">Not Funded</span>'}
                        ${p.is_valid ? '' : '<span class="badge badge-banned">Invalid</span>'}
                    </div>
                </div>
                <div style="text-align:right">
                    <div class="mono" style="font-size:1.1rem;font-weight:700;color:var(--color-purple)">${formatDashShort(p.payment_amount)}</div>
                    <div class="text-muted" style="font-size:0.85rem">per superblock</div>
                </div>
            </div>

            <div class="proposal-votes">
                <span class="vote-count vote-yes">&#x2713; ${p.yes_count}</span>
                <span class="vote-count vote-no">&#x2717; ${p.no_count}</span>
                <span class="vote-count vote-abstain">&#x25CB; ${p.abstain_count}</span>
                <span class="text-muted" style="font-size:0.85rem;margin-left:auto">Net: ${p.absolute_yes_count} / ${threshold} needed</span>
            </div>

            <div class="vote-bar">
                <div class="vote-bar-fill" style="width:${yesPercent}%;background:var(--color-success)"></div>
            </div>

            <div class="detail-grid" style="margin-top:0.75rem;font-size:0.85rem">
                <div class="detail-label">URL</div>
                <div class="detail-value normal-font"><a href="${escapeHtml(p.url)}" target="_blank" rel="noopener">${escapeHtml(p.url)}</a></div>
                <div class="detail-label">Payment Address</div>
                <div class="detail-value">${fullAddressLink(p.payment_address)}</div>
                <div class="detail-label">Created</div>
                <div class="detail-value normal-font">${formatTime(p.creation_time)}</div>
            </div>
        </div>
    `;
}

function escapeHtml(str) {
    const div = document.createElement('div');
    div.textContent = str;
    return div.innerHTML;
}
