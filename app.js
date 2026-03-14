let accounts = [];
let emails = [];
let currentView = 'inbox';
let selectedEmail = null;
let selectedAccount = null;

const views = document.querySelectorAll('.view');
const navBtns = document.querySelectorAll('.nav-btn');
const accountSelect = document.getElementById('accountSelect');
const emailList = document.getElementById('emailList');
const accountModal = document.getElementById('accountModal');

document.addEventListener('DOMContentLoaded', () => {
    loadAccounts();
    setupEventListeners();
});

function setupEventListeners() {
    navBtns.forEach(btn => {
        btn.addEventListener('click', () => switchView(btn.dataset.view));
    });
    accountSelect.addEventListener('change', (e) => {
        selectedAccount = e.target.value;
        if (selectedAccount) loadEmails(selectedAccount);
    });
    document.getElementById('refreshBtn').addEventListener('click', () => {
        if (selectedAccount) loadEmails(selectedAccount);
    });
    document.getElementById('backToList').addEventListener('click', () => switchView('inbox'));
    document.getElementById('composeForm').addEventListener('submit', sendEmail);
    document.getElementById('addAccountBtn').addEventListener('click', () => accountModal.classList.add('active'));
    document.getElementById('closeModal').addEventListener('click', () => accountModal.classList.remove('active'));
    document.getElementById('accountForm').addEventListener('submit', saveAccount);
}

function switchView(viewName) {
    currentView = viewName;
    views.forEach(v => v.classList.remove('active'));
    document.getElementById(viewName + 'View').classList.add('active');
    navBtns.forEach(btn => btn.classList.toggle('active', btn.dataset.view === viewName));
    if (viewName === 'accounts') renderAccounts();
}

async function loadAccounts() {
    try {
        const result = await window.__TAURI__.core.invoke('get_accounts');
        accounts = result || [];
        updateAccountSelector();
        renderAccounts();
    } catch (e) { console.log('No accounts:', e); }
}

function updateAccountSelector() {
    accountSelect.innerHTML = '<option value="">Select Account</option>';
    accounts.forEach(a => {
        const opt = document.createElement('option');
        opt.value = a.id;
        opt.textContent = a.email;
        accountSelect.appendChild(opt);
    });
}

async function saveAccount(e) {
    e.preventDefault();
    const acc = {
        email: document.getElementById('accountEmail').value,
        password: document.getElementById('accountPassword').value,
        imap_host: document.getElementById('imapHost').value,
        imap_port: parseInt(document.getElementById('imapPort').value),
        smtp_host: document.getElementById('smtpHost').value,
        smtp_port: parseInt(document.getElementById('smtpPort').value),
    };
    try {
        await window.__TAURI__.core.invoke('add_account', acc);
        loadAccounts();
        accountModal.classList.remove('active');
        document.getElementById('accountForm').reset();
    } catch (e) { alert('Error: ' + e); }
}

async function deleteAccount(id) {
    if (!confirm('Delete this account?')) return;
    try {
        await window.__TAURI__.core.invoke('delete_account', { id });
        loadAccounts();
    } catch (e) { alert('Error: ' + e); }
}

function renderAccounts() {
    const list = document.getElementById('accountsList');
    if (accounts.length === 0) {
        list.innerHTML = '<div class="empty-state"><p>No accounts</p></div>';
        return;
    }
    list.innerHTML = accounts.map(a => `
        <div class="account-card">
            <div class="account-info"><h4>${a.email}</h4><p>${a.imap_host}</p></div>
            <button class="btn" onclick="deleteAccount('${a.id}')">Delete</button>
        </div>
    `).join('');
}

async function loadEmails(accountId) {
    try {
        emailList.innerHTML = '<div class="empty-state"><p>Loading...</p></div>';
        const result = await window.__TAURI__.core.invoke('fetch_emails', { accountId, limit: 50 });
        emails = result || [];
        renderEmailList();
    } catch (e) {
        emailList.innerHTML = '<div class="empty-state"><p>Error loading emails</p></div>';
    }
}

function renderEmailList() {
    if (emails.length === 0) {
        emailList.innerHTML = '<div class="empty-state"><p>No emails</p></div>';
        return;
    }
    emailList.innerHTML = emails.map(e => `
        <div class="email-item ${e.read ? '' : 'unread'}" onclick="viewEmail('${e.id}')">
            <div class="sender">${e.from}</div>
            <div class="subject">${e.subject}</div>
        </div>
    `).join('');
}

async function viewEmail(id) {
    selectedEmail = emails.find(e => e.id === id);
    if (!selectedEmail) return;
    document.getElementById('detailFrom').textContent = selectedEmail.from;
    document.getElementById('detailSubject').textContent = selectedEmail.subject;
    document.getElementById('detailDate').textContent = selectedEmail.date;
    document.getElementById('detailBody').textContent = selectedEmail.body;
    switchView('emailDetail');
}

async function sendEmail(e) {
    e.preventDefault();
    if (!selectedAccount) { alert('Select an account first'); return; }
    const email = {
        to: document.getElementById('composeTo').value,
        subject: document.getElementById('composeSubject').value,
        body: document.getElementById('composeBody').value,
    };
    try {
        await window.__TAURI__.core.invoke('send_email', { accountId: selectedAccount, email });
        alert('Sent!');
        document.getElementById('composeForm').reset();
        switchView('inbox');
    } catch (e) { alert('Error: ' + e); }
}

window.viewEmail = viewEmail;
window.deleteAccount = deleteAccount;
