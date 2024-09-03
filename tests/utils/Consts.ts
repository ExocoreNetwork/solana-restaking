import { PublicKey } from "@solana/web3.js";
import {evmAddressToPaddedArray, readKeypairToFile} from "./Helper";
import {createMint} from "./Utils";
import {Program, web3} from "@coral-xyz/anchor";
import { LstRestaking } from "../../target/types/lst_restaking";


export const LST_RESTAKING_PROGRAM_ID = new PublicKey(
  "68av2QdR1k1QeaxsJwjiB16QHXDhTuaS14tyTNM3MgHX"
);

export const ENDPOINT_PROGRAM_ID = new PublicKey("76y77prsiCMvXMjuoZ5VRrhG5qYBrUMYTE5WgHqgjEn6");

export const SYSTEM_PROGRAM_ID = new PublicKey("11111111111111111111111111111111");

export const remoteOapp = evmAddressToPaddedArray("0xC025454388d52CBCc92B8D1F573c677bA3F8f1b8");
export const remoteEid = 40259;

export const eid = 40168;


export const testKeys = async () => {
  const OWNER = await readKeypairToFile("owner");
  const NEW_OWNER = await readKeypairToFile("new_owner");
  const USER = await readKeypairToFile("user");
  const INVALID_OWNER = await readKeypairToFile("user2");
  const ANOTHER_USER = await readKeypairToFile("user2");
  const DELEGATE = await readKeypairToFile("delegate");


  return [OWNER, NEW_OWNER, USER, INVALID_OWNER, ANOTHER_USER, DELEGATE]
};

