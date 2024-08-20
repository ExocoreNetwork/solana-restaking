import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LstRestaking } from "../../../target/types/lst_restaking";
import {createMint, getConfig, testKeys, TestMint} from "../../utils";
import {assert} from "chai";

describe("solana-restaking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();

  // Set the provider with customized options
  const customProvider = new anchor.AnchorProvider(
      provider.connection,
      provider.wallet,
      {
        preflightCommitment: "processed",  // or "confirmed" / "finalized"
        commitment: "confirmed",           // or "finalized"
      }
  );

  anchor.setProvider(customProvider);

  const program = anchor.workspace.LstRestaking as Program<LstRestaking>;

  it("Add token into white lists!", async () => {
      const [owner] = await testKeys();
      const [config] = await getConfig();

      await createMint(program, owner);

      const tx = await program.methods.updateWhiteLists({add: {}})
        .accounts({
            owner: owner.publicKey,
            config,
            mint: TestMint
        })
        .signers([owner])
        .rpc();
    console.log("Your transaction signature", tx);

      const configState = await program.account.config.fetch(config);

      assert.equal(configState.whiteListTokens.length, 1, "Add Token into white list failed");

      assert.equal(configState.whiteListTokens.at(0).toString(), TestMint.toString(), "Add Token into white list failed");
  });
});
