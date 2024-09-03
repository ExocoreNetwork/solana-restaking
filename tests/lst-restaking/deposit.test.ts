import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import {LstRestaking} from "../../target/types/lst_restaking";
import {IDL} from "../../target/types/endpoint";
import {
    airdrop,
    eid,
    ENDPOINT_PROGRAM_ID,
    getConfig,
    getEventAuthority,
    getNonce,
    getOApp,
    getOAppRegistry,
    getPendingNonce,
    getReceiveLibraryConfig,
    getSendLibraryConfig, getTokenAccount, getVault,
    remoteEid,
    remoteOapp,
    SYSTEM_PROGRAM_ID,
    testKeys
} from "../utils";
import {assert} from "chai";
import {LAMPORTS_PER_SOL} from "@solana/web3.js";
import BN from "bn.js";
import {TOKEN_PROGRAM_ID} from "@coral-xyz/anchor/dist/cjs/utils/token";


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

    // const endpoint_program = new Program(IDL, ENDPOINT_PROGRAM_ID, provider);

    it("Deposit", async () => {
        const [,,user,,, delegate] = await testKeys();

        const mint = new web3.PublicKey(process.env.MINT_ADDRESS);

        const [config] = await getConfig();

        const [vault] = await getVault(mint, user.publicKey);

        const poolTokenAccount = await getTokenAccount(config, mint, true);

        const userTokenAccount = await getTokenAccount(user.publicKey, mint);

        const [OApp] = await getOApp();

        const [oappRegistry] = await getOAppRegistry(OApp);

        const [eventAuthority] = await getEventAuthority();

        const [nonce] = await getNonce(OApp, remoteEid, remoteOapp);

        const [pendingInboundNonce] = await getPendingNonce(OApp, remoteEid, remoteOapp);

        const [sendLibraryConfig] = await getSendLibraryConfig(OApp, eid);

        const [receiveLibraryConfig] = await getReceiveLibraryConfig(OApp, eid);

        console.log(`delegate pubkey: ${delegate.publicKey}`);

        await airdrop(program.provider.connection, user.publicKey);
        await airdrop(program.provider.connection, delegate.publicKey);

        const depositAmount = new BN(10000 * LAMPORTS_PER_SOL);

        const init_tx= await program.methods
            .deposit({
                amountIn: depositAmount
            })
            .accounts({
                depositor: user.publicKey,
                vault,
                mint,
                config,
                depositorTokenAccount: userTokenAccount,
                poolTokenAccount,
                tokenProgram: TOKEN_PROGRAM_ID,
                endpointProgram: ENDPOINT_PROGRAM_ID
            })
            .remainingAccounts([
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: ENDPOINT_PROGRAM_ID
                },
                {
                    isSigner: true,
                    isWritable: true,
                    pubkey: user.publicKey
                },
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: config
                },
                {
                    isSigner: false,
                    isWritable: true,
                    pubkey: oappRegistry
                },
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: SYSTEM_PROGRAM_ID
                },
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: eventAuthority
                },
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: ENDPOINT_PROGRAM_ID
                }

            ])
            .signers([user])
            .rpc()
            .catch((e) => {console.log(e)});

        console.log("Your transaction signature", init_tx);

        const VaultState = await program.account.vault.fetch(vault);

        assert.equal(VaultState.depositBalance.toString(), depositAmount.toString(), "Deposit failed");
    });
});
