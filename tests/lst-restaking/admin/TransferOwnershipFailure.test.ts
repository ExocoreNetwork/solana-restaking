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
    it("Transfer ownership without permission!", async () => {
        const [owner, newOwner, user] = await testKeys();
        const [config] = await getConfig();

        const tx = await program.methods.transferOwnership()
            .accounts({
                owner: owner.publicKey,
                newOwner: newOwner.publicKey,
                config
            })
            .signers([user])
            .rpc();

        console.log("Your transaction signature", tx);
    });
});
