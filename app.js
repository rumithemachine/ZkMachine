// ZK Machine - Frontend JavaScript

// Simulated wallet connection (gerçek uygulamada Freighter veya Albedo kullanılacak)
let connectedWallet = null;
let userStats = {
    adViews: 0,
    earned: 0
};

// Contract address (Stellar Testnet)
const CONTRACT_ADDRESS = "CBMH5XGPXFLU5AV4JHMPMGYC2U2OLMTEXGUV7VID76YIDFFEBLJMVVKT";
const NETWORK = "testnet";

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    setupEventListeners();
    updateStats();
});

function setupEventListeners() {
    document.getElementById('connectWallet').addEventListener('click', connectWallet);
    document.getElementById('disconnectWallet')?.addEventListener('click', disconnectWallet);
}

// Wallet bağlantısı
async function connectWallet() {
    try {
        // Simulated wallet connection
        // Gerçek uygulamada: await window.freighter.connect()
        
        const mockAddress = generateMockAddress();
        connectedWallet = mockAddress;
        
        // UI güncelle
        document.getElementById('connectWallet').classList.add('hidden');
        document.getElementById('walletInfo').classList.remove('hidden');
        document.getElementById('walletAddress').textContent = 
            mockAddress.substring(0, 8) + '...' + mockAddress.substring(mockAddress.length - 8);
        
        document.getElementById('dashboard').classList.remove('hidden');
        
        // Kullanıcıyı kaydet (ZK proof ile)
        await registerUser();
        
        showNotification('Cüzdan başarıyla bağlandı! 🎉', 'success');
    } catch (error) {
        console.error('Wallet connection error:', error);
        showNotification('Cüzdan bağlantısı başarısız oldu', 'error');
    }
}

function disconnectWallet() {
    connectedWallet = null;
    document.getElementById('connectWallet').classList.remove('hidden');
    document.getElementById('walletInfo').classList.add('hidden');
    document.getElementById('dashboard').classList.add('hidden');
    
    showNotification('Cüzdan bağlantısı kesildi', 'info');
}

// ZK Proof oluştur (basitleştirilmiş)
function generateZKProof() {
    // Gerçek uygulamada: zk-SNARKs veya zk-STARKs kullanılacak
    // Örnek: snarkjs, circom, veya Stellar'a özel ZK kütüphanesi
    
    const timestamp = Date.now();
    const randomness = Math.floor(Math.random() * 1000000);
    
    // Basit bir hash simülasyonu
    const proofHash = (timestamp + randomness) % 999999 + 1000;
    
    return {
        proofHash: proofHash,
        timestamp: timestamp,
        isValid: true
    };
}

// Kullanıcı kaydı
async function registerUser() {
    try {
        const zkProof = generateZKProof();
        
        // Smart contract çağrısı simülasyonu
        // Gerçek uygulamada: contract.register_user(connectedWallet, zkProof.proofHash)
        
        console.log('User registered with ZK proof:', zkProof);
        
        // Simulated delay
        await new Promise(resolve => setTimeout(resolve, 1000));
        
        return true;
    } catch (error) {
        console.error('Registration error:', error);
        return false;
    }
}

// Reklam izle
async function watchAd(campaignId) {
    if (!connectedWallet) {
        showNotification('Lütfen önce cüzdanınızı bağlayın', 'warning');
        return;
    }
    
    // Modal'ı aç
    const modal = document.getElementById('adModal');
    modal.classList.remove('hidden');
    
    // ZK Proof oluştur
    const zkProof = generateZKProof();
    
    // Progress bar animasyonu
    let progress = 0;
    const progressBar = document.getElementById('zkProgress');
    const progressInterval = setInterval(() => {
        progress += 5;
        progressBar.style.width = progress + '%';
        if (progress >= 100) {
            clearInterval(progressInterval);
        }
    }, 100);
    
    // Reklam timer
    let timeLeft = 15;
    const timerElement = document.getElementById('adTimer');
    const timerInterval = setInterval(() => {
        timeLeft--;
        timerElement.textContent = timeLeft;
        
        if (timeLeft <= 0) {
            clearInterval(timerInterval);
            completeAdView(campaignId, zkProof);
        }
    }, 1000);
}

async function completeAdView(campaignId, zkProof) {
    try {
        // Smart contract çağrısı simülasyonu
        // Gerçek uygulamada: contract.watch_ad(connectedWallet, campaignId, zkProof.proofHash)
        
        const reward = 0.1 + (campaignId * 0.05); // Simulated reward
        
        // Kullanıcı istatistiklerini güncelle
        userStats.adViews++;
        userStats.earned += reward;
        
        updateUserDashboard();
        updateStats();
        
        // Modal'ı kapat
        setTimeout(() => {
            closeAdModal();
            showNotification(`Tebrikler! ${reward} XLM kazandınız! 💰`, 'success');
        }, 1000);
        
    } catch (error) {
        console.error('Ad view error:', error);
        showNotification('Reklam izleme sırasında hata oluştu', 'error');
        closeAdModal();
    }
}

function closeAdModal() {
    const modal = document.getElementById('adModal');
    modal.classList.add('hidden');
    
    // Reset progress
    document.getElementById('zkProgress').style.width = '0%';
    document.getElementById('adTimer').textContent = '15';
}

// Dashboard güncelle
function updateUserDashboard() {
    document.getElementById('userAdViews').textContent = userStats.adViews;
    document.getElementById('userEarned').textContent = userStats.earned.toFixed(2) + ' XLM';
}

// Genel istatistikleri güncelle
function updateStats() {
    // Simulated stats
    document.getElementById('totalUsers').textContent = Math.floor(Math.random() * 1000) + 500;
    document.getElementById('totalAds').textContent = 3;
    document.getElementById('totalEarned').textContent = 
        (Math.floor(Math.random() * 10000) + 5000).toFixed(2) + ' XLM';
}

// Bildirim göster
function showNotification(message, type = 'info') {
    // Basit console log (gerçek uygulamada toast notification kullanılabilir)
    console.log(`[${type.toUpperCase()}] ${message}`);
    
    // Tarayıcı bildirimi
    if ('Notification' in window && Notification.permission === 'granted') {
        new Notification('ZK Machine', {
            body: message,
            icon: '🔐'
        });
    }
}

// Mock address generator
function generateMockAddress() {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567';
    let address = 'G';
    for (let i = 0; i < 55; i++) {
        address += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return address;
}

// Contract interaction helpers (gerçek uygulamada kullanılacak)
async function callContract(method, params) {
    // Stellar SDK kullanarak contract çağrısı
    // const server = new SorobanClient.Server('https://soroban-testnet.stellar.org');
    // const contract = new SorobanClient.Contract(CONTRACT_ADDRESS);
    // return await contract.call(method, params);
    
    console.log(`Contract call: ${method}`, params);
    return { success: true };
}

// Export functions for HTML onclick handlers
window.watchAd = watchAd;
window.closeAdModal = closeAdModal;
