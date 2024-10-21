import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { IDL as LstRestakingIdl } from "../../target/types/lst_restaking";
import {
    getConfig, getDefaultReceiveConfig, getDefaultSendConfig,
    getDefaultSendLibraryConfig,
    getLZReceiveTypes, getMessageLib, getMessageLibInfo,
    getMessage,
    getNonce,
    getOApp,
    getOAppRegistry,
    getPendingNonce,
    getReceiveConfig,
    getReceiveLibraryConfig,
    getSendConfig,
    getSendLibraryConfig,
    LST_RESTAKING_PROGRAM_ID,
    testKeys, remotePeer,
} from "../utils";
import {Connection, Keypair, PublicKey, Signer, TransactionInstruction} from "@solana/web3.js";
import {EndpointId} from "@layerzerolabs/lz-definitions";
import {
    buildVersionedTransaction, EndpointProgram,
    ExecutorPDADeriver,
    SetConfigType,
    UlnProgram
} from "@layerzerolabs/lz-solana-sdk-v2";
import {arrayify, hexZeroPad} from "@ethersproject/bytes";

describe("solana-restaking", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();

    // Set the provider with customized options
    const customProvider = new anchor.AnchorProvider(
        provider.connection,
        provider.wallet,
        {
            preflightCommitment: "processed", // or "confirmed" / "finalized"
            commitment: "confirmed", // or "finalized"
        }
    );

    anchor.setProvider(customProvider);

    const lst_program = new Program(
        LstRestakingIdl,
        LST_RESTAKING_PROGRAM_ID,
        provider
    );

    const endpointProgram = new EndpointProgram.Endpoint(new PublicKey('76y77prsiCMvXMjuoZ5VRrhG5qYBrUMYTE5WgHqgjEn6')) // endpoint program id, mainnet and testnet are the same
    const ulnProgram = new UlnProgram.Uln(new PublicKey('7a4WjyR8VZ7yZz5XJAKm39BUGn5iT9CKcv2pmG9tdXVH')) // uln program id, mainnet and testnet are the same
    const executorProgram = new PublicKey('6doghB248px58JSSwG4qejQ46kFMW4AMj7vzJnWZHNZn') // executor program id, mainnet and testnet are the same

    it("Is initialized!", async () => {
        const [owner, new_owner, user, invalid_owner, another_user, delegate, dev] =
            await testKeys();

        console.log(`delegate pubkey: ${delegate.publicKey}`);

        const conn = anchor.getProvider().connection as unknown as Connection;

        const remotePeerBytes = arrayify(hexZeroPad(remotePeer, 32))

        await initSendLibrary(conn, delegate as unknown as Keypair, EndpointId.EXOCORE_V2_TESTNET)
        await initReceiveLibrary(conn, delegate as unknown as Keypair, EndpointId.EXOCORE_V2_TESTNET)
        await initOappNonce(conn, delegate as unknown as Keypair, EndpointId.EXOCORE_V2_TESTNET, remotePeerBytes)
        await setSendLibrary(conn, delegate as unknown as Keypair, EndpointId.EXOCORE_V2_TESTNET)
        await setReceiveLibrary(conn, delegate as unknown as Keypair, EndpointId.EXOCORE_V2_TESTNET)
        await initUlnConfig(conn, delegate as unknown as Keypair, delegate as unknown as Keypair, EndpointId.EXOCORE_V2_TESTNET)
        await setOappExecutor(conn, delegate as unknown as Keypair, EndpointId.EXOCORE_V2_TESTNET);

    });

    async function initSendLibrary(connection: Connection, admin: Keypair, remote: EndpointId): Promise<void> {
        const [id] = await getOApp();
        const ix = await endpointProgram.initSendLibrary(connection, admin.publicKey, id, remote)
        if (ix == null) {
            return Promise.resolve()
        }
        await sendAndConfirm(connection, [admin], [ix])
        console.log(`initSendLibrary is complete`)
    }

    async function initReceiveLibrary(connection: Connection, admin: Keypair, remote: EndpointId): Promise<void> {
        const [id] = await getOApp();
        const ix = await endpointProgram.initReceiveLibrary(connection, admin.publicKey, id, remote)
        if (ix == null) {
            return Promise.resolve()
        }
        await sendAndConfirm(connection, [admin], [ix])
        console.log(`initSendLibrary is complete`)
    }

    async function initOappNonce(
        connection: Connection,
        admin: Keypair,
        remote: EndpointId,
        remotePeer: Uint8Array
    ): Promise<void> {
        const [id] = await getOApp();
        const ix = await endpointProgram.initOAppNonce(connection, admin.publicKey, remote, id, remotePeer)
        if (ix === null) return Promise.resolve()
        const current = false
        try {
            const nonce = await endpointProgram.getNonce(connection, id, remote, remotePeer)
            if (nonce) {
                console.log('nonce already set')
                return Promise.resolve()
            }
        } catch (e) {
            /*nonce not init*/
        }
        await sendAndConfirm(connection, [admin], [ix])
        console.log(`initOappNonce is complete`)
    }

    async function setSendLibrary(connection: Connection, admin: Keypair, remote: EndpointId): Promise<void> {
        const [id] = await getOApp();
        const sendLib = await endpointProgram.getSendLibrary(connection, id, remote)
        const current = sendLib ? sendLib.msgLib.toBase58() : ''
        const [expectedSendLib] = ulnProgram.deriver.messageLib()
        const expected = expectedSendLib.toBase58()
        if (current === expected) {
            console.log(`==`)
            return Promise.resolve()
        }
        const ix = await endpointProgram.setSendLibrary(admin.publicKey, id, ulnProgram.program, remote)
        await sendAndConfirm(connection, [admin], [ix])
        console.log(`setSendLibrary is complete`)
    }

    async function setReceiveLibrary(connection: Connection, admin: Keypair, remote: EndpointId): Promise<void> {
        const [id] = await getOApp();
        const receiveLib = await endpointProgram.getReceiveLibrary(connection, id, remote)
        const current = receiveLib ? receiveLib.msgLib.toBase58() : ''
        const [expectedMessageLib] = ulnProgram.deriver.messageLib()
        const expected = expectedMessageLib.toBase58()
        if (current === expected) {
            console.log(`==`)
            return Promise.resolve()
        }
        const ix = await endpointProgram.setReceiveLibrary(admin.publicKey, id, ulnProgram.program, remote)
        await sendAndConfirm(connection, [admin], [ix])
        console.log(`setReceiveLibrary is complete`)
    }

    async function initUlnConfig(
        connection: Connection,
        payer: Keypair,
        admin: Keypair,
        remote: EndpointId
    ): Promise<void> {
        const [id] = await getOApp();

        const current = await ulnProgram.getSendConfigState(connection, id, remote)
        if (current) {
            return Promise.resolve()
        }
        const ix = await endpointProgram.initOappConfig(admin.publicKey, ulnProgram, payer.publicKey, id, remote)
        await sendAndConfirm(connection, [admin], [ix])
        console.log(`initUlnConfig is complete`)
    }
    async function setOappExecutor(connection: Connection, admin: Keypair, remote: EndpointId): Promise<void> {


        const [id] = await getOApp();
        const defaultOutboundMaxMessageSize = 10000

        const [executorPda] = new ExecutorPDADeriver(executorProgram).config()
        const expected: UlnProgram.types.ExecutorConfig = {
            maxMessageSize: defaultOutboundMaxMessageSize,
            executor: executorPda,
        }

        const current = (await ulnProgram.getSendConfigState(connection, id, remote))?.executor
        const ix = await endpointProgram.setOappConfig(connection, admin.publicKey, id, ulnProgram.program, remote, {
            configType: SetConfigType.EXECUTOR,
            value: expected,
        })
        if (
            current &&
            current.executor.toBase58() === expected.executor.toBase58() &&
            current.maxMessageSize === expected.maxMessageSize
        ) {
            return Promise.resolve()
        }
        await sendAndConfirm(connection, [admin], [ix])
        console.log(`setOappExecutor is complete`)
    }

    async function sendAndConfirm(
        connection: Connection,
        signers: Signer[],
        instructions: TransactionInstruction[]
    ): Promise<void> {
        const tx = await buildVersionedTransaction(connection, signers[0].publicKey, instructions, 'confirmed')
        tx.sign(signers)
        const hash = await connection.sendTransaction(tx, { skipPreflight: true })
        await connection.confirmTransaction(hash, 'processed')
    }
});
