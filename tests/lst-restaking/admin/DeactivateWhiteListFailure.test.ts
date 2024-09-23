import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LstRestaking } from "../../../target/types/lst_restaking";
import {createTestMint, getConfig, getPDATokenAccount, getTokens, testKeys} from "../../utils";
import {expect} from "chai";
import {Connection, } from "@solana/web3.js";
import {TOKEN_PROGRAM_ID} from "@coral-xyz/anchor/dist/cjs/utils/token";
import {ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";

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
        const [tokenWhiteList] = await getTokens();

        const conn = anchor.getProvider().connection as unknown as Connection;

        const mint = await createTestMint(conn, owner);

        const poolTokenAccount = await getPDATokenAccount(mint, config);

        try {
            await program.methods.updateWhiteList({deactivate: {}})
                .accounts({
                    operator: owner.publicKey,
                    config,
                    mint,
                    poolTokenAccount,
                    tokenWhiteList,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
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
