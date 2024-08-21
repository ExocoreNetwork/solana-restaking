import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LstRestaking } from "../../../target/types/lst_restaking";
import {getConfig, testKeys, createMint} from "../../utils";
import {assert} from "chai";
import * as fs from "node:fs";

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

  it("Add token into white list!", async () => {
      const [owner] = await testKeys();
      const [config] = await getConfig();

      const mint = await createMint(program, owner);
      fs.writeFileSync(".env", `MINT_ADDRESS=${mint.toBase58()}`);

      const tx = await program.methods.updateWhiteList({add: {}})
        .accounts({
            owner: owner.publicKey,
            config,
            mint,
        })
        .signers([owner])
        .rpc();
    console.log("Your transaction signature", tx);

      const configState = await program.account.config.fetch(config);

      assert.equal(configState.whiteListTokens.length, 1, "Add Token into white list failed");

      assert.equal(configState.whiteListTokens.at(0).mint.toString(), mint.toString(), "Add Token into white list failed");

      assert.equal(configState.whiteListTokens.at(0).active, true, "Add Token into white list failed");
  });
});
