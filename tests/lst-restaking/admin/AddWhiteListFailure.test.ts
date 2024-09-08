import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import { LstRestaking } from "../../../target/types/lst_restaking";
import {getConfig, getPDATokenAccount, getTokenAccount, getTokenWhiteList, testKeys} from "../../utils";
import {expect} from "chai";
import {config} from "dotenv";
import {TOKEN_PROGRAM_ID} from "@coral-xyz/anchor/dist/cjs/utils/token";
import {ASSOCIATED_TOKEN_PROGRAM_ID} from "@solana/spl-token";

config();

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
    it("Add token which is already exists in white list!", async () => {
        const [owner] = await testKeys();
        const [config] = await getConfig();
        const [tokenWhiteList] = await getTokenWhiteList();


        const mint = new web3.PublicKey(process.env.MINT_ADDRESS);

        const poolTokenAccount = await getPDATokenAccount(mint, config);


        if (!mint)
        {
            console.log(`mint should not be empty`);
            return;
        }

        try {
            await program.methods.updateWhiteList({add: {}})
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
            // console.log(err);

            expect(err.error.errorCode.code).to.equal('MintAlreadyExists');

            expect(err.error.errorCode.number).to.equal(6002);

            expect(err.error.errorMessage).to.equal("Mint is already exists");
        }
    });
});
