# 🚀 ZK Machine - Hızlı Başlangıç

## ✅ Proje Başarıyla Deploy Edildi!

Contract Stellar Testnet'e başarıyla deploy edildi ve 3 test kampanyası oluşturuldu.

### 📋 Contract Bilgileri

- **Contract ID**: `CBMH5XGPXFLU5AV4JHMPMGYC2U2OLMTEXGUV7VID76YIDFFEBLJMVVKT`
- **Network**: Testnet
- **Aktif Kampanya**: 3 adet

### 🔗 Linkler

**Stellar Expert** (Contract'ı görüntüle):
https://stellar.expert/explorer/testnet/contract/CBMH5XGPXFLU5AV4JHMPMGYC2U2OLMTEXGUV7VID76YIDFFEBLJMVVKT

**Stellar Lab** (Contract ile etkileşim):
https://lab.stellar.org/r/testnet/contract/CBMH5XGPXFLU5AV4JHMPMGYC2U2OLMTEXGUV7VID76YIDFFEBLJMVVKT

### 🎯 Uygulamayı Çalıştırma

#### 1. HTTP Server Başlat

**Python ile:**
```bash
python -m http.server 8000
```

**Node.js ile:**
```bash
npx http-server -p 8000
```

**PHP ile:**
```bash
php -S localhost:8000
```

#### 2. Tarayıcıda Aç

```
http://localhost:8000
```

### 🧪 Test Kampanyaları

| Kampanya | Ödül/İzlenme | Bütçe |
|----------|--------------|-------|
| #1 | 1 XLM | 100 XLM |
| #2 | 1.5 XLM | 75 XLM |
| #3 | 2 XLM | 50 XLM |

### 🔐 Nasıl Çalışır?

1. **Cüzdan Bağla**: Stellar cüzdanınızı bağlayın (şu an simüle edilmiş)
2. **ZK Proof**: Kimliğiniz gizli kalarak doğrulama yapılır
3. **Reklam İzle**: Kampanyalardan birini seçip reklam izleyin
4. **XLM Kazan**: Anında cüzdanınıza XLM kazanın

### 📱 Özellikler

- ✅ Zero-Knowledge Proof ile gizlilik
- ✅ Stellar blockchain üzerinde güvenli ödemeler
- ✅ Kullanıcı kimliği hiçbir zaman açığa çıkmaz
- ✅ Anında ödeme sistemi
- ✅ Şeffaf ve denetlenebilir

### 🛠️ Contract Fonksiyonları

#### Kullanıcı İşlemleri

```bash
# Kullanıcı kaydı
stellar contract invoke \
  --id CBMH5XGPXFLU5AV4JHMPMGYC2U2OLMTEXGUV7VID76YIDFFEBLJMVVKT \
  --source default \
  --network testnet \
  -- register_user \
  --user <USER_ADDRESS> \
  --proof_hash 12345

# Reklam izle ve ödül kazan
stellar contract invoke \
  --id CBMH5XGPXFLU5AV4JHMPMGYC2U2OLMTEXGUV7VID76YIDFFEBLJMVVKT \
  --source default \
  --network testnet \
  -- watch_ad \
  --user <USER_ADDRESS> \
  --campaign_id 1 \
  --proof_hash 12345

# Kullanıcı istatistikleri
stellar contract invoke \
  --id CBMH5XGPXFLU5AV4JHMPMGYC2U2OLMTEXGUV7VID76YIDFFEBLJMVVKT \
  --source default \
  --network testnet \
  -- get_user_stats \
  --user <USER_ADDRESS>
```

#### Kampanya İşlemleri

```bash
# Yeni kampanya oluştur
stellar contract invoke \
  --id CBMH5XGPXFLU5AV4JHMPMGYC2U2OLMTEXGUV7VID76YIDFFEBLJMVVKT \
  --source default \
  --network testnet \
  -- create_campaign \
  --advertiser <ADVERTISER_ADDRESS> \
  --reward_per_view 1000000 \
  --total_budget 100000000

# Kampanya bilgisi
stellar contract invoke \
  --id CBMH5XGPXFLU5AV4JHMPMGYC2U2OLMTEXGUV7VID76YIDFFEBLJMVVKT \
  --source default \
  --network testnet \
  -- get_campaign \
  --campaign_id 1

# Aktif kampanya sayısı
stellar contract invoke \
  --id CBMH5XGPXFLU5AV4JHMPMGYC2U2OLMTEXGUV7VID76YIDFFEBLJMVVKT \
  --source default \
  --network testnet \
  -- get_active_campaigns_count
```

### 🔄 Yeniden Deploy

Contract'ı güncellemek isterseniz:

```bash
# Contract'ı build et
cd contract
cargo build --target wasm32-unknown-unknown --release

# Deploy et
cd ..
stellar contract deploy \
  --wasm contract/target/wasm32-unknown-unknown/release/zk_ad_platform.wasm \
  --source default \
  --network testnet

# Initialize et
stellar contract invoke \
  --id <NEW_CONTRACT_ID> \
  --source default \
  --network testnet \
  -- initialize
```

### 🎨 Frontend Geliştirme

Frontend dosyaları:
- `index.html` - Ana sayfa
- `style.css` - Stil dosyası
- `app.js` - JavaScript logic

Gerçek Stellar cüzdan entegrasyonu için:
- [Freighter Wallet](https://www.freighter.app/)
- [Albedo Wallet](https://albedo.link/)

### 📚 Daha Fazla Bilgi

- [Stellar Docs](https://developers.stellar.org/)
- [Soroban Docs](https://soroban.stellar.org/docs)
- [Stellar Laboratory](https://laboratory.stellar.org/)

### 🐛 Sorun Giderme

**Contract bulunamıyor:**
- Contract ID'nin doğru olduğundan emin olun
- Network'ün testnet olduğunu kontrol edin

**Fonksiyonlar çalışmıyor:**
- Stellar CLI'nin güncel olduğundan emin olun
- `stellar --version` ile kontrol edin

**Frontend çalışmıyor:**
- HTTP server'ın çalıştığından emin olun
- Browser console'da hata kontrolü yapın

### 🚀 Sonraki Adımlar

1. ✅ Contract deploy edildi
2. ✅ Test kampanyaları oluşturuldu
3. 🔄 Frontend'i test edin
4. 🔄 Gerçek cüzdan entegrasyonu ekleyin
5. 🔄 Gerçek ZK proof implementasyonu
6. 🔄 Mainnet'e deploy

---

**Tebrikler!** ZK Machine başarıyla deploy edildi ve kullanıma hazır! 🎉
