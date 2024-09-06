import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import {LstRestaking} from "../../target/types/lst_restaking";

import {
    airdrop,
    ENDPOINT_PROGRAM_ID,
    getConfig,
    getPDATokenAccount,
    getVault,
    sendRemainingAccounts,
    testKeys,
} from "../utils";
import {assert} from "chai";
import {ComputeBudgetProgram, LAMPORTS_PER_SOL} from "@solana/web3.js";
import BN from "bn.js";
import {config} from "dotenv";
import {Options} from "@layerzerolabs/lz-v2-utilities";

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

    it("Withdraw principal from Exocore", async () => {
        const [owner , user, , , delegate] = await testKeys();

        const mint = new web3.PublicKey(process.env.MINT_ADDRESS);

        const [config] = await getConfig();

        const [vault] = await getVault(mint, user.publicKey);

        const poolTokenAccount = await getPDATokenAccount(mint, config);

        console.log(`poolTokenAccount: ${poolTokenAccount}`);

        console.log(`delegate pubkey: ${delegate.publicKey}`);

        await airdrop(anchor.getProvider().connection, user.publicKey);
        await airdrop(anchor.getProvider().connection, delegate.publicKey);

        const withdrawAmount = 5000 * LAMPORTS_PER_SOL;

        const options = Options.newOptions().addExecutorLzReceiveOption(500_000).toBytes();

        const accounts = await sendRemainingAccounts(user.publicKey);

        let tx = new web3.Transaction();

        tx.add(ComputeBudgetProgram.setComputeUnitLimit({ units: 1000_000}));

        tx.add(await program.methods
            .withdrawPrincipalFromExocore({
                amountOut: new BN(withdrawAmount),
                opts: Buffer.from(options)
            })
            .accounts({
                depositor: user.publicKey,
                vault,
                mint,
                config,
                endpointProgram: ENDPOINT_PROGRAM_ID
            })
            .remainingAccounts(
                accounts
            ).instruction()
        );

        const transactionSignature = await provider.connection.sendTransaction(tx, [user]);

        console.log(`Your transaction signature: ${transactionSignature}`);

        const VaultState = await program.account.vault.fetch(vault);

        assert.equal(VaultState.withdrawableBalance.toString(), withdrawAmount.toString(), "Deposit failed");
    });
});
