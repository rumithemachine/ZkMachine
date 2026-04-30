#!/bin/bash

# ZK Machine - Stellar Soroban Contract Deploy Script

echo "🚀 ZK Machine Contract Deploy Script"
echo "======================================"

# Renk kodları
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Soroban CLI kontrolü
if ! command -v soroban &> /dev/null; then
    echo -e "${RED}❌ Soroban CLI bulunamadı!${NC}"
    echo "Kurulum için: cargo install --locked soroban-cli"
    exit 1
fi

echo -e "${GREEN}✓ Soroban CLI bulundu${NC}"

# Network seçimi
echo ""
echo "Network seçin:"
echo "1) Testnet (önerilen)"
echo "2) Futurenet"
echo "3) Mainnet"
read -p "Seçiminiz (1-3): " network_choice

case $network_choice in
    1)
        NETWORK="testnet"
        RPC_URL="https://soroban-testnet.stellar.org"
        ;;
    2)
        NETWORK="futurenet"
        RPC_URL="https://rpc-futurenet.stellar.org"
        ;;
    3)
        NETWORK="mainnet"
        RPC_URL="https://soroban-mainnet.stellar.org"
        echo -e "${RED}⚠️  UYARI: Mainnet'e deploy ediyorsunuz!${NC}"
        read -p "Devam etmek istediğinizden emin misiniz? (yes/no): " confirm
        if [ "$confirm" != "yes" ]; then
            echo "Deploy iptal edildi."
            exit 0
        fi
        ;;
    *)
        echo -e "${RED}Geçersiz seçim!${NC}"
        exit 1
        ;;
esac

echo -e "${BLUE}📡 Network: $NETWORK${NC}"

# Contract build
echo ""
echo -e "${BLUE}🔨 Contract build ediliyor...${NC}"
cd contract
cargo build --target wasm32-unknown-unknown --release

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Build başarısız!${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Build başarılı${NC}"

# Optimize (opsiyonel)
echo ""
read -p "Contract'ı optimize etmek ister misiniz? (y/n): " optimize

if [ "$optimize" = "y" ]; then
    echo -e "${BLUE}⚡ Optimizing...${NC}"
    soroban contract optimize --wasm target/wasm32-unknown-unknown/release/zk_ad_platform.wasm
    WASM_FILE="target/wasm32-unknown-unknown/release/zk_ad_platform.optimized.wasm"
else
    WASM_FILE="target/wasm32-unknown-unknown/release/zk_ad_platform.wasm"
fi

# Identity kontrolü
echo ""
echo -e "${BLUE}🔑 Identity kontrolü...${NC}"

if ! soroban keys ls | grep -q "default"; then
    echo "Default identity bulunamadı. Oluşturuluyor..."
    soroban keys generate default --network $NETWORK
fi

echo -e "${GREEN}✓ Identity hazır${NC}"

# Deploy
echo ""
echo -e "${BLUE}🚀 Contract deploy ediliyor...${NC}"

CONTRACT_ID=$(soroban contract deploy \
    --wasm $WASM_FILE \
    --source default \
    --network $NETWORK \
    2>&1 | tail -n 1)

if [ -z "$CONTRACT_ID" ]; then
    echo -e "${RED}❌ Deploy başarısız!${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Contract başarıyla deploy edildi!${NC}"
echo ""
echo -e "${GREEN}📝 Contract ID: $CONTRACT_ID${NC}"

# Contract'ı initialize et
echo ""
echo -e "${BLUE}🔧 Contract initialize ediliyor...${NC}"

soroban contract invoke \
    --id $CONTRACT_ID \
    --source default \
    --network $NETWORK \
    -- initialize

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Contract initialize edildi${NC}"
else
    echo -e "${RED}⚠️  Initialize başarısız (zaten initialize edilmiş olabilir)${NC}"
fi

# Sonuçları kaydet
cd ..
echo ""
echo -e "${BLUE}💾 Bilgiler kaydediliyor...${NC}"

cat > contract_info.txt << EOF
ZK Machine Contract Deployment Info
====================================
Network: $NETWORK
RPC URL: $RPC_URL
Contract ID: $CONTRACT_ID
Deploy Date: $(date)

Frontend'de kullanmak için:
const CONTRACT_ADDRESS = "$CONTRACT_ID";
const NETWORK = "$NETWORK";
EOF

echo -e "${GREEN}✓ Bilgiler contract_info.txt dosyasına kaydedildi${NC}"

# Test kampanya oluştur (opsiyonel)
echo ""
read -p "Test kampanyası oluşturmak ister misiniz? (y/n): " create_campaign

if [ "$create_campaign" = "y" ]; then
    echo -e "${BLUE}📢 Test kampanyası oluşturuluyor...${NC}"
    
    IDENTITY_ADDRESS=$(soroban keys address default)
    
    soroban contract invoke \
        --id $CONTRACT_ID \
        --source default \
        --network $NETWORK \
        -- create_campaign \
        --advertiser $IDENTITY_ADDRESS \
        --reward_per_view 1000000 \
        --total_budget 100000000
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Test kampanyası oluşturuldu${NC}"
    fi
fi

echo ""
echo -e "${GREEN}🎉 Deploy tamamlandı!${NC}"
echo ""
echo "Sonraki adımlar:"
echo "1. contract_info.txt dosyasındaki Contract ID'yi app.js'e ekleyin"
echo "2. index.html'i tarayıcıda açın"
echo "3. Cüzdanınızı bağlayın ve test edin"
echo ""
echo -e "${BLUE}Stellar Expert'te görüntüle:${NC}"
echo "https://stellar.expert/explorer/$NETWORK/contract/$CONTRACT_ID"
