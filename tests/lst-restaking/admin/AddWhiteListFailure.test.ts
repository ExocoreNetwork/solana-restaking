import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import { LstRestaking } from "../../../target/types/lst_restaking";
import {getConfig, testKeys} from "../../utils";
import {expect} from "chai";
import {config} from "dotenv";

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

        const mint = new web3.PublicKey(process.env.MINT_ADDRESS);

        if (!mint)
        {
            console.log(`mint should not be empty`);
            return;
        }

        try {
            await program.methods.updateWhiteList({add: {}})
                .accounts({
                    owner: owner.publicKey,
                    config,
                    mint,
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
