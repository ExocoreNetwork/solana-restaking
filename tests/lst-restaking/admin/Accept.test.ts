import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LstRestaking } from "../../../target/types/lst_restaking";
import {getConfig, testKeys} from "../../utils";
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
    it("Accept request of transfer ownership!", async () => {
        const [owner, newOwner] = await testKeys();
        const [config] = await getConfig();

        const tx = await program.methods.accept()
            .accounts({
                newOwner: newOwner.publicKey,
                config
            })
            .signers([newOwner])
            .rpc();
        console.log("Your transaction signature", tx);

        const configState = await program.account.config.fetch(config);

        assert.equal(configState.owner.toString(), newOwner.publicKey.toString(), "Initialize is failed");
    });
});
