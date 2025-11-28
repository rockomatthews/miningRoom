import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MiningRoom } from "../target/types/mining_room";

describe("mining-room skeleton", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.MiningRoom as Program<MiningRoom>;

  it("loads program id", () => {
    expect(program.programId).toBeDefined();
  });
});

