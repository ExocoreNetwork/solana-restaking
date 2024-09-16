import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LstRestaking } from "../../../target/types/lst_restaking";

describe("solana-restaking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaRestaking as Program<LstRestaking>;

  it("Accept!", async () => {
    // Add your test here.
    const tx = await program.methods.accept().rpc();
    console.log("Your transaction signature", tx);
  });
});
