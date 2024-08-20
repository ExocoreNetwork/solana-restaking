import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LstRestaking } from "../../../target/types/lst_restaking";
import {createMint, getConfig, testKeys, TestMint} from "../../utils";

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
    it("Remove token from white lists!", async () => {
        const [owner] = await testKeys();
        const [config] = await getConfig();

        const tx = await program.methods.updateWhiteLists({add: {}})
            .accounts({
                owner: owner.publicKey,
                config,
                mint: TestMint
            })
            .signers([owner])
            .rpc();
        console.log("Your transaction signature", tx);
    });
});
