# Mining Room Design Notes

## 1. Gameplay Loop

- **Room state** – Each player owns a single `Room` PDA containing: slot availability, installed miners, and décor (purely cosmetic at MVP). The visual is an empty room with traced placeholders and `+` icons marking empty install locations.
- **Acquisition flow**
  1. Player pays SOL to open a new slot (unlocking a traced area).
  2. Player purchases a miner model (basic > advanced > hyperscale) for SOL or the utility token once launched.
  3. Miner starts producing hashrate immediately but requires upgrades (power supply, cooling) for better yield.
- **Heat & gamble** – Each miner tracks `powerOutput` vs. `coolingCapacity`. Running miners in the red zone boosts earnings (Derby-like thrill) but introduces a failure roll that can fry the unit and wipe pending rewards. Cooling investments and insurance reduce—yet never remove—the risk.
- **Upgrade loop** – Power upgrades, efficiency mods, and firmware boosts each have cooldowns. Upgrades consume SOL/token, raise the miner’s tier, and increase payout share but also raise maintenance fees and heat output.
- **Sell / recycle** – Player can decommission a miner, receiving part of its value back (in utility token) while a portion is burned or routed to insurance reserves. Selling frees the slot.
- **Session cadence** – Intended to be a light-touch idle loop: players check in every few hours to claim rewards, schedule upgrades, or rotate miners.

## 2. Economy & Resource Flows

| Action | Cost Source | Flow Destination | Notes |
| --- | --- | --- | --- |
| Unlock slot | SOL | 80% to Mining Pool PDA, 15% to Insurance buffer, 5% dev ops | One-time per slot |
| Buy miner | SOL → swap to utility token on-chain | 70% to Mining Pool, 20% to model manufacturer queue (controls supply), 10% burn | Prevents unsustainable emissions |
| Upgrade | Utility token | 60% burn, 30% pool, 10% maintenance pot | Creates constant token sink |
| Claim rewards | Mining Pool | Player wallet in utility token (with optional auto-compound) | Subject to throttled emissions |
| Sell miner | Utility token escrow | 50% to player, 30% burn, 20% insurance | Limits quick flipping |
| Swap desk | SOL ⇄ utility token ⇄ HASH | 50 bps fee split 60/40 between burn and insurance | Guarantees on-site liquidity without external LP |

### Mining Pool Mechanics

- Every SOL deposit mints an equivalent share of utility token into the pool’s reserve wallet; emissions track `hashrate_share / total_hashrate`.
- Pool has **target cover ratio** (e.g., 120% of outstanding player balances). If cover < target, payouts slow and upgrade costs get a multiplier to refill the pool.
- Daily emission cap per player plus per-miner cap prevents draining. Excess share accrues as claimable `vPoints` that can be redeemed when liquidity recovers.

### Anti-drain Controls

- **Per-wallet buy caps** for fresh SOL inflow per epoch; larger buyers must stake more token for longer vesting.
- **Dynamic pricing** – Miner cost scales with total network hashrate to avoid runaway growth.
- **Upgrade cooldowns** – Hard minimum time between upgrades per miner, enforced by PDA timestamps.

## 3. Token & Treasury Mechanics

- **Utility token:** `MRC` (Mining Room Coin) – governs upgrades, staking, and governance.
- **Stable mining token:** `HASH`, a treasury-backed asset targeting 1 USD. Rewards are emitted in HASH so players feel like they are minting a stable yield; HASH can be redeemed on-site for SOL (subject to caps) to preserve the peg.
- **Supply policy (MRC):** Elastic, soft-capped at 100M circulating. Minting occurs only when SOL enters the system (slot unlocks, miner buys, swap desk usage) and the minted MRC sits as reserves in the pool. Burning happens on upgrades, miner sales, swap fees, and voluntary insurance staking exits.
- **Dual balance design:** Each player has `mrcLiquid` (transferable) and `mrcStaked` (escrowed for governance/boost multipliers). Large reward claims auto-stake a percentage into `mrcStaked` so whales help secure the system.
- **HASH stabilization:** Treasury holds SOL and liquid staking receipts; minting HASH requires locking equivalent SOL plus a safety premium. Redemptions burn HASH and release SOL, while an internal controller tweaks fees ±20 bps whenever the peg drifts.
- **Pricing oracle & swap desk:** Program maintains a TWAP from recent SOL inflows/outflows, which prices MRC and HASH without an external LP. A built-in swap UI lets players swap SOL ⇄ MRC ⇄ HASH, charging 50 bps (30 bps burn, 20 bps insurance).
- **Stability buffer:** 15% of every SOL inflow routes to an `Insurance PDA`. If Mining Pool liquidity ratio drops below 1.05, insurance automatically backstops claims and temporarily raises burn rates until equilibrium returns.
- **Hash parity curve:** HASH/day per wallet = `base_hashrate(model + upgrades)` × `cooling efficiency` × `staked_mrc_share`. Running miners in the red without enough staked MRC triggers simulated brownouts, while over-staking without strong rigs only yields minimum emissions. Players must balance heat gambles and staking commitments to unlock peak HASH/second so both earning vectors reinforce each other.

### Whale-proofing extras

- **Progressive tax:** Claim events scale a withdrawal fee from 0–12% depending on how much of the daily quota the wallet consumes. Fees go to insurance/burn.
- **Vesting for large buyers:** Any purchase above 2× the median daily spend mints MRC with a 7-day linear vest; unvested MRC can’t be sold or used to claim top-tier miners.
- **Room-level constraints:** Each room has tiered caps (e.g., max 5 hyperscale miners). To exceed, players must stake additional MRC into a global governance lock that can be slashed for abuse.

### Governance & Maintenance

- **Dev ops fee:** 5% of inflows, automatically streamed to a multisig with a 30-day cliff (cannot be instantly withdrawn).
- **Emergency pause:** DAO-controlled switch that freezes new purchases and slows emissions; requires 3/5 multisig + on-chain vote of stakers.
- **Parameter updates:** Items like emission rate, cooldowns, tax curve are stored in a config PDA and can only change via time-locked governance proposals.

## 4. Technical Architecture & Build Steps

### Solana Program

- **Stack:** Anchor + TypeScript client. Consider using Bubblegum/MPL for optional miner NFTs later, but MVP stores miners as PDA accounts.
- **Key accounts**
  - `GlobalConfig` PDA – emission params, pricing curve, authorities.
  - `MiningPool` PDA – tracks SOL + MRC balances plus HASH liabilities, cover ratio, last emission tick.
  - `InsuranceVault` PDA – separate treasury for safety net.
  - `Room` PDA (per player) – slot unlock state, installed miners, UI metadata.
  - `Miner` PDA – child of `Room`, storing model type, power level, cooldowns, accrued vPoints.
  - `StakeAccount` PDA – holds staked MRC, vesting info, governance power.
- **Instruction set**
  - `init_room`, `unlock_slot`, `purchase_miner`, `upgrade_miner`, `sell_miner`.
  - `claim_rewards` with throttled logic referencing global cover ratio.
  - `stake_mrc`, `unstake_mrc`, `governance_vote`, `update_params` (time-locked).
  - `emergency_pause` / `resume`.
- **Security considerations:** Use PDA seeds that include version bytes to support migrations; gate large value transfers via signer seeds + multisig; add circuit breakers on emission if cover ratio < 0.9.

### Frontend (MVP)

- **Framework:** Next.js + TypeScript with Material UI for layout/components. Use Canvas (Fabric.js or PixiJS) inside a Next.js page for the interactive room.
- **Wallet integration:** @solana/wallet-adapter (Next.js package) with Phantom/Solflare; include Solana Pay link for quick top-ups.
- **Views**
  - `RoomView` – renders traced floor plan, `+` icons for empty slots, drag/drop miners, and heat gauges showing gamble risk.
  - `ControlPanel` – purchase/upgrade buttons, cooldown timers, payout meters, and insurance toggles.
  - `SwapDesk` – SOL ⇄ MRC ⇄ HASH swapper with peg status and fee breakdown.
  - `TreasuryDashboard` – displays pool cover ratio, insurance health, governance proposals.
- **UI polish:** Material UI theming for dashboards/modals; start with simple vector art for the room, leaving hooks for cosmetic NFT décor later.

### Build Milestones

1. **Prototype (Week 1-2)** – Implement Anchor program with deposits, pool accounting, heat risk simulation, basic `claim_rewards`. CLI client for manual testing.
2. **Token integration (Week 3)** – Mint MRC + HASH SPL tokens, wire mint/burn hooks, add staking escrow and internal swap desk logic.
3. **Frontend alpha (Week 4)** – Next.js + Material UI room UI, wallet connect, ability to buy first miner model, show hashrate, heat gauges, and pending HASH rewards.
4. **Upgrades & resale (Week 5)** – Add upgrade cooldown logic, sell flow, progressive tax, and live SOL ⇄ MRC ⇄ HASH swapping.
5. **Governance & pause (Week 6)** – Roll out staking UI, proposal creation, emergency controls.
6. **Audit / testnet (Week 7)** – Security review, fuzz tests with solana-program-test, deploy to devnet with faucet for playtesting.


