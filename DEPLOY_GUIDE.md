# 🚀 ZK Machine - Deploy Rehberi

## Hızlı Başlangıç

### 1. Gereksinimleri Yükleyin

```bash
# Rust kurulumu (eğer yoksa)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Soroban CLI kurulumu
cargo install --locked soroban-cli --version 21.5.0

# WASM target ekle
rustup target add wasm32-unknown-unknown
```

### 2. Contract'ı Build Edin

```bash
cd contract
cargo build --target wasm32-unknown-unknown --release
```

### 3. Soroban Network Yapılandırması

```bash
# Testnet için
soroban network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

# Identity oluştur
soroban keys generate default --network testnet

# Adresinizi görüntüleyin
soroban keys address default
```

### 4. Testnet'ten XLM Alın

Adresinizi kopyalayın ve şu adresten test XLM alın:
https://laboratory.stellar.org/#account-creator?network=test

### 5. Contract'ı Deploy Edin

#### Otomatik Deploy (Önerilen)

```bash
# Deploy script'ini çalıştırılabilir yapın
chmod +x deploy.sh

# Deploy edin
./deploy.sh
```

#### Manuel Deploy

```bash
# Contract'ı deploy et
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/zk_ad_platform.wasm \
  --source default \
  --network testnet

# Çıktıdaki Contract ID'yi kaydedin
# Örnek: CBGTG7KCNKQFQVXQHQZQXQZQXQZQXQZQXQZQXQZQXQZQXQZQXQZQXQ

# Contract'ı initialize edin
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source default \
  --network testnet \
  -- initialize
```

### 6. Test Kampanyası Oluşturun

```bash
# Adresinizi alın
IDENTITY=$(soroban keys address default)

# Kampanya oluştur
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source default \
  --network testnet \
  -- create_campaign \
  --advertiser $IDENTITY \
  --reward_per_view 1000000 \
  --total_budget 100000000
```

### 7. Frontend'i Yapılandırın

`app.js` dosyasında CONTRACT_ADDRESS'i güncelleyin:

```javascript
const CONTRACT_ADDRESS = "SIZIN_CONTRACT_ID";
```

### 8. Uygulamayı Çalıştırın

```bash
# Basit HTTP server ile
python -m http.server 8000

# Veya Node.js ile
npx http-server -p 8000
```

Tarayıcıda açın: http://localhost:8000

## Contract Fonksiyonları

### Kullanıcı İşlemleri

```bash
# Kullanıcı kaydı
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source default \
  --network testnet \
  -- register_user \
  --user <USER_ADDRESS> \
  --proof_hash 12345

# Reklam izle
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source default \
  --network testnet \
  -- watch_ad \
  --user <USER_ADDRESS> \
  --campaign_id 1 \
  --proof_hash 12345

# Kullanıcı istatistikleri
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source default \
  --network testnet \
  -- get_user_stats \
  --user <USER_ADDRESS>
```

### Kampanya İşlemleri

```bash
# Kampanya bilgisi
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source default \
  --network testnet \
  -- get_campaign \
  --campaign_id 1

# Aktif kampanya sayısı
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source default \
  --network testnet \
  -- get_active_campaigns_count
```

## Testler

```bash
cd contract
cargo test
```

## Sorun Giderme

### "Contract not found" hatası
- Contract ID'nin doğru olduğundan emin olun
- Network'ün doğru olduğunu kontrol edin

### "Insufficient balance" hatası
- Testnet'ten daha fazla XLM alın
- https://laboratory.stellar.org/#account-creator?network=test

### Build hataları
- Rust ve Soroban CLI versiyonlarını kontrol edin
- `cargo clean` yapıp tekrar build edin

## Mainnet'e Deploy

⚠️ **UYARI**: Mainnet'e deploy etmeden önce:

1. Contract'ı testnet'te kapsamlı test edin
2. Güvenlik denetimi yaptırın
3. Gerçek XLM kullanacağınızı unutmayın

```bash
# Mainnet network ekle
soroban network add mainnet \
  --rpc-url https://soroban-mainnet.stellar.org \
  --network-passphrase "Public Global Stellar Network ; September 2015"

# Mainnet identity oluştur
soroban keys generate mainnet-key --network mainnet

# Deploy
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/zk_ad_platform.wasm \
  --source mainnet-key \
  --network mainnet
```

## Kaynaklar

- [Soroban Docs](https://soroban.stellar.org/docs)
- [Stellar Laboratory](https://laboratory.stellar.org/)
- [Stellar Expert](https://stellar.expert/)
- [Soroban Examples](https://github.com/stellar/soroban-examples)

## Destek

Sorularınız için:
- Stellar Discord: https://discord.gg/stellar
- Soroban Docs: https://soroban.stellar.org/docs
