use anchor_lang::prelude::*;

declare_id!("61zXhvyP5jZhnj2X8XjT6S6Q1HJZk1wt4MPf9vVg89fq");

pub const GLOBAL_CONFIG_SEED: &[u8] = b"global-config";
pub const MINING_POOL_SEED: &[u8] = b"mining-pool";
pub const INSURANCE_VAULT_SEED: &[u8] = b"insurance";
pub const ROOM_SEED: &[u8] = b"room";
pub const MINER_SEED: &[u8] = b"miner";
pub const STAKE_SEED: &[u8] = b"stake";

#[program]
pub mod mining_room {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        let config = &mut ctx.accounts.global_config;
        config.bump = ctx.bumps.global_config;
        config.authority = ctx.accounts.authority.key();
        config.mrc_mint = params.mrc_mint;
        config.hash_mint = params.hash_mint;
        config.emission_rate = params.emission_rate;
        config.cover_target_bps = params.cover_target_bps;
        config.paused = false;

        let pool = &mut ctx.accounts.mining_pool;
        pool.bump = ctx.bumps.mining_pool;
        pool.total_sol = 0;
        pool.total_mrc = 0;
        pool.hash_liability = 0;

        let insurance = &mut ctx.accounts.insurance_vault;
        insurance.bump = ctx.bumps.insurance_vault;
        insurance.reserve_sol = 0;
        insurance.cover_ratio_bps = params.cover_target_bps;
        Ok(())
    }

    pub fn unlock_slot(_ctx: Context<UnlockSlot>, _slot_index: u8) -> Result<()> {
        Ok(())
    }

    pub fn purchase_miner(_ctx: Context<PurchaseMiner>, _model: MinerModel) -> Result<()> {
        Ok(())
    }

    pub fn upgrade_miner(_ctx: Context<UpgradeMiner>, _upgrade: UpgradeKind) -> Result<()> {
        Ok(())
    }

    pub fn sell_miner(_ctx: Context<SellMiner>) -> Result<()> {
        Ok(())
    }

    pub fn claim_rewards(_ctx: Context<ClaimRewards>) -> Result<()> {
        Ok(())
    }

    pub fn stake_mrc(_ctx: Context<StakeMrc>, _amount: u64, _lock_days: u16) -> Result<()> {
        Ok(())
    }

    pub fn unstake_mrc(_ctx: Context<UnstakeMrc>) -> Result<()> {
        Ok(())
    }

    pub fn update_params(ctx: Context<UpdateParams>, params: UpdateParamsInput) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.global_config.authority,
            ctx.accounts.authority.key(),
            MiningRoomError::Unauthorized
        );
        let config = &mut ctx.accounts.global_config;
        if let Some(emission) = params.emission_rate {
            config.emission_rate = emission;
        }
        if let Some(target) = params.cover_target_bps {
            config.cover_target_bps = target;
        }
        if let Some(paused) = params.paused {
            config.paused = paused;
        }
        Ok(())
    }

    pub fn emergency_pause(ctx: Context<EmergencyPause>, paused: bool) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.global_config.authority,
            ctx.accounts.authority.key(),
            MiningRoomError::Unauthorized
        );
        ctx.accounts.global_config.paused = paused;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = GlobalConfig::SPACE,
        seeds = [GLOBAL_CONFIG_SEED],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,
    #[account(
        init,
        payer = authority,
        space = MiningPool::SPACE,
        seeds = [MINING_POOL_SEED],
        bump
    )]
    pub mining_pool: Account<'info, MiningPool>,
    #[account(
        init,
        payer = authority,
        space = InsuranceVault::SPACE,
        seeds = [INSURANCE_VAULT_SEED],
        bump
    )]
    pub insurance_vault: Account<'info, InsuranceVault>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnlockSlot<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [ROOM_SEED, authority.key().as_ref()], bump = room.bump)]
    pub room: Account<'info, Room>,
    #[account(mut)]
    pub mining_pool: Account<'info, MiningPool>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseMiner<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, seeds = [ROOM_SEED, owner.key().as_ref()], bump = room.bump)]
    pub room: Account<'info, Room>,
    #[account(
        init,
        payer = owner,
        space = MinerAccount::SPACE,
        seeds = [MINER_SEED, room.key().as_ref(), &[room.next_miner_index]],
        bump
    )]
    pub miner: Account<'info, MinerAccount>,
    #[account(mut)]
    pub mining_pool: Account<'info, MiningPool>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpgradeMiner<'info> {
    pub owner: Signer<'info>,
    #[account(mut, has_one = owner)]
    pub miner: Account<'info, MinerAccount>,
    pub global_config: Account<'info, GlobalConfig>,
}

#[derive(Accounts)]
pub struct SellMiner<'info> {
    pub owner: Signer<'info>,
    #[account(mut, close = owner, has_one = owner)]
    pub miner: Account<'info, MinerAccount>,
    #[account(mut, seeds = [ROOM_SEED, owner.key().as_ref()], bump = room.bump)]
    pub room: Account<'info, Room>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    pub owner: Signer<'info>,
    #[account(mut, seeds = [ROOM_SEED, owner.key().as_ref()], bump = room.bump)]
    pub room: Account<'info, Room>,
    #[account(mut)]
    pub mining_pool: Account<'info, MiningPool>,
    pub global_config: Account<'info, GlobalConfig>,
}

#[derive(Accounts)]
pub struct StakeMrc<'info> {
    pub owner: Signer<'info>,
    #[account(
        init_if_needed,
        payer = owner,
        space = StakeAccount::SPACE,
        seeds = [STAKE_SEED, owner.key().as_ref()],
        bump
    )]
    pub stake: Account<'info, StakeAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnstakeMrc<'info> {
    pub owner: Signer<'info>,
    #[account(mut, seeds = [STAKE_SEED, owner.key().as_ref()], bump = stake.bump, close = owner)]
    pub stake: Account<'info, StakeAccount>,
}

#[derive(Accounts)]
pub struct UpdateParams<'info> {
    pub authority: Signer<'info>,
    #[account(mut, seeds = [GLOBAL_CONFIG_SEED], bump = global_config.bump)]
    pub global_config: Account<'info, GlobalConfig>,
}

#[derive(Accounts)]
pub struct EmergencyPause<'info> {
    pub authority: Signer<'info>,
    #[account(mut, seeds = [GLOBAL_CONFIG_SEED], bump = global_config.bump)]
    pub global_config: Account<'info, GlobalConfig>,
}

#[account]
pub struct GlobalConfig {
    pub bump: u8,
    pub authority: Pubkey,
    pub mrc_mint: Pubkey,
    pub hash_mint: Pubkey,
    pub emission_rate: u64,
    pub cover_target_bps: u16,
    pub paused: bool,
}

impl GlobalConfig {
    pub const SPACE: usize = 8 + 1 + 32 + 32 + 32 + 8 + 2 + 1;
}

#[account]
pub struct MiningPool {
    pub bump: u8,
    pub total_sol: u64,
    pub total_mrc: u64,
    pub hash_liability: u64,
}

impl MiningPool {
    pub const SPACE: usize = 8 + 1 + 8 + 8 + 8;
}

#[account]
pub struct InsuranceVault {
    pub bump: u8,
    pub reserve_sol: u64,
    pub cover_ratio_bps: u16,
}

impl InsuranceVault {
    pub const SPACE: usize = 8 + 1 + 8 + 2;
}

#[account]
pub struct Room {
    pub bump: u8,
    pub owner: Pubkey,
    pub slots_unlocked: u8,
    pub next_miner_index: u8,
    pub heat_risk_bps: u16,
}

impl Room {
    pub const SPACE: usize = 8 + 1 + 32 + 1 + 1 + 2;
}

#[account]
pub struct MinerAccount {
    pub owner: Pubkey,
    pub room: Pubkey,
    pub model: MinerModel,
    pub power_kw: u16,
    pub cooling_pct: u16,
    pub risk_bps: u16,
    pub status: MinerStatus,
}

impl MinerAccount {
    pub const SPACE: usize = 8 + 32 + 32 + 1 + 2 + 2 + 2 + 1;
}

#[account]
pub struct StakeAccount {
    pub bump: u8,
    pub owner: Pubkey,
    pub amount: u64,
    pub lock_end_ts: i64,
}

impl StakeAccount {
    pub const SPACE: usize = 8 + 1 + 32 + 8 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct InitializeParams {
    pub mrc_mint: Pubkey,
    pub hash_mint: Pubkey,
    pub emission_rate: u64,
    pub cover_target_bps: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct UpdateParamsInput {
    pub emission_rate: Option<u64>,
    pub cover_target_bps: Option<u16>,
    pub paused: Option<bool>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum MinerModel {
    Basic,
    Advanced,
    Hyperscale,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum UpgradeKind {
    Power,
    Cooling,
    Firmware,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum MinerStatus {
    Idle,
    Mining,
    Cooling,
    Fried,
}

#[error_code]
pub enum MiningRoomError {
    #[msg("Unauthorized action for caller")]
    Unauthorized,
}

