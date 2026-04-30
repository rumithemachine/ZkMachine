#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec, symbol_short};

// Kullanıcı verisi (ZK proof ile doğrulanmış)
#[contracttype]
#[derive(Clone)]
pub struct UserProfile {
    pub user_id: Address,
    pub ad_views: u32,
    pub total_earned: i128,
    pub is_verified: bool,
}

// Reklam kampanyası
#[contracttype]
#[derive(Clone)]
pub struct AdCampaign {
    pub campaign_id: u32,
    pub advertiser: Address,
    pub reward_per_view: i128,
    pub total_budget: i128,
    pub remaining_budget: i128,
    pub is_active: bool,
}

// ZK Proof (basitleştirilmiş - gerçek uygulamada daha karmaşık olacak)
#[contracttype]
#[derive(Clone)]
pub struct ZKProof {
    pub proof_hash: u32,
    pub timestamp: u64,
    pub is_valid: bool,
}

const USER_PROFILES: Symbol = symbol_short!("USERS");
const CAMPAIGNS: Symbol = symbol_short!("CAMPAIGNS");
const CAMPAIGN_COUNT: Symbol = symbol_short!("CAMP_CNT");

#[contract]
pub struct ZKAdPlatform;

#[contractimpl]
impl ZKAdPlatform {
    
    /// Platform'u başlat
    pub fn initialize(env: Env) {
        env.storage().instance().set(&CAMPAIGN_COUNT, &0u32);
    }

    /// Yeni reklam kampanyası oluştur
    pub fn create_campaign(
        env: Env,
        advertiser: Address,
        reward_per_view: i128,
        total_budget: i128,
    ) -> u32 {
        advertiser.require_auth();
        
        let mut campaign_count: u32 = env.storage()
            .instance()
            .get(&CAMPAIGN_COUNT)
            .unwrap_or(0);
        
        campaign_count += 1;
        
        let campaign = AdCampaign {
            campaign_id: campaign_count,
            advertiser: advertiser.clone(),
            reward_per_view,
            total_budget,
            remaining_budget: total_budget,
            is_active: true,
        };
        
        let mut campaigns: Vec<AdCampaign> = env.storage()
            .instance()
            .get(&CAMPAIGNS)
            .unwrap_or(Vec::new(&env));
        
        campaigns.push_back(campaign);
        env.storage().instance().set(&CAMPAIGNS, &campaigns);
        env.storage().instance().set(&CAMPAIGN_COUNT, &campaign_count);
        
        campaign_count
    }

    /// Kullanıcı kaydı (ZK proof ile)
    pub fn register_user(env: Env, user: Address, proof_hash: u32) -> bool {
        user.require_auth();
        
        // ZK proof doğrulama (basitleştirilmiş)
        let is_valid = Self::verify_zk_proof(env.clone(), proof_hash);
        
        if !is_valid {
            return false;
        }
        
        let profile = UserProfile {
            user_id: user.clone(),
            ad_views: 0,
            total_earned: 0,
            is_verified: true,
        };
        
        let mut users: Vec<UserProfile> = env.storage()
            .instance()
            .get(&USER_PROFILES)
            .unwrap_or(Vec::new(&env));
        
        users.push_back(profile);
        env.storage().instance().set(&USER_PROFILES, &users);
        
        true
    }

    /// Reklam izleme ve ödül alma
    pub fn watch_ad(
        env: Env,
        user: Address,
        campaign_id: u32,
        proof_hash: u32,
    ) -> i128 {
        user.require_auth();
        
        // ZK proof ile kullanıcının gerçek olduğunu doğrula
        if !Self::verify_zk_proof(env.clone(), proof_hash) {
            return 0;
        }
        
        // Kampanyayı bul
        let campaigns: Vec<AdCampaign> = env.storage()
            .instance()
            .get(&CAMPAIGNS)
            .unwrap_or(Vec::new(&env));
        
        let mut reward: i128 = 0;
        let mut updated_campaigns = Vec::new(&env);
        
        for i in 0..campaigns.len() {
            let mut campaign = campaigns.get(i).unwrap();
            
            if campaign.campaign_id == campaign_id && campaign.is_active {
                if campaign.remaining_budget >= campaign.reward_per_view {
                    campaign.remaining_budget -= campaign.reward_per_view;
                    reward = campaign.reward_per_view;
                    
                    if campaign.remaining_budget == 0 {
                        campaign.is_active = false;
                    }
                }
            }
            
            updated_campaigns.push_back(campaign);
        }
        
        env.storage().instance().set(&CAMPAIGNS, &updated_campaigns);
        
        // Kullanıcı profilini güncelle
        Self::update_user_stats(env, user, reward);
        
        reward
    }

    /// Kullanıcı istatistiklerini güncelle
    fn update_user_stats(env: Env, user: Address, earned: i128) {
        let users: Vec<UserProfile> = env.storage()
            .instance()
            .get(&USER_PROFILES)
            .unwrap_or(Vec::new(&env));
        
        let mut updated_users = Vec::new(&env);
        
        for i in 0..users.len() {
            let mut profile = users.get(i).unwrap();
            
            if profile.user_id == user {
                profile.ad_views += 1;
                profile.total_earned += earned;
            }
            
            updated_users.push_back(profile);
        }
        
        env.storage().instance().set(&USER_PROFILES, &updated_users);
    }

    /// ZK Proof doğrulama (basitleştirilmiş)
    /// Gerçek uygulamada zk-SNARKs veya zk-STARKs kullanılmalı
    fn verify_zk_proof(env: Env, proof_hash: u32) -> bool {
        // Basit doğrulama: proof_hash > 0 ve timestamp geçerli
        if proof_hash == 0 {
            return false;
        }
        
        // Gerçek uygulamada:
        // 1. Proof'u parse et
        // 2. Public inputs'u kontrol et
        // 3. Cryptographic verification yap
        // 4. Proof'un daha önce kullanılmadığını kontrol et
        
        let _timestamp = env.ledger().timestamp();
        
        // Demo için basit doğrulama - her proof geçerli kabul edilir
        proof_hash > 0
    }

    /// Kullanıcı bilgilerini getir (gizlilik korunarak)
    pub fn get_user_stats(env: Env, user: Address) -> (u32, i128) {
        let users: Vec<UserProfile> = env.storage()
            .instance()
            .get(&USER_PROFILES)
            .unwrap_or(Vec::new(&env));
        
        for i in 0..users.len() {
            let profile = users.get(i).unwrap();
            if profile.user_id == user {
                return (profile.ad_views, profile.total_earned);
            }
        }
        
        (0, 0)
    }

    /// Kampanya bilgilerini getir
    pub fn get_campaign(env: Env, campaign_id: u32) -> Option<AdCampaign> {
        let campaigns: Vec<AdCampaign> = env.storage()
            .instance()
            .get(&CAMPAIGNS)
            .unwrap_or(Vec::new(&env));
        
        for i in 0..campaigns.len() {
            let campaign = campaigns.get(i).unwrap();
            if campaign.campaign_id == campaign_id {
                return Some(campaign);
            }
        }
        
        None
    }

    /// Aktif kampanya sayısını getir
    pub fn get_active_campaigns_count(env: Env) -> u32 {
        let campaigns: Vec<AdCampaign> = env.storage()
            .instance()
            .get(&CAMPAIGNS)
            .unwrap_or(Vec::new(&env));
        
        let mut count = 0u32;
        for i in 0..campaigns.len() {
            let campaign = campaigns.get(i).unwrap();
            if campaign.is_active {
                count += 1;
            }
        }
        
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_create_campaign() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ZKAdPlatform);
        let client = ZKAdPlatformClient::new(&env, &contract_id);
        
        let advertiser = Address::generate(&env);
        
        env.mock_all_auths();
        
        client.initialize();
        let campaign_id = client.create_campaign(&advertiser, &1000000, &10000000);
        
        assert_eq!(campaign_id, 1);
    }

    #[test]
    fn test_register_and_watch_ad() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ZKAdPlatform);
        let client = ZKAdPlatformClient::new(&env, &contract_id);
        
        let advertiser = Address::generate(&env);
        let user = Address::generate(&env);
        
        env.mock_all_auths();
        
        client.initialize();
        
        // Kampanya oluştur
        let campaign_id = client.create_campaign(&advertiser, &1000000, &10000000);
        
        // Kullanıcı kaydı
        let registered = client.register_user(&user, &12345);
        assert!(registered);
        
        // Reklam izle
        let reward = client.watch_ad(&user, &campaign_id, &12345);
        assert_eq!(reward, 1000000);
        
        // İstatistikleri kontrol et
        let (views, earned) = client.get_user_stats(&user);
        assert_eq!(views, 1);
        assert_eq!(earned, 1000000);
    }
}
