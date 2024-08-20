import * as anchor from "@coral-xyz/anchor";

import { PublicKey } from "@solana/web3.js";
import { LST_RESTAKING_PROGRAM_ID } from "./Consts";

const LST_RESTAKING_CONFIG_PREFIX = Buffer.from(
  anchor.utils.bytes.utf8.encode("config")
);
const LST_RESTAKING_VAULT_PREFIX = Buffer.from(
  anchor.utils.bytes.utf8.encode("vault")
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
