import * as anchor from "@coral-xyz/anchor";

import { PublicKey } from "@solana/web3.js";
import {LST_RESTAKING_PROGRAM_ID, ENDPOINT_PROGRAM_ID, ULN302_PROGRAM_ID, EXECUTOR_PROGRAM_ID} from "./Consts";
import BN from "bn.js";
import {TOKEN_PROGRAM_ID} from "@coral-xyz/anchor/dist/cjs/utils/token";
import {getAssociatedTokenAddressSync} from "@solana/spl-token";

const LST_RESTAKING_CONFIG_PREFIX = Buffer.from(
  anchor.utils.bytes.utf8.encode("config")
);
const LST_RESTAKING_VAULT_PREFIX = Buffer.from(
  anchor.utils.bytes.utf8.encode("vault")
);

const LST_RESTAKING_MESSAGE_LIST_PREFIX = Buffer.from(
    anchor.utils.bytes.utf8.encode("message-list")
);

const LST_RESTAKING_TOKEN_WHITE_LIST= Buffer.from(
    anchor.utils.bytes.utf8.encode("tokenWhiteList")
);

const OAPP_SEEDS= Buffer.from(
    anchor.utils.bytes.utf8.encode("OApp")
);

const NONCE_SEEDS= Buffer.from(
    anchor.utils.bytes.utf8.encode("Nonce")
);

const ENDPOINT_SEEDS= Buffer.from(
    anchor.utils.bytes.utf8.encode("Endpoint")
);

const PENDING_NONCE_SEEDS= Buffer.from(
    anchor.utils.bytes.utf8.encode("PendingNonce")
);

const MESSAGE_LIB_SEEDS= Buffer.from(
    anchor.utils.bytes.utf8.encode("MessageLib")
);

const SEND_LIBRARY_CONFIG= Buffer.from(
    anchor.utils.bytes.utf8.encode("SendLibraryConfig")
);

const RECEIVE_LIBRARY_CONFIG= Buffer.from(
    anchor.utils.bytes.utf8.encode("ReceiveLibraryConfig")
);

const EVENT_AUTHORITY= Buffer.from(
    anchor.utils.bytes.utf8.encode("__event_authority")
);

const SEND_CONFIG_SEED= Buffer.from(
    anchor.utils.bytes.utf8.encode("SendConfig")
);

const LZ_RECEIVE_TYPES_SEED= Buffer.from(
    anchor.utils.bytes.utf8.encode("LzReceiveTypes")
);
export async function getConfig(): Promise<[PublicKey, number]> {
  const [address, bump] = PublicKey.findProgramAddressSync(
    [LST_RESTAKING_CONFIG_PREFIX],
    LST_RESTAKING_PROGRAM_ID
  );

  return [address, bump];
}

export async function getMessageList(config: PublicKey): Promise<[PublicKey, number]> {
    const [address, bump] = PublicKey.findProgramAddressSync(
        [LST_RESTAKING_MESSAGE_LIST_PREFIX, config.toBuffer()],
        LST_RESTAKING_PROGRAM_ID
    );

    return [address, bump];
}

export async function getVault(
  mint: PublicKey,
  user: PublicKey
): Promise<[PublicKey, number]> {
  const [address, bump] = PublicKey.findProgramAddressSync(
    [LST_RESTAKING_VAULT_PREFIX, mint.toBuffer(), user.toBuffer()],
    LST_RESTAKING_PROGRAM_ID
  );

  return [address, bump];
}

export async function getOApp(): Promise<[PublicKey, number]> {
    const [address, bump] = PublicKey.findProgramAddressSync(
        [LST_RESTAKING_CONFIG_PREFIX],
        LST_RESTAKING_PROGRAM_ID
    );

    console.log(`OApp: ${address}`);

    return [address, bump]
}

export async function getTokens(): Promise<[PublicKey, number]> {
    const [address, bump] = PublicKey.findProgramAddressSync(
        [LST_RESTAKING_TOKEN_WHITE_LIST],
        LST_RESTAKING_PROGRAM_ID
    );

    console.log(`TokenWhiteList: ${address}`);

    return [address, bump]
}

export async function getLZReceiveTypes(OApp: PublicKey): Promise<[PublicKey, number]> {
    const [address, bump] = PublicKey.findProgramAddressSync(
        [LZ_RECEIVE_TYPES_SEED, OApp.toBuffer()],
        LST_RESTAKING_PROGRAM_ID
    );

    console.log(`LZReceiveTypes: ${address}`);

    return [address, bump]
}

//////////////////////////////Endpoint program/////////////////////////////
export async function getOAppRegistry(localOApp: PublicKey): Promise<[PublicKey, number]> {
  const [address, bump] = PublicKey.findProgramAddressSync(
      [OAPP_SEEDS, localOApp.toBuffer()],
      ENDPOINT_PROGRAM_ID
  );

    console.log(`OAppRegistry: ${address}`);
  return [address, bump]
}

export async function getMessageLibInfo(lib: PublicKey): Promise<[PublicKey, number]> {
    const [address, bump] = PublicKey.findProgramAddressSync(
        [MESSAGE_LIB_SEEDS, lib.toBuffer()],
        ENDPOINT_PROGRAM_ID
    );

    console.log(`MessageLibInfo: ${address}`);
    return [address, bump]
}

export async function getNonce(localOApp: PublicKey, remoteEid: number, remoteOApp: number[]): Promise<[PublicKey, number]> {
  const localOAppBytes = localOApp.toBuffer();

  // const remoteEidBytes = Buffer.alloc(4);
  // remoteEidBytes.writeUInt32BE(remoteEid);

    const eid = new BN(remoteEid);

  if (remoteOApp.length !== 32) {
    throw new Error("remoteOApp must be exactly 32 bytes");
  }

  const [address, bump] = PublicKey.findProgramAddressSync(
      [
        NONCE_SEEDS,
        localOAppBytes,
        eid.toBuffer("be", 4),
        Buffer.from(remoteOApp),
      ],
      ENDPOINT_PROGRAM_ID
  );

    console.log(`nonce: ${address}`);
  return [address, bump];
}

export async function getPendingNonce(localOApp: PublicKey, remoteEid: number, remoteOApp: number[]): Promise<[PublicKey, number]> {
  const localOAppBytes = localOApp.toBuffer();

  const remoteEidBytes = Buffer.alloc(4);
  remoteEidBytes.writeUInt32BE(remoteEid);

  if (remoteOApp.length !== 32) {
    throw new Error("remoteOApp must be exactly 32 bytes");
  }

  const [address, bump] = PublicKey.findProgramAddressSync(
      [
        PENDING_NONCE_SEEDS,
        localOAppBytes,
        remoteEidBytes,
        Buffer.from(remoteOApp),
      ],
      ENDPOINT_PROGRAM_ID
  );

    console.log(`pendingNonce: ${address}`);
  return [address, bump];
}

export async function getSendLibraryConfig(sender: PublicKey, eid: number): Promise<[PublicKey, number]> {
    const eidBytes = Buffer.alloc(4);
    eidBytes.writeUInt32BE(eid);

    const [address, bump] = PublicKey.findProgramAddressSync(
        [
            SEND_LIBRARY_CONFIG,
            sender.toBuffer(),
            eidBytes,
        ],
        ENDPOINT_PROGRAM_ID
    );

    console.log(`sendLibraryConfig: ${address}`);
    return [address, bump];
}

export async function getDefaultSendLibraryConfig(eid: number): Promise<[PublicKey, number]> {
    const eidBytes = Buffer.alloc(4);
    eidBytes.writeUInt32BE(eid);

    const [address, bump] = PublicKey.findProgramAddressSync(
        [
            SEND_LIBRARY_CONFIG,
            eidBytes,
        ],
        ENDPOINT_PROGRAM_ID
    );

    console.log(`defaultSendLibraryConfig: ${address}`);
    return [address, bump];
}

export async function getReceiveLibraryConfig(receiver: PublicKey, eid: number): Promise<[PublicKey, number]> {
    const [address, bump] = PublicKey.findProgramAddressSync(
        [
            RECEIVE_LIBRARY_CONFIG,
            receiver.toBuffer(),
            new BN(eid).toBuffer("be", 4),
        ],
        ENDPOINT_PROGRAM_ID
    );

    console.log(`receiveLibraryConfig: ${address}`);
    return [address, bump];
}

// export async function getLZReceiveTypes(OApp: PublicKey): Promise<[PublicKey, number]> {
//     const [address, bump] = PublicKey.findProgramAddressSync(
//         [LZ_RECEIVE_TYPES_SEED, OApp.toBuffer()],
//         ENDPOINT_PROGRAM_ID
//     );
//
//     console.log(`LZReceiveTypes: ${address}`);
//
//     return [address, bump]
// }

//////////////////////////////Uln program/////////////////////////////
export async function getSendConfig(sender: PublicKey, dstEid: number): Promise<[PublicKey, number]> {
    const [address, bump] = PublicKey.findProgramAddressSync(
        [
            SEND_CONFIG_SEED,
            new BN(dstEid).toBuffer("be", 4),
            sender.toBuffer()
        ],
        ULN302_PROGRAM_ID
    );

    console.log(`SendConfig: ${address}`);
    return [address, bump];
}

export async function getDefaultSendConfig(dstEid: number): Promise<[PublicKey, number]> {
    const [address, bump] = PublicKey.findProgramAddressSync(
        [
            SEND_CONFIG_SEED,
            new BN(dstEid).toBuffer("be", 4),
        ],
        ULN302_PROGRAM_ID
    );

    console.log(`DefaultSendConfig: ${address}`);
    return [address, bump];
}


