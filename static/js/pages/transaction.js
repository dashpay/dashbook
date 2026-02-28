import { api } from '../api.js';
import { chainlockBadge, instantSendBadge, txTypeBadge, formatDashValue, formatTime, formatNumber, hashLink, blockLink, addressLink, fullAddressLink, showLoading, escapeHtml } from '../components.js';

export async function renderTransaction({ txid }) {
    const app = document.getElementById('app');
    showLoading(app);

    try {
        const tx = await api.get(`/api/tx/${txid}`);

        const totalIn = tx.inputs.reduce((s, i) => s + (i.value || 0), 0);
        const totalOut = tx.outputs.reduce((s, o) => s + o.value, 0);

        app.innerHTML = `
            <div class="breadcrumb"><a href="#/">Home</a> / Transaction</div>
            <div class="page-title">
                <h1>Transaction</h1>
                ${txTypeBadge(tx.tx_type)}
                ${chainlockBadge(tx.chainlock)}
                ${instantSendBadge(tx.instantlock, tx.instantlock_internal)}
            </div>

            <div class="card" style="margin-bottom:1.5rem">
                <div class="card-header"><h3>Overview</h3></div>
                <div class="card-body">
                    <div class="detail-grid">
                        <div class="detail-label">TxID</div>
                        <div class="detail-value">${tx.txid}</div>
                        <div class="detail-label">Type</div>
                        <div class="detail-value normal-font">${tx.tx_type_label} (type ${tx.tx_type})</div>
                        <div class="detail-label">Block</div>
                        <div class="detail-value">${tx.block_hash ? blockLink(tx.block_height) : '<span class="text-muted">Unconfirmed</span>'}</div>
                        <div class="detail-label">Time</div>
                        <div class="detail-value normal-font">${tx.time ? formatTime(tx.time) : 'Pending'}</div>
                        <div class="detail-label">Confirmations</div>
                        <div class="detail-value">${tx.confirmations != null ? formatNumber(tx.confirmations) : '0'}</div>
                        <div class="detail-label">Size</div>
                        <div class="detail-value">${formatNumber(tx.size)} bytes</div>
                        <div class="detail-label">Fee</div>
                        <div class="detail-value">${tx.fee != null ? formatDashValue(tx.fee) + ' DASH' : 'N/A'}</div>
                        <div class="detail-label">InstantSend</div>
                        <div class="detail-value normal-font">${tx.instantlock ? (tx.instantlock_internal ? 'LLMQ-signed' : 'Locked') : 'No'}</div>
                        <div class="detail-label">ChainLock</div>
                        <div class="detail-value normal-font">${tx.chainlock ? 'Yes' : 'No'}</div>
                    </div>
                </div>
            </div>

            ${tx.special_tx_payload ? renderSpecialPayload(tx.special_tx_payload) : ''}

            <div class="card">
                <div class="card-header"><h3>Inputs & Outputs</h3></div>
                <div class="card-body">
                    <div class="tx-io">
                        <div>
                            <h4 style="margin-bottom:0.5rem;color:var(--text-muted);font-size:0.85rem">INPUTS (${tx.inputs.length})</h4>
                            <ul class="tx-io-list">
                                ${tx.inputs.map(input => `
                                    <li class="tx-io-item">
                                        <div>
                                            ${input.is_coinbase
                                                ? '<span class="badge badge-coinbase">Coinbase</span>'
                                                : `${input.address ? addressLink(input.address) : hashLink(input.txid)}${input.vout != null ? `<span class="text-muted">:${input.vout}</span>` : ''}`
                                            }
                                        </div>
                                        <div class="tx-io-amount neutral">
                                            ${input.value != null ? formatDashValue(input.value) : ''}
                                        </div>
                                    </li>
                                `).join('')}
                            </ul>
                            ${!tx.inputs[0]?.is_coinbase ? `<div style="padding:0.5rem 0.75rem;font-size:0.85rem;color:var(--text-muted)">Total: ${formatDashValue(totalIn)} DASH</div>` : ''}
                        </div>
                        <div class="tx-io-arrow">&rarr;</div>
                        <div>
                            <h4 style="margin-bottom:0.5rem;color:var(--text-muted);font-size:0.85rem">OUTPUTS (${tx.outputs.length})</h4>
                            <ul class="tx-io-list">
                                ${tx.outputs.map(output => `
                                    <li class="tx-io-item">
                                        <div>
                                            ${output.address ? addressLink(output.address) : `<span class="text-muted">${output.script_type}</span>`}
                                            ${output.script_type === 'nulldata' && output.script_asm ? `<div class="op-return-data">${decodeOpReturn(output.script_asm)}</div>` : ''}
                                            ${output.is_spent ? `<span class="tx-io-spent">(spent in ${hashLink(output.spent_tx_id)})</span>` : ''}
                                        </div>
                                        <div class="tx-io-amount positive">
                                            ${formatDashValue(output.value)}
                                        </div>
                                    </li>
                                `).join('')}
                            </ul>
                            <div style="padding:0.5rem 0.75rem;font-size:0.85rem;color:var(--text-muted)">Total: ${formatDashValue(totalOut)} DASH</div>
                        </div>
                    </div>
                </div>
            </div>
        `;
    } catch (e) {
        app.innerHTML = `<div class="error-message"><h2>Error</h2><p>${escapeHtml(e.message)}</p></div>`;
    }
}

function renderSpecialPayload(payload) {
    const type = payload.type;
    const data = payload.data;

    let fields = '';
    if (type === 'ProRegTx') {
        const revReason = { 0: 'Not revoked', 1: 'Compromised', 2: 'Key change', 3: 'Upgrade' };
        fields = `
            ${pf('Version', data.version)}
            ${pf('MN Type', data.type === 1 ? 'Evo (HPMN)' : 'Regular')}
            ${pf('Collateral', `${hashLink(data.collateralHash)}:${data.collateralIndex}`)}
            ${pf('Service', data.service)}
            ${pf('Owner Address', fullAddressLink(data.ownerAddress))}
            ${pf('Voting Address', fullAddressLink(data.votingAddress))}
            ${pf('Payout Address', fullAddressLink(data.payoutAddress))}
            ${pf('Operator Reward', data.operatorReward + '%')}
            ${data.platformNodeID ? pf('Platform Node ID', data.platformNodeID) : ''}
            ${data.platformP2PPort ? pf('Platform P2P Port', data.platformP2PPort) : ''}
            ${data.platformHTTPPort ? pf('Platform HTTP Port', data.platformHTTPPort) : ''}
        `;
    } else if (type === 'CbTx') {
        fields = `
            ${pf('Height', data.height)}
            ${pf('Credit Pool Balance', formatDashValue(data.creditPoolBalance) + ' DASH')}
            ${pf('MN List Root', data.merkleRootMNList)}
            ${pf('Quorums Root', data.merkleRootQuorums)}
            ${pf('CL Height Diff', data.bestCLHeightDiff)}
        `;
    } else if (type === 'QcTx') {
        const c = data.commitment;
        fields = `
            ${pf('Height', data.height)}
            ${pf('LLMQ Type', c.llmqType)}
            ${pf('Quorum Hash', c.quorumHash)}
            ${pf('Signers', `${c.signersCount} members`)}
            ${pf('Valid Members', c.validMembersCount)}
            ${pf('Quorum Public Key', c.quorumPublicKey)}
        `;
    } else if (type === 'AssetUnlockTx') {
        fields = `
            ${pf('Index', data.index)}
            ${pf('Fee', data.fee + ' duffs')}
            ${pf('Requested Height', data.requestedHeight)}
            ${pf('Quorum Hash', data.quorumHash)}
        `;
    } else if (type === 'ProUpServTx' || type === 'ProUpRegTx' || type === 'ProUpRevTx') {
        fields = Object.entries(data).map(([k, v]) => pf(k, typeof v === 'object' ? JSON.stringify(v) : v)).join('');
    }

    return `
        <div class="card" style="margin-bottom:1.5rem">
            <div class="card-header"><h3>Special Payload: ${type}</h3></div>
            <div class="card-body">
                <div class="payload-section">${fields}</div>
            </div>
        </div>
    `;
}

function pf(key, val) {
    return `<div class="payload-field"><span class="payload-key">${key}</span><span class="payload-val">${val}</span></div>`;
}

function decodeOpReturn(asm) {
    const parts = asm.split(' ');
    if (parts[0] !== 'OP_RETURN') return `<span class="mono text-muted">${asm}</span>`;
    const hexData = parts.slice(1).join(' ');
    let decoded = '';
    try {
        const hex = hexData.replace(/\s/g, '');
        for (let i = 0; i < hex.length; i += 2) {
            const code = parseInt(hex.substr(i, 2), 16);
            decoded += (code >= 32 && code < 127) ? String.fromCharCode(code) : '.';
        }
    } catch (e) { decoded = ''; }
    return `<span class="badge badge-standard">OP_RETURN</span> <span class="mono text-muted" style="font-size:0.8rem">${hexData}</span>${decoded ? `<div class="op-return-decoded mono">${decoded}</div>` : ''}`;
}
