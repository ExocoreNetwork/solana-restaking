import * as anchor from "@coral-xyz/anchor";
import * as fs from "fs/promises";
import * as path from "path";

export const writeKeypairToFile = async (
  sk: Uint8Array,
  fileName: string
): Promise<void> => {
  const filePath = path.join(".keys/", `${fileName}.json`);

  try {
    await fs.writeFile(filePath, JSON.stringify(Array.from(sk)));
    console.log(`Keypair written to file: ${filePath}`);
  } catch (error) {
    console.error(`Error writing keypair to file: ${(error as Error).message}`);
  }
};

export const readKeypairToFile = async (
  fileName: string
): Promise<anchor.web3.Keypair | undefined> => {
  const filePath = path.join(".keys/", `${fileName}.json`);

  try {
    const raw = await fs.readFile(filePath);
    const formattedData = JSON.parse(raw.toString());

    const keypair = anchor.web3.Keypair.fromSecretKey(
      Uint8Array.from(formattedData)
    );
    console.log(keypair.publicKey.toString());
    return keypair;
  } catch (error) {
    console.error(
      `Error reading keypair from file: ${(error as Error).message}`
    );
  }
};
