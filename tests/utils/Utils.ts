import { PublicKey, LAMPORTS_PER_SOL, Connection, Signer } from "@solana/web3.js";
import {
    NATIVE_MINT,
    createMint,
    getAssociatedTokenAddressSync,
    getOrCreateAssociatedTokenAccount, mintTo
} from "@solana/spl-token";
import {TOKEN_PROGRAM_ID} from "@coral-xyz/anchor/dist/cjs/utils/token";

// airdrop
export const airdrop = async (conn: Connection, user: PublicKey) => {
  const signature = await conn.requestAirdrop(user, 50 * LAMPORTS_PER_SOL);
  await conn.confirmTransaction(signature, "confirmed");

};

export const createTestMint = async (connection: Connection, mintAuthority: Signer) => {
    return await createMint(
        connection,
        mintAuthority,
        mintAuthority.publicKey,
        null,
        9
    );
}

export const getPDATokenAccount = async (mint: PublicKey, owner: PublicKey) => {
    return getAssociatedTokenAddressSync(mint, owner, true);
}

export const getTokenAccount = async (
    connection: Connection,
    mint: PublicKey,
    owner: PublicKey,
    payer: Signer,
    mintAuthority: Signer,
): Promise<PublicKey> => {
    const tokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mint,
        owner,
    );

    await mintTo(
        connection,
        payer,
        mint,
        tokenAccount.address,
        mintAuthority,
        100_000_000_000_000,
    );

    return tokenAccount.address;
}