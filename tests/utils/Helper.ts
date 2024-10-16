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

export function evmAddressToPaddedArray(evmAddress: string): number[] {
  // Step 1: Remove the '0x' prefix if it exists
  const strippedAddress = evmAddress.startsWith("0x") ? evmAddress.slice(2) : evmAddress;

  // Step 2: Convert the hex string to a byte array
  const addressBytes = Buffer.from(strippedAddress, 'hex');

  // Step 3: Create a 32-byte array filled with zeros
  const result = new Array(32).fill(0);

  // Step 4: Copy the address bytes into the last 20 bytes of the result array
  for (let i = 0; i < addressBytes.length; i++) {
    result[12 + i] = addressBytes[i];
  }

  return result;
}


export function hexToUint8Array(hex: string): Uint8Array {
  if (hex.startsWith("0x")) {
    hex = hex.slice(2);
  }

  if (hex.length % 2 !== 0) {
    throw new Error("Hex string length must be even.");
  }

  const byteArray = new Uint8Array(hex.length / 2);
  for (let i = 0; i < hex.length; i += 2) {
    byteArray[i / 2] = parseInt(hex.substr(i, 2), 16);
  }

  return byteArray;
}
