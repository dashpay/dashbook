import { Router } from './router.js';
import { initSearch } from './components.js';
import { initLive } from './live.js';

import { renderDashboard } from './pages/dashboard.js';
import { renderBlocks } from './pages/blocks.js';
import { renderBlockDetail } from './pages/block-detail.js';
import { renderTransaction } from './pages/transaction.js';
import { renderAddress } from './pages/address.js';
import { renderMasternodes } from './pages/masternodes.js';
import { renderMasternode } from './pages/masternode.js';
import { renderMempool } from './pages/mempool.js';
import { renderGovernance } from './pages/governance.js';
import { renderNetwork } from './pages/network.js';

const router = new Router();

router.on('/', renderDashboard);
router.on('/blocks', renderBlocks);
router.on('/block/:id', renderBlockDetail);
router.on('/tx/:txid', renderTransaction);
router.on('/address/:addr', renderAddress);
router.on('/masternodes', renderMasternodes);
router.on('/masternode/:hash', renderMasternode);
router.on('/mempool', renderMempool);
router.on('/governance', renderGovernance);
router.on('/network', renderNetwork);

// Mobile nav toggle
const navToggle = document.getElementById('nav-toggle');
const mainNav = document.getElementById('main-nav');
if (navToggle && mainNav) {
    navToggle.addEventListener('click', () => mainNav.classList.toggle('open'));
    mainNav.addEventListener('click', (e) => {
        if (e.target.closest('.nav-link')) mainNav.classList.remove('open');
    });
}

initSearch();
initLive();
router.start();
