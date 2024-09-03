import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LstRestaking } from "../../../target/types/lst_restaking";
import {createMint, getConfig, getTokenAccount, testKeys} from "../../utils";
import {expect} from "chai";
import {PublicKey} from "@solana/web3.js";
import {TOKEN_PROGRAM_ID} from "@coral-xyz/anchor/dist/cjs/utils/token";
import {ASSOCIATED_TOKEN_PROGRAM_ID} from "@solana/spl-token";

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
    it("Remove token which is not in white list!", async () => {
        const [owner] = await testKeys();
        const [config] = await getConfig();

        const mint = await createMint(program, owner);

        const poolTokenAccount = await getTokenAccount(config, mint, true);

        try {
            await program.methods.updateWhiteList({deactivate: {}})
                .accounts({
                    operator: owner.publicKey,
                    config,
                    mint,
                    poolTokenAccount,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedToken: ASSOCIATED_TOKEN_PROGRAM_ID,
                })
                .signers([owner])
                .rpc();
            expect.fail("Transaction should fail but success");
        } catch (err) {
            expect(err.error.errorCode.number).to.equal(6003);

            expect(err.error.errorMessage).to.equal("Mint is not exists");
        }
    });
});
