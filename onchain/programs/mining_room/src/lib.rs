use anchor_lang::{prelude::*, solana_program::clock::Clock};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount},
};

declare_id!("61zXhvyP5jZhnj2X8XjT6S6Q1HJZk1wt4MPf9vVg89fq");

pub const GLOBAL_CONFIG_SEED: &[u8] = b"global-config";
pub const MINING_POOL_SEED: &[u8] = b"mining-pool";
pub const INSURANCE_VAULT_SEED: &[u8] = b"insurance";
pub const ROOM_SEED: &[u8] = b"room";
pub const MINER_SEED: &[u8] = b"miner";
pub const STAKE_SEED: &[u8] = b"stake";
pub const MAX_ROOM_SLOTS: u8 = 36;
pub const HASH_UNITS_PER_KW: u64 = 25;
pub const STAKE_TARGET_MRC_PER_KW: u64 = 50;
pub const MIN_STAKE_EFFICIENCY_BPS: u64 = 2_000;
pub const MAX_STAKE_EFFICIENCY_BPS: u64 = 15_000;
pub const COOLING_BASE_BPS: u64 = 10_000;

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

    pub fn unlock_slot(ctx: Context<UnlockSlot>, slot_index: u8) -> Result<()> {
        let room = &mut ctx.accounts.room;
        require_keys_eq!(room.owner, ctx.accounts.authority.key(), MiningRoomError::Unauthorized);
        require!(
            room.slots_unlocked < MAX_ROOM_SLOTS,
            MiningRoomError::SlotLimitReached
        );
        require!(
            slot_index == room.slots_unlocked,
            MiningRoomError::InvalidSlotIndex
        );
        room.slots_unlocked = room
            .slots_unlocked
            .checked_add(1)
            .ok_or(MiningRoomError::SlotLimitReached)?;
        Ok(())
    }

    pub fn purchase_miner(ctx: Context<PurchaseMiner>, model: MinerModel) -> Result<()> {
        let room = &mut ctx.accounts.room;
        require_keys_eq!(room.owner, ctx.accounts.owner.key(), MiningRoomError::Unauthorized);
        require!(
            room.miner_count < room.slots_unlocked,
            MiningRoomError::NoAvailableSlots
        );

        let stats = model.stats();
        let miner = &mut ctx.accounts.miner;
        miner.owner = ctx.accounts.owner.key();
        miner.room = room.key();
        miner.model = model;
        miner.power_kw = stats.power_kw;
        miner.cooling_pct = stats.cooling_bps;
        miner.risk_bps = stats.risk_bps;
        miner.status = MinerStatus::Mining;

        apply_room_delta(
            room,
            i64::from(stats.power_kw),
            i64::from(stats.cooling_bps),
            i64::from(stats.risk_bps),
            1,
        );

        room.next_miner_index = room
            .next_miner_index
            .checked_add(1)
            .ok_or(MiningRoomError::SlotLimitReached)?;

        if room.miner_count == 1 {
            room.last_harvest_ts = Clock::get()?.unix_timestamp;
        }

        Ok(())
    }

    pub fn upgrade_miner(ctx: Context<UpgradeMiner>, upgrade: UpgradeKind) -> Result<()> {
        let room = &mut ctx.accounts.room;
        let miner = &mut ctx.accounts.miner;
        require_keys_eq!(miner.owner, ctx.accounts.owner.key(), MiningRoomError::Unauthorized);
        require_keys_eq!(miner.room, room.key(), MiningRoomError::InvalidRoomReference);

        let old_power = miner.power_kw;
        let old_cooling = miner.cooling_pct;
        let old_risk = miner.risk_bps;

        match upgrade {
            UpgradeKind::Power => {
                let delta = ((u32::from(miner.power_kw) * 1500) / 10_000 + 5) as u16;
                miner.power_kw = miner.power_kw.saturating_add(delta);
                miner.risk_bps = miner.risk_bps.saturating_add(150);
            }
            UpgradeKind::Cooling => {
                let delta = ((u32::from(miner.cooling_pct) * 1200) / 10_000 + 50) as u16;
                miner.cooling_pct = miner.cooling_pct.saturating_add(delta);
                miner.risk_bps = miner.risk_bps.saturating_sub(200);
            }
            UpgradeKind::Firmware => {
                let power_boost = ((u32::from(miner.power_kw) * 500) / 10_000 + 2) as u16;
                let cooling_boost = ((u32::from(miner.cooling_pct) * 300) / 10_000 + 10) as u16;
                miner.power_kw = miner.power_kw.saturating_add(power_boost);
                miner.cooling_pct = miner.cooling_pct.saturating_add(cooling_boost);
                miner.risk_bps = miner.risk_bps.saturating_sub(50);
            }
        }

        apply_room_delta(
            room,
            i64::from(miner.power_kw) - i64::from(old_power),
            i64::from(miner.cooling_pct) - i64::from(old_cooling),
            i64::from(miner.risk_bps) - i64::from(old_risk),
            0,
        );

        Ok(())
    }

    pub fn sell_miner(ctx: Context<SellMiner>) -> Result<()> {
        let room = &mut ctx.accounts.room;
        let miner = &mut ctx.accounts.miner;
        require_keys_eq!(miner.owner, ctx.accounts.owner.key(), MiningRoomError::Unauthorized);
        require_keys_eq!(miner.room, room.key(), MiningRoomError::InvalidRoomReference);

        apply_room_delta(
            room,
            -i64::from(miner.power_kw),
            -i64::from(miner.cooling_pct),
            -i64::from(miner.risk_bps),
            -1,
        );

        Ok(())
    }

    pub fn claim_rewards(
        ctx: Context<ClaimRewards>,
        amount: u64,
        auto_compound: bool,
    ) -> Result<()> {
        require!(amount > 0, MiningRoomError::InvalidAmount);

        let stake = &mut ctx.accounts.stake;
        require_keys_eq!(stake.owner, ctx.accounts.owner.key(), MiningRoomError::Unauthorized);

        let room = &mut ctx.accounts.room;
        let now = Clock::get()?.unix_timestamp;
        let elapsed = if room.last_harvest_ts == 0 {
            0
        } else {
            now.saturating_sub(room.last_harvest_ts)
        };

        let generated = calculate_room_emission(
            room,
            stake.amount,
            ctx.accounts.global_config.emission_rate,
            elapsed,
        );

        let available = room.accrued_hash.saturating_add(generated);
        require!(amount <= available, MiningRoomError::ExceedsEmissionCap);

        room.accrued_hash = available - amount;
        room.last_harvest_ts = now;

        ctx.accounts.mining_pool.hash_liability = ctx
            .accounts
            .mining_pool
            .hash_liability
            .saturating_add(generated);

        require!(
            ctx.accounts.mining_pool.hash_liability >= amount,
            MiningRoomError::InsufficientRewards
        );
        ctx.accounts.mining_pool.hash_liability -= amount;

        if auto_compound {
            stake.amount = stake.amount.saturating_add(amount);
            let lock_extension = 86_400;
            stake.lock_end_ts = stake.lock_end_ts.max(now).saturating_add(lock_extension);
        } else {
            if ctx.accounts.pending_hash.owner == Pubkey::default() {
                ctx.accounts.pending_hash.owner = ctx.accounts.owner.key();
                ctx.accounts.pending_hash.bump = ctx.bumps.pending_hash;
            }
            ctx.accounts.pending_hash.amount =
                ctx.accounts.pending_hash.amount.saturating_add(amount);
        }

        Ok(())
    }

    pub fn stake_mrc(ctx: Context<StakeMrc>, amount: u64, lock_days: u16) -> Result<()> {
        require!(amount > 0, MiningRoomError::InvalidAmount);
        require!(lock_days > 0, MiningRoomError::InvalidLockPeriod);

        require_keys_eq!(
            ctx.accounts.payer_mrc_account.mint,
            ctx.accounts.mrc_mint.key(),
            MiningRoomError::InvalidMint
        );
        require_keys_eq!(
            ctx.accounts.stake_mrc_account.mint,
            ctx.accounts.mrc_mint.key(),
            MiningRoomError::InvalidMint
        );
        require_keys_eq!(
            ctx.accounts.payer_mrc_account.owner,
            ctx.accounts.owner.key(),
            MiningRoomError::Unauthorized
        );
        require_keys_eq!(
            ctx.accounts.stake_mrc_account.owner,
            ctx.accounts.stake.key(),
            MiningRoomError::Unauthorized
        );

        let stake = &mut ctx.accounts.stake;

        if stake.owner == Pubkey::default() {
            stake.owner = ctx.accounts.owner.key();
            stake.bump = ctx.bumps.stake;
        } else {
            require_keys_eq!(stake.owner, ctx.accounts.owner.key(), MiningRoomError::Unauthorized);
        }

        let now = Clock::get()?.unix_timestamp;
        let lock_extension = i64::from(lock_days) * 86_400;
        stake.amount = stake.amount.saturating_add(amount);
        stake.lock_end_ts = now.saturating_add(lock_extension);
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.payer_mrc_account.to_account_info(),
                    to: ctx.accounts.stake_mrc_account.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn unstake_mrc(ctx: Context<UnstakeMrc>) -> Result<()> {
        let stake = &mut ctx.accounts.stake;
        require_keys_eq!(stake.owner, ctx.accounts.owner.key(), MiningRoomError::Unauthorized);

        let now = Clock::get()?.unix_timestamp;
        require!(now >= stake.lock_end_ts, MiningRoomError::StakeLocked);

        let amount = stake.amount;
        stake.amount = 0;
        stake.lock_end_ts = now;

        let owner_key = ctx.accounts.owner.key();
        let stake_pubkey = stake.key();
        require_keys_eq!(
            ctx.accounts.owner_mrc_account.owner,
            owner_key,
            MiningRoomError::InvalidTokenAccount
        );
        require_keys_eq!(
            ctx.accounts.stake_mrc_account.owner,
            stake_pubkey,
            MiningRoomError::InvalidTokenAccount
        );
        require_keys_eq!(
            ctx.accounts.owner_mrc_account.mint,
            ctx.accounts.mrc_mint.key(),
            MiningRoomError::InvalidMint
        );
        require_keys_eq!(
            ctx.accounts.stake_mrc_account.mint,
            ctx.accounts.mrc_mint.key(),
            MiningRoomError::InvalidMint
        );
        let stake_bump = stake.bump;
        drop(stake);

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.stake_mrc_account.to_account_info(),
                    to: ctx.accounts.owner_mrc_account.to_account_info(),
                    authority: ctx.accounts.stake.to_account_info(),
                },
                &[&[
                    STAKE_SEED,
                    owner_key.as_ref(),
                    &[stake_bump],
                ]],
            ),
            amount,
        )?;

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
    #[account(
        mut,
        seeds = [ROOM_SEED, authority.key().as_ref()],
        bump = room.bump,
        constraint = room.owner == authority.key() @ MiningRoomError::Unauthorized
    )]
    pub room: Account<'info, Room>,
    #[account(mut)]
    pub mining_pool: Account<'info, MiningPool>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseMiner<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [ROOM_SEED, owner.key().as_ref()],
        bump = room.bump,
        has_one = owner
    )]
    pub room: Account<'info, Room>,
    #[account(
        init,
        payer = owner,
        space = MinerAccount::SPACE,
        seeds = [
            MINER_SEED,
            room.key().as_ref(),
            &room.next_miner_index.to_le_bytes()
        ],
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
    #[account(
        mut,
        seeds = [ROOM_SEED, owner.key().as_ref()],
        bump = room.bump,
        has_one = owner
    )]
    pub room: Account<'info, Room>,
    #[account(mut, has_one = owner, constraint = miner.room == room.key())]
    pub miner: Account<'info, MinerAccount>,
}

#[derive(Accounts)]
pub struct SellMiner<'info> {
    pub owner: Signer<'info>,
    #[account(mut, close = owner, has_one = owner, constraint = miner.room == room.key())]
    pub miner: Account<'info, MinerAccount>,
    #[account(
        mut,
        seeds = [ROOM_SEED, owner.key().as_ref()],
        bump = room.bump,
        has_one = owner
    )]
    pub room: Account<'info, Room>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [ROOM_SEED, owner.key().as_ref()],
        bump = room.bump,
        has_one = owner
    )]
    pub room: Account<'info, Room>,
    #[account(mut)]
    pub mining_pool: Account<'info, MiningPool>,
    pub global_config: Account<'info, GlobalConfig>,
    #[account(mut, seeds = [STAKE_SEED, owner.key().as_ref()], bump = stake.bump)]
    pub stake: Account<'info, StakeAccount>,
    #[account(
        init_if_needed,
        payer = owner,
        space = PendingHash::SPACE,
        seeds = [b"pending-hash", owner.key().as_ref()],
        bump
    )]
    pub pending_hash: Account<'info, PendingHash>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StakeMrc<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mrc_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mrc_mint,
        associated_token::authority = owner
    )]
    pub payer_mrc_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = owner,
        space = StakeAccount::SPACE,
        seeds = [STAKE_SEED, owner.key().as_ref()],
        bump
    )]
    pub stake: Account<'info, StakeAccount>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mrc_mint,
        associated_token::authority = stake
    )]
    pub stake_mrc_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct UnstakeMrc<'info> {
    pub owner: Signer<'info>,
    #[account(mut, seeds = [STAKE_SEED, owner.key().as_ref()], bump = stake.bump, close = owner)]
    pub stake: Account<'info, StakeAccount>,
    #[account(
        mut,
        constraint = stake_mrc_account.owner == stake.key() @ MiningRoomError::InvalidTokenAccount,
        constraint = stake_mrc_account.mint == mrc_mint.key() @ MiningRoomError::InvalidMint
    )]
    pub stake_mrc_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = owner_mrc_account.owner == owner.key() @ MiningRoomError::InvalidTokenAccount,
        constraint = owner_mrc_account.mint == mrc_mint.key() @ MiningRoomError::InvalidMint
    )]
    pub owner_mrc_account: Account<'info, TokenAccount>,
    pub mrc_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
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

fn apply_room_delta(
    room: &mut Room,
    power_delta: i64,
    cooling_delta: i64,
    risk_delta: i64,
    miner_delta: i8,
) {
    room.total_power_kw = adjust_total(room.total_power_kw, power_delta);
    room.total_cooling_bps = adjust_total(room.total_cooling_bps, cooling_delta);
    room.total_risk_bps = adjust_total(room.total_risk_bps, risk_delta);

    if miner_delta >= 0 {
        room.miner_count = room
            .miner_count
            .saturating_add(miner_delta as u8);
    } else {
        let remove = (-miner_delta) as u8;
        room.miner_count = room.miner_count.saturating_sub(remove);
        if room.miner_count == 0 {
            room.last_harvest_ts = 0;
        }
    }
}

fn adjust_total(current: u64, delta: i64) -> u64 {
    if delta >= 0 {
        current.saturating_add(delta as u64)
    } else {
        current.saturating_sub((-delta) as u64)
    }
}

fn calculate_room_emission(
    room: &Room,
    stake_amount: u64,
    emission_rate: u64,
    elapsed_secs: i64,
) -> u64 {
    if elapsed_secs <= 0 || room.total_power_kw == 0 || room.miner_count == 0 {
        return 0;
    }

    let hash_units = hash_units_from_power(room.total_power_kw);
    let base = hash_units
        .saturating_mul(emission_rate as u128)
        .saturating_mul(elapsed_secs as u128);

    let avg_cooling = room
        .total_cooling_bps
        .checked_div(room.miner_count as u64)
        .unwrap_or(COOLING_BASE_BPS);
    let avg_risk = room
        .total_risk_bps
        .checked_div(room.miner_count as u64)
        .unwrap_or(1_000);

    let cooling_bonus = clamp_u128(avg_cooling as u128, 8_000, 15_000);
    let risk_penalty = 10_000u128.saturating_sub(u128::from(avg_risk.min(5_000)) * 2);

    let stake_needed = u128::from(room.total_power_kw)
        .saturating_mul(STAKE_TARGET_MRC_PER_KW as u128)
        .max(1);
    let stake_ratio = u128::from(stake_amount)
        .saturating_mul(10_000)
        .checked_div(stake_needed)
        .unwrap_or(0);
    let stake_efficiency = clamp_u128(
        stake_ratio,
        MIN_STAKE_EFFICIENCY_BPS as u128,
        MAX_STAKE_EFFICIENCY_BPS as u128,
    );

    let efficiency = cooling_bonus
        .saturating_mul(risk_penalty)
        .saturating_mul(stake_efficiency)
        / 10_000
        / 10_000;

    let weighted = base.saturating_mul(efficiency);
    let scaled = weighted / 10_000;
    scaled.min(u128::from(u64::MAX)) as u64
}

fn clamp_u128(value: u128, min_value: u128, max_value: u128) -> u128 {
    if value < min_value {
        min_value
    } else if value > max_value {
        max_value
    } else {
        value
    }
}

fn hash_units_from_power(power_kw: u64) -> u128 {
    u128::from(power_kw).saturating_mul(HASH_UNITS_PER_KW as u128)
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
    pub total_power_kw: u64,
    pub total_cooling_bps: u64,
    pub total_risk_bps: u64,
    pub miner_count: u8,
    pub accrued_hash: u64,
    pub last_harvest_ts: i64,
}

impl Room {
    pub const SPACE: usize = 8
        + 1
        + 32
        + 1
        + 1
        + 8
        + 8
        + 8
        + 1
        + 8
        + 8;
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

#[account]
pub struct PendingHash {
    pub bump: u8,
    pub owner: Pubkey,
    pub amount: u64,
}

impl PendingHash {
    pub const SPACE: usize = 8 + 1 + 32 + 8;
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

#[derive(Clone, Copy)]
struct MinerStats {
    power_kw: u16,
    cooling_bps: u16,
    risk_bps: u16,
}

impl MinerModel {
    fn stats(&self) -> MinerStats {
        match self {
            MinerModel::Basic => MinerStats {
                power_kw: 40,
                cooling_bps: 8_500,
                risk_bps: 1_000,
            },
            MinerModel::Advanced => MinerStats {
                power_kw: 80,
                cooling_bps: 9_200,
                risk_bps: 1_150,
            },
            MinerModel::Hyperscale => MinerStats {
                power_kw: 150,
                cooling_bps: 9_600,
                risk_bps: 1_400,
            },
        }
    }
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
    #[msg("Insufficient rewards to claim")]
    InsufficientRewards,
    #[msg("Token account does not match expected mint or owner")]
    InvalidTokenAccount,
    #[msg("Mint does not match expected account")]
    InvalidMint,
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
    #[msg("Lock period must be greater than zero")]
    InvalidLockPeriod,
    #[msg("Stake is still locked")]
    StakeLocked,
    #[msg("No additional slots available")]
    SlotLimitReached,
    #[msg("Slot index is invalid for this room")]
    InvalidSlotIndex,
    #[msg("No unlocked slots are free for another miner")]
    NoAvailableSlots,
    #[msg("Miner does not belong to the provided room")]
    InvalidRoomReference,
    #[msg("Requested amount exceeds the room's emission allowance")]
    ExceedsEmissionCap,
}

