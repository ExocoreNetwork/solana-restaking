import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {LstRestaking} from "../../target/types/lst_restaking";
import {Transaction} from "@solana/web3.js";
import {airdrop, getConfig, testKeys} from "../utils";
import {assert} from "chai";
import BN from "bn.js";
import {
    createInitNonceInstruction,
    createInitReceiveLibraryInstruction,
    createInitSendLibraryInstruction,
} from "@layerzerolabs/lz-solana-sdk-v2";

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

    it("Is initialized!", async () => {
        const [owner] = await testKeys();

        await airdrop(program.provider.connection, owner.publicKey);

        const [config] = await getConfig();


        const init_tx= await program.methods
            .initialize()
            .accounts({
                owner: owner.publicKey,
                config,
            })
            .signers([owner])
            .rpc();

        console.log("Your transaction signature", init_tx);


        let tx = new Transaction();
        // init nonce
        const instr1 = createInitNonceInstruction();

        tx.add(instr1);
        // init_send_library
        const instr2 = createInitSendLibraryInstruction();

        tx.add(instr2);
        // init_receive_library
        const instr3 = createInitReceiveLibraryInstruction();

        tx.add(instr3);

        await provider.connection.sendTransaction(tx, [delegate]);

        console.log("Your transaction signature", tx);

        const configState = await program.account.config.fetch(config);

        assert.equal(configState.owner.toString(), owner.publicKey.toString(), "Initialize is failed");
        assert.isTrue(configState.nonce.eq(new BN(0)), "Initialize is failed");
    });
});
