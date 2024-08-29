import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import {
  createAssociatedTokenAccountInstruction,
  createInitializeMintInstruction,
} from "@solana/spl-token";
import { PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { LstRestaking } from "../../target/types/lst_restaking";

// airdrop
export const airdrop = async (conn: web3.Connection, user: PublicKey) => {
  const signature = await conn.requestAirdrop(user, LAMPORTS_PER_SOL);
  await conn.confirmTransaction(signature, "confirmed");

};

// create mint
export const createMint = async (
  program: Program<LstRestaking>,
  payer: web3.Keypair,
  decimals: number = 9
) => {
  const mint = anchor.web3.Keypair.generate();
  const tx = new web3.Transaction().add(
    web3.SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: mint.publicKey,
      space: 82, // 82 bytes for mint account
      lamports:
        await program.provider.connection.getMinimumBalanceForRentExemption(82),
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMintInstruction(
      mint.publicKey,
      decimals,
      payer.publicKey,
      payer.publicKey,
      TOKEN_PROGRAM_ID
    )
  );

  await program.provider.sendAndConfirm(tx, [payer, mint]);

  console.log(`created new mint: ${mint.publicKey}`);

  return mint.publicKey;
};

// create token for user
export const createOrGetTokenAccount = async (
  program: Program,
  mint: PublicKey,
  user: PublicKey
): Promise<PublicKey> => {
  const tokenAccount = anchor.utils.token.associatedAddress({
    mint,
    owner: user,
  });

  const accountInfo = await program.provider.connection.getAccountInfo(
    tokenAccount
  );

  if (accountInfo === null) {
    // Account does not exist, create it
    const tx = new web3.Transaction().add(
      createAssociatedTokenAccountInstruction(
        program.provider.publicKey, // payer
        tokenAccount, // associated token account
        user, // owner of the account
        mint // mint
      )
    );

    await program.provider.sendAndConfirm(tx);
  }

  return tokenAccount;
};
