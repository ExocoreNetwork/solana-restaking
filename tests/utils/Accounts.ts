import {
    DVN_CONFIG,
    DVN_PROGRAM_ID, eid,
    ENDPOINT_EVENT_AUTHORITY,
    ENDPOINT_PROGRAM_ID,
    ENDPOINT_SETTINGS,
    EXECUTOR_CONFIG,
    EXECUTOR_PROGRAM_ID,
    PRICE_FEED_CONFIG,
    PRICE_FEED_PROGRAM_ID,
    remoteEid,
    remoteOapp,
    SEND_LIBRARY_INFO,
    SYSTEM_PROGRAM_ID,
    ULN302_PROGRAM_ID,
    ULN_EVENT_AUTHORITY,
    ULN_SETTINGS
} from "./Consts";
import {
    getDefaultSendConfig,
    getDefaultSendLibraryConfig,
    getNonce,
    getOApp,
    getSendConfig,
    getSendLibraryConfig
} from "./Pda";
import {PublicKey} from "@solana/web3.js";




export const sendRemainingAccounts = async (user: PublicKey) => {
    const [OApp] = await getOApp();

    const [nonce] = await getNonce(OApp);

    const [defaultSendLibraryConfig] = await getDefaultSendLibraryConfig();

    const [sendLibraryConfig] = await getSendLibraryConfig(OApp);

    const [sendConfig] = await getSendConfig(OApp);

    const [defaultSendConfig] = await getDefaultSendConfig();

    return [
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
            pubkey: OApp
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
            pubkey: user
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
    ];
}