import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import {LstRestaking} from "../../target/types/lst_restaking";

import {
    airdrop,
    DVN_CONFIG,
    DVN_PROGRAM_ID,
    eid,
    ENDPOINT_EVENT_AUTHORITY,
    ENDPOINT_PROGRAM_ID,
    ENDPOINT_SETTINGS,
    EXECUTOR_CONFIG,
    EXECUTOR_PROGRAM_ID,
    getConfig,
    getDefaultSendConfig,
    getDefaultSendLibraryConfig,
    getMessageLibInfo,
    getNonce,
    getOApp,
    getOAppRegistry, getPDATokenAccount,
    getPendingNonce,
    getReceiveLibraryConfig,
    getSendConfig,
    getSendLibraryConfig,
    getTokenAccount,
    getVault,
    PRICE_FEED_CONFIG,
    PRICE_FEED_PROGRAM_ID,
    remoteEid,
    remoteOapp, SEND_LIBRARY_INFO,
    SYSTEM_PROGRAM_ID,
    testKeys,
    ULN302_PROGRAM_ID,
    ULN_EVENT_AUTHORITY,
    ULN_SETTINGS,
} from "../utils";
import {assert} from "chai";
import {LAMPORTS_PER_SOL} from "@solana/web3.js";
import BN from "bn.js";
import {TOKEN_PROGRAM_ID} from "@coral-xyz/anchor/dist/cjs/utils/token";
import {config} from "dotenv";
import {getAssociatedTokenAddressSync} from "@solana/spl-token";

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

    // const endpoint_program = new Program(IDL, ENDPOINT_PROGRAM_ID, provider);

    it("Deposit", async () => {
        const [owner , user, , , delegate] = await testKeys();

        const mint = new web3.PublicKey(process.env.MINT_ADDRESS);

        const [config] = await getConfig();

        const [vault] = await getVault(mint, user.publicKey);

        const poolTokenAccount = await getPDATokenAccount(mint, config);

        console.log(`poolTokenAccount: ${poolTokenAccount}`);

        const [OApp] = await getOApp();

        const [oappRegistry] = await getOAppRegistry(OApp);

        const [nonce] = await getNonce(OApp, remoteEid, remoteOapp);

        const [pendingInboundNonce] = await getPendingNonce(OApp, remoteEid, remoteOapp);

        const [defaultSendLibraryConfig] = await getDefaultSendLibraryConfig(remoteEid);

        const [sendLibraryConfig] = await getSendLibraryConfig(OApp, remoteEid);

        const [receiveLibraryConfig] = await getReceiveLibraryConfig(OApp, eid);

        const [sendConfig] = await getSendConfig(OApp, remoteEid);

        const [defaultSendConfig] = await getDefaultSendConfig(remoteEid);

        console.log(`delegate pubkey: ${delegate.publicKey}`);

        await airdrop(program.provider.connection, user.publicKey);
        await airdrop(program.provider.connection, delegate.publicKey);

        const depositAmount = 10000 * LAMPORTS_PER_SOL;

        const userTokenAccount = await getTokenAccount(anchor.getProvider().connection, mint, user.publicKey, user, owner);

        const init_tx = await program.methods
            .deposit({
                amountIn: new BN(depositAmount)
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
                // endpoint program 0
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: ENDPOINT_PROGRAM_ID
                },
                // sender 1
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: config
                },
                // sender library program 2
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: ULN302_PROGRAM_ID
                },
                // Send Library Config 3
                {
                    isSigner: false,
                    isWritable: true,
                    pubkey: sendLibraryConfig
                },
                // Default Send Library Config 4
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: defaultSendLibraryConfig
                },
                // Send Library Info 5
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: SEND_LIBRARY_INFO
                },
                // Endpoint 6
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: ENDPOINT_SETTINGS
                },
                // Nonce writable 7
                {
                    isSigner: false,
                    isWritable: true,
                    pubkey: nonce
                },
                // Event Authority 8
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: ENDPOINT_EVENT_AUTHORITY
                },
                // Endpoint Program 9
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: ENDPOINT_PROGRAM_ID
                },
                // UlnSettings 10
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: ULN_SETTINGS
                },
                // Send Config 11
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: sendConfig
                },
                // Default Send Config 12
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: defaultSendConfig
                },
                // payer writable 13
                {
                    isSigner: true,
                    isWritable: true,
                    pubkey: user.publicKey
                },
                // Treasury 14
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: ULN302_PROGRAM_ID
                },
                // System 15
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: SYSTEM_PROGRAM_ID
                },
                // Uln Event Authority 16
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: ULN_EVENT_AUTHORITY
                },
                // ULN program 17
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: ULN302_PROGRAM_ID
                },
                // Excutor program 18
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: EXECUTOR_PROGRAM_ID
                },
                // Executor Config 19
                {
                    isSigner: false,
                    isWritable: true,
                    pubkey: EXECUTOR_CONFIG
                },
                // Price Feed Program 20
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: PRICE_FEED_PROGRAM_ID
                },
                // Price Feed Config 21
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: PRICE_FEED_CONFIG
                },
                // Dvn 22
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: DVN_PROGRAM_ID
                },
                // Dvn Config 23
                {
                    isSigner: false,
                    isWritable: true,
                    pubkey: DVN_CONFIG
                },
                // Price Feed Program 24
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: PRICE_FEED_PROGRAM_ID
                },
                // Price Feed Config 25
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: PRICE_FEED_CONFIG
                }
            ])
            .signers([user])
            .rpc()
            .catch((e) => {
                console.log(e)
            });

        console.log("Your transaction signature", init_tx);


        const VaultState = await program.account.vault.fetch(vault);

        assert.equal(VaultState.depositBalance.toString(), depositAmount.toString(), "Deposit failed");
    });
});
