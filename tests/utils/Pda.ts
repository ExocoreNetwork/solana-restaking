import * as anchor from "@coral-xyz/anchor";

import { PublicKey } from "@solana/web3.js";
import {LST_RESTAKING_PROGRAM_ID, ENDPOINT_PROGRAM_ID} from "./Consts";
import BN from "bn.js";
import {TOKEN_PROGRAM_ID} from "@coral-xyz/anchor/dist/cjs/utils/token";
import {getAssociatedTokenAddressSync} from "@solana/spl-token";

const LST_RESTAKING_CONFIG_PREFIX = Buffer.from(
  anchor.utils.bytes.utf8.encode("config")
);
const LST_RESTAKING_VAULT_PREFIX = Buffer.from(
  anchor.utils.bytes.utf8.encode("vault")
);

const LST_RESTAKING_SENDER_PREFIX = Buffer.from(
    anchor.utils.bytes.utf8.encode("sender")
);

const OAPP_SEEDS= Buffer.from(
    anchor.utils.bytes.utf8.encode("OApp")
);

const NONCE_SEEDS= Buffer.from(
    anchor.utils.bytes.utf8.encode("Nonce")
);

const PENDING_NONCE_SEEDS= Buffer.from(
    anchor.utils.bytes.utf8.encode("PendingNonce")
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
export async function getConfig(): Promise<[PublicKey, number]> {
  const [address, bump] = PublicKey.findProgramAddressSync(
    [LST_RESTAKING_CONFIG_PREFIX],
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


export async function getTokenAccount(
    owner: PublicKey,
    mint: PublicKey,
    offCurve = false
): Promise<PublicKey> {
    return getAssociatedTokenAddressSync(
        mint,
        owner,
        offCurve
    );
}

export async function getOAppRegistry(localOApp: PublicKey): Promise<[PublicKey, number]> {
  const [address, bump] = PublicKey.findProgramAddressSync(
      [OAPP_SEEDS, localOApp.toBuffer()],
      ENDPOINT_PROGRAM_ID
  );

    console.log(`OAppRegistry: ${address}`);
  return [address, bump]
}

export async function getEventAuthority(): Promise<[PublicKey, number]> {
    const [address, bump] = PublicKey.findProgramAddressSync(
        [EVENT_AUTHORITY],
        ENDPOINT_PROGRAM_ID
    );

    console.log(`EventAuthority: ${address}`);
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

  const [nonceAddress, bump] = PublicKey.findProgramAddressSync(
      [
        NONCE_SEEDS,
        localOAppBytes,
        eid.toBuffer("be", 4),
        Buffer.from(remoteOApp),
      ],
      ENDPOINT_PROGRAM_ID
  );

    console.log(`nonce: ${nonceAddress}`);
  return [nonceAddress, bump];
}

export async function getPendingNonce(localOApp: PublicKey, remoteEid: number, remoteOApp: number[]): Promise<[PublicKey, number]> {
  const localOAppBytes = localOApp.toBuffer();

  const remoteEidBytes = Buffer.alloc(4);
  remoteEidBytes.writeUInt32BE(remoteEid);

  if (remoteOApp.length !== 32) {
    throw new Error("remoteOApp must be exactly 32 bytes");
  }

  const [pendingNonceAddress, bump] = PublicKey.findProgramAddressSync(
      [
        PENDING_NONCE_SEEDS,
        localOAppBytes,
        remoteEidBytes,
        Buffer.from(remoteOApp),
      ],
      ENDPOINT_PROGRAM_ID
  );

    console.log(`pendingNonce: ${pendingNonceAddress}`);
  return [pendingNonceAddress, bump];
}

export async function getSendLibraryConfig(sender: PublicKey, eid: number): Promise<[PublicKey, number]> {
    const eidBytes = Buffer.alloc(4);
    eidBytes.writeUInt32BE(eid);

    const [sendLibraryConfigAddress, bump] = PublicKey.findProgramAddressSync(
        [
            SEND_LIBRARY_CONFIG,
            sender.toBuffer(),
            eidBytes,
        ],
        ENDPOINT_PROGRAM_ID
    );

    console.log(`sendLibraryConfig: ${sendLibraryConfigAddress}`);
    return [sendLibraryConfigAddress, bump];
}

export async function getReceiveLibraryConfig(receiver: PublicKey, eid: number): Promise<[PublicKey, number]> {
    const [receiveLibraryConfigAddress, bump] = PublicKey.findProgramAddressSync(
        [
            RECEIVE_LIBRARY_CONFIG,
            receiver.toBuffer(),
            new BN(eid).toBuffer("be", 4),
        ],
        ENDPOINT_PROGRAM_ID
    );

    console.log(`receiveLibraryConfig: ${receiveLibraryConfigAddress}`);
    return [receiveLibraryConfigAddress, bump];
}