# ZK Machine
<img width="736" height="629" alt="indir" src="https://github.com/user-attachments/assets/f0b963c3-331a-4899-9b46-ef287e99c8d7" />

Privacy-preserving advertising platform on Stellar blockchain using zero-knowledge proofs. Users watch ads anonymously and earn XLM rewards without revealing their identity.

## Architecture

```
User → ZK Proof Generator → Smart Contract → XLM Payment
       (Identity Hidden)    (Verify & Reward)
```

## Tech Stack

- **Smart Contract**: Rust + Soroban SDK 21.7.0
- **Blockchain**: Stellar (Soroban)
- **Frontend**: Vanilla JS, HTML5, CSS3
- **ZK Proof**: Simplified implementation (production requires zk-SNARKs/zk-STARKs)

## Deployment

### Live Contract (Testnet)

```
Contract ID: CBMH5XGPXFLU5AV4JHMPMGYC2U2OLMTEXGUV7VID76YIDFFEBLJMVVKT
Network: Stellar Testnet
RPC: https://soroban-testnet.stellar.org
```

**Explorer**: [View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CBMH5XGPXFLU5AV4JHMPMGYC2U2OLMTEXGUV7VID76YIDFFEBLJMVVKT)

### Active Campaigns

| ID | Reward/View | Budget |
|----|-------------|--------|
| 1  | 1 XLM       | 100 XLM |
| 2  | 1.5 XLM     | 75 XLM |
| 3  | 2 XLM       | 50 XLM |

## Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Stellar CLI
cargo install --locked stellar-cli

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Build & Deploy

```bash
# Build contract
cd contract
cargo build --target wasm32-unknown-unknown --release

# Configure network
stellar network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

# Generate identity & fund
stellar keys generate default --network testnet
stellar keys fund default --network testnet

# Deploy
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/zk_ad_platform.wasm \
  --source default \
  --network testnet

# Initialize
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source default \
  --network testnet \
  -- initialize
```

### Run Frontend

```bash
python -m http.server 8000
# Open http://localhost:8000
```

## Contract API

### User Functions

```rust
register_user(user: Address, proof_hash: u32) -> bool
watch_ad(user: Address, campaign_id: u32, proof_hash: u32) -> i128
get_user_stats(user: Address) -> (u32, i128)
```

### Campaign Functions

```rust
create_campaign(advertiser: Address, reward_per_view: i128, total_budget: i128) -> u32
get_campaign(campaign_id: u32) -> Option<AdCampaign>
get_active_campaigns_count() -> u32
```

## Testing

```bash
cd contract
cargo test
```

## Security Notes

⚠️ **Current ZK implementation is simplified for demo purposes**

Production deployment requires:
- Proper zk-SNARK/zk-STARK implementation
- Secure proof generation circuit
- Proof replay protection
- Comprehensive security audit

## License

MIT
