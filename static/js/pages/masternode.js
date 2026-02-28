import { api } from '../api.js';
import { mnStatusBadge, mnTypeBadge, formatNumber, hashLink, fullAddressLink, blockLink, showLoading } from '../components.js';

export async function renderMasternode({ hash }) {
    const app = document.getElementById('app');
    showLoading(app);

    try {
        const mn = await api.get(`/api/masternode/${hash}`);

        const revReasons = { 0: 'Not revoked', 1: 'Compromised', 2: 'Key change', 3: 'Upgrade' };
        const status = mn.pose_ban_height > 0 ? 'POSE_BANNED' : 'ENABLED';

        app.innerHTML = `
            <div class="breadcrumb"><a href="#/masternodes">Masternodes</a> / Detail</div>
            <div class="page-title">
                <h1>Masternode</h1>
                ${mnTypeBadge(mn.mn_type)}
                ${mnStatusBadge(status)}
            </div>

            <div class="card" style="margin-bottom:1.5rem">
                <div class="card-header"><h3>Registration</h3></div>
                <div class="card-body">
                    <div class="detail-grid">
                        <div class="detail-label">ProTx Hash</div>
                        <div class="detail-value">${mn.pro_tx_hash}</div>
                        <div class="detail-label">Type</div>
                        <div class="detail-value normal-font">${mn.mn_type}${mn.mn_type === 'Evo' ? ' (HPMN - 4000 DASH)' : ' (1000 DASH)'}</div>
                        <div class="detail-label">Collateral TX</div>
                        <div class="detail-value">${hashLink(mn.collateral_hash)}:${mn.collateral_index}</div>
                        <div class="detail-label">Collateral Addr</div>
                        <div class="detail-value">${fullAddressLink(mn.collateral_address)}</div>
                        <div class="detail-label">Registered</div>
                        <div class="detail-value">${blockLink(mn.registered_height)}</div>
                        <div class="detail-label">Confirmations</div>
                        <div class="detail-value">${formatNumber(mn.confirmations)}</div>
                        <div class="detail-label">Operator Reward</div>
                        <div class="detail-value">${mn.operator_reward}%</div>
                    </div>
                </div>
            </div>

            <div class="card" style="margin-bottom:1.5rem">
                <div class="card-header"><h3>Network & Service</h3></div>
                <div class="card-body">
                    <div class="detail-grid">
                        <div class="detail-label">Service</div>
                        <div class="detail-value">${mn.service}</div>
                        <div class="detail-label">Owner Address</div>
                        <div class="detail-value">${fullAddressLink(mn.owner_address)}</div>
                        <div class="detail-label">Voting Address</div>
                        <div class="detail-value">${fullAddressLink(mn.voting_address)}</div>
                        <div class="detail-label">Payout Address</div>
                        <div class="detail-value">${fullAddressLink(mn.payout_address)}</div>
                        <div class="detail-label">Operator Key</div>
                        <div class="detail-value" style="font-size:0.85rem">${mn.pub_key_operator}</div>
                        ${mn.platform_node_id ? `
                            <div class="detail-label">Platform Node ID</div>
                            <div class="detail-value">${mn.platform_node_id}</div>
                            <div class="detail-label">Platform HTTP</div>
                            <div class="detail-value">Port ${mn.platform_http_port}</div>
                            <div class="detail-label">Platform P2P</div>
                            <div class="detail-value">Port ${mn.platform_p2p_port}</div>
                        ` : ''}
                    </div>
                </div>
            </div>

            <div class="card" style="margin-bottom:1.5rem">
                <div class="card-header"><h3>Proof of Service</h3></div>
                <div class="card-body">
                    <div class="detail-grid">
                        <div class="detail-label">PoSe Penalty</div>
                        <div class="detail-value ${mn.pose_penalty > 0 ? 'text-warning' : ''}">${mn.pose_penalty}</div>
                        <div class="detail-label">PoSe Ban Height</div>
                        <div class="detail-value ${mn.pose_ban_height > 0 ? 'text-danger' : ''}">${mn.pose_ban_height > 0 ? blockLink(mn.pose_ban_height) : 'Not banned'}</div>
                        <div class="detail-label">PoSe Revived</div>
                        <div class="detail-value">${mn.pose_revived_height > 0 ? blockLink(mn.pose_revived_height) : 'N/A'}</div>
                        <div class="detail-label">Revocation</div>
                        <div class="detail-value normal-font">${revReasons[mn.revocation_reason] || 'Unknown'}</div>
                        <div class="detail-label">Platform Banned</div>
                        <div class="detail-value normal-font">${mn.is_platform_banned ? '<span class="text-danger">Yes</span>' : 'No'}</div>
                    </div>
                </div>
            </div>

            <div class="card">
                <div class="card-header"><h3>Payment & Mixing</h3></div>
                <div class="card-body">
                    <div class="detail-grid">
                        <div class="detail-label">Last Paid</div>
                        <div class="detail-value">${mn.last_paid_height > 0 ? blockLink(mn.last_paid_height) : 'Never'}</div>
                        <div class="detail-label">Consecutive Pays</div>
                        <div class="detail-value">${mn.consecutive_payments}</div>
                        <div class="detail-label">Mixing TX Count</div>
                        <div class="detail-value">${mn.mixing_tx_count}</div>
                        <div class="detail-label">Last DSQ</div>
                        <div class="detail-value">${mn.last_dsq}</div>
                    </div>
                </div>
            </div>
        `;
    } catch (e) {
        app.innerHTML = `<div class="error-message"><h2>Error</h2><p>${e.message}</p></div>`;
    }
}
