import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {LstRestaking} from "../../target/types/lst_restaking";
import {airdrop, getConfig, testKeys} from "../utils";
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

    it("Is initialized!", async () => {
        const [owner] = await testKeys();

        await airdrop(program.provider.connection, owner.publicKey);

        const [config] = await getConfig();

        const tx = await program.methods
            .initialize()
            .accounts({
                owner: owner.publicKey,
                config,
            })
            .signers([owner])
            .rpc();
        console.log("Your transaction signature", tx);

        const configState = await program.account.config.fetch(config);

        assert.equal(configState.owner.toString(), owner.publicKey.toString(), "Initialize is failed");
    });
});
