import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import { LstRestaking } from "../../../target/types/lst_restaking";
import {getConfig, testKeys } from "../../utils";
import {config} from "dotenv";
import {assert} from "chai";

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
    it("Remove token from white list!", async () => {
        const [owner] = await testKeys();
        const [config] = await getConfig();

        const mint = new web3.PublicKey(process.env.MINT_ADDRESS);

        const tx = await program.methods.updateWhiteList({deactivate: {}})
            .accounts({
                owner: owner.publicKey,
                config,
                mint,
            })
            .signers([owner])
            .rpc();
        console.log("Your transaction signature", tx);

        const configState = await program.account.config.fetch(config);

        assert.equal(configState.whiteListTokens.at(0).mint.toString(), mint.toString(), "Add Token into white list failed");
        assert.equal(configState.whiteListTokens.at(0).active, false, "Add Token into white list failed");

    });
});
