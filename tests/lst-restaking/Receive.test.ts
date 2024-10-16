import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { IDL as EndpointIdl } from "../../target/types/endpoint";
import { IDL as LstRestakingIdl } from "../../target/types/lst_restaking";
import {
    eid,
    ENDPOINT_EVENT_AUTHORITY,
    ENDPOINT_PROGRAM_ID,
    getConfig,
    getDefaultSendLibraryConfig,
    getLZReceiveTypes,
    getMessages,
    getNonce,
    getOApp,
    getOAppRegistry,
    getPendingNonce, getReceiveConfig,
    getReceiveLibraryConfig, getSendConfig,
    getSendLibraryConfig,
    getTokens, hexToUint8Array,
    LST_RESTAKING_PROGRAM_ID,
    remoteEid,
    remoteOapp, SEND_LIBRARY_INFO,
    SYSTEM_PROGRAM_ID,
    testKeys, ULN302_PROGRAM_ID, ULN_SETTINGS,
} from "../utils";
import { assert } from "chai";
import { Connection } from "@solana/web3.js";
import BN from "bn.js";

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

    // const lst_program = anchor.workspace.LstRestaking as Program<LstRestaking>;
    // const endpoint_program = anchor.workspace.Endpoint as Program<Endpoint>;

    const lst_program = new Program(LstRestakingIdl, LST_RESTAKING_PROGRAM_ID, provider);
    const endpoint_program = new Program(EndpointIdl, ENDPOINT_PROGRAM_ID, provider);

    it("Is initialized!", async () => {
        const [owner, new_owner, user, invalid_owner, another_user, delegate, dev] = await testKeys();

        const [config] = await getConfig();

        const [OApp] = await getOApp();

        const [tokens] = await getTokens();

        const [oappRegistry] = await getOAppRegistry(OApp);

        const [nonce] = await getNonce(OApp, remoteEid, remoteOapp);

        const [pendingInboundNonce] = await getPendingNonce(
            OApp,
            remoteEid,
            remoteOapp
        );

        const [sendLibraryConfig] = await getSendLibraryConfig(OApp, remoteEid);

        const [defaultSendLibraryConfig] = await getDefaultSendLibraryConfig(
            remoteEid
        );

        const [sendConfig] = await getSendConfig(OApp, eid);

        const [receiveConfig] = await getReceiveConfig(OApp, eid);

        const [receiveLibraryConfig] = await getReceiveLibraryConfig(OApp, eid);

        const [messages] = await getMessages(config);

        const [lzReceiveTypes] = await getLZReceiveTypes(OApp);

        console.log(`delegate pubkey: ${delegate.publicKey}`);

        const message = hexToUint8Array("0x07b4538cd1218f9ba9a1e3447b59a1b359c59989e1458fc4f8fdb8b1918d5bdd62");

        const conn = anchor.getProvider().connection as unknown as Connection;

        const init_tx = await lst_program.methods
            .lzReceive({
                srcEid: remoteEid,
                sender: remoteOapp,
                nonce: new BN(1),
                guid: [],
                message: Buffer.from(message),
                extraData: undefined
            })
            .accounts({
                payer: owner.publicKey,
                config,
            })
            .remainingAccounts([
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: ENDPOINT_PROGRAM_ID,
                },
                {
                    isSigner: true,
                    isWritable: true,
                    pubkey: owner.publicKey,
                },
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: config,
                },
                {
                    isSigner: false,
                    isWritable: true,
                    pubkey: oappRegistry,
                },
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: SYSTEM_PROGRAM_ID,
                },
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: ENDPOINT_EVENT_AUTHORITY,
                },
                {
                    isSigner: false,
                    isWritable: false,
                    pubkey: ENDPOINT_PROGRAM_ID,
                },
            ])
            .signers([owner])
            .rpc()
            .catch((e) => {
                console.log(e);
            });

        console.log("Your transaction signature", init_tx);



        let defaultSendLibraryConfigState =
            await endpoint_program.account.sendLibraryConfig.fetch(
                defaultSendLibraryConfig
            );

        console.log(
            `defaultSendLibraryConfig: ${defaultSendLibraryConfigState.messageLib}`
        );

        const configState = await lst_program.account.config.fetch(config);

        assert.equal(
            configState.owner.toString(),
            owner.publicKey.toString(),
            "Initialize failed"
        );

        const lzReceiveTypesState = await lst_program.account.lzReceiveTypesAccount.fetch(
            lzReceiveTypes
        );

        console.log(lzReceiveTypesState);
    });
});
