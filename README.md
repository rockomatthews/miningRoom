## Mining Room Workspace

Interactive Solana game concept where players build a virtual mining room, manage risk (heat vs. cooling), and interact with the MRC/HASH dual-token economy.

### Repo Layout

- `site/` – Next.js + Material UI dashboard (fullscreen board, store, wallet integration).
- `onchain/` – Anchor workspace for the `mining_room` program (`programs/mining_room`).
- `docs/mining-room-spec.md` – Living tokenomics & system design reference (still current).

### Frontend

```bash
cd site
npm install
npm run dev
```

The main screen is a non-scrolling game board, with the Store, swap, inventory buttons in the bottom bar. Replace `public/room.svg` when the final background art is ready.

### On-chain Program

```bash
cd onchain
npm install
npm run build   # wraps Anchor build
npm run test    # wraps Anchor test
```

- `scripts/run-anchor.sh` looks for an Anchor CLI binary (prefers `~/.cargo/bin/anchor` at v0.30.1). Override with `ANCHOR_BIN=/custom/path npm run build`.
- Program skeleton already defines PDAs, accounts, and instruction stubs (`initialize`, `unlock_slot`, `purchase_miner`, staking, emergency pause, etc.). Flesh out SOL/MRC/HASH logic next.

### Tokenomics Reference

The original game-economy plan, including HASH stablecoin mechanics, pool covers, insurance, and whale controls, lives in `docs/mining-room-spec.md`. Keep that file updated as numbers evolve to preserve context across sessions.

