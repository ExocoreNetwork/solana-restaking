import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LstRestaking } from "../../../target/types/lst_restaking";
import {getConfig, testKeys} from "../../utils";
import {assert, expect} from "chai";

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

        try {
            const tx = await program.methods.accept()
                .accounts({
                    newOwner: owner.publicKey,
                    config
                })
                .signers([owner])
                .rpc();
            console.log("Your transaction signature", tx);

            expect.fail("The transaction should be fail but successful");
        } catch(err) {
            expect(err.error.errorCode.number).to.equal(6004);
            expect(err.error.errorMessage).to.equal("Invalid new owner");
        }

    });
});
