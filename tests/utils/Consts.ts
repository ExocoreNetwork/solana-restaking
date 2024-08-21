import { PublicKey } from "@solana/web3.js";
import { readKeypairToFile } from "./Helper";
import {createMint} from "./Utils";
import {Program, web3} from "@coral-xyz/anchor";
import { LstRestaking } from "../../target/types/lst_restaking";


export const LST_RESTAKING_PROGRAM_ID = new PublicKey(
  "68av2QdR1k1QeaxsJwjiB16QHXDhTuaS14tyTNM3MgHX"
);

export const testKeys = async () => {
  const OWNER = await readKeypairToFile("owner");
  const NEW_OWNER = await readKeypairToFile("new_owner");
  const USER = await readKeypairToFile("user");
  const INVALID_OWNER = await readKeypairToFile("user2");
  const ANOTHER_USER = await readKeypairToFile("user2");


  return [OWNER, NEW_OWNER, USER, INVALID_OWNER, ANOTHER_USER]
};

