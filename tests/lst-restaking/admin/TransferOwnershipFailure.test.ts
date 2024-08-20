import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {LstRestaking} from "../../../target/types/lst_restaking";
import {getConfig, testKeys} from "../../utils";
import {expect} from "chai";

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

        try {
            const tx = await program.methods.transferOwnership()
                .accounts({
                    owner: user.publicKey,
                    newOwner: newOwner.publicKey,
                    config
                })
                .signers([user])
                .rpc();

            console.log("Your transaction signature", tx);
            expect.fail("Transaction should be fail but success");
        } catch (err) {
            expect(err.error.errorCode.number).to.equal(6000);

            expect(err.error.errorMessage).to.equal("Invalid owner");
        }
    });
});
