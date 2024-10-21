import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { IDL as LstRestakingIdl } from "../../target/types/lst_restaking";
import {
  airdrop,
  ENDPOINT_EVENT_AUTHORITY,
  ENDPOINT_PROGRAM_ID,
  getConfig,
  getLZReceiveTypes,
  getOApp,
  getOAppRegistry,
  LST_RESTAKING_PROGRAM_ID,
  remoteEid,
  remoteOapp,
  SYSTEM_PROGRAM_ID,
  testKeys,
} from "../utils";
import { PublicKey,} from "@solana/web3.js";
import {
   EndpointProgram,
  UlnProgram
} from "@layerzerolabs/lz-solana-sdk-v2";

describe("solana-restaking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();

  // Set the provider with customized options
  const customProvider = new anchor.AnchorProvider(
      provider.connection,
      provider.wallet,
      {
        preflightCommitment: "processed", // or "confirmed" / "finalized"
        commitment: "confirmed", // or "finalized"
      }
  );

  anchor.setProvider(customProvider);

  // const lst_program = anchor.workspace.LstRestaking as Program<LstRestaking>;
  // const endpoint_program = anchor.workspace.Endpoint as Program<Endpoint>;

  const lst_program = new Program(
      LstRestakingIdl,
      LST_RESTAKING_PROGRAM_ID,
      provider
  );

  it("Is initialized!", async () => {
    const [owner, new_owner, user, invalid_owner, another_user, delegate, dev] =
        await testKeys();

    const [config] = await getConfig();

    const [OApp] = await getOApp();  // ok

    const [oappRegistry] = await getOAppRegistry(OApp);  // ok

    const [lzReceiveTypes] = await getLZReceiveTypes(OApp);

    console.log(`delegate pubkey: ${delegate.publicKey}`);

    // await airdrop(conn, owner.publicKey);
    // await airdrop(conn, dev.publicKey);
    // await airdrop(conn, delegate.publicKey);

    const init_tx = await lst_program.methods
        .initConfig({
          dstEid: remoteEid,
          receiver: remoteOapp,
        })
        .accounts({
          owner: owner.publicKey,
          config,
          lzReceiveTypes,
          operator: delegate.publicKey,
          delegate: delegate.publicKey,
          endpointProgram: ENDPOINT_PROGRAM_ID,
        })
        .remainingAccounts([
          {
            isSigner: false,
            isWritable: false,
            pubkey: ENDPOINT_PROGRAM_ID,
          },
          {
            isSigner: true,
            isWritable: true,
            pubkey: owner.publicKey,
          },
          {
            isSigner: false,
            isWritable: false,
            pubkey: config,
          },
          {
            isSigner: false,
            isWritable: true,
            pubkey: oappRegistry,
          },
          {
            isSigner: false,
            isWritable: false,
            pubkey: SYSTEM_PROGRAM_ID,
          },
          {
            isSigner: false,
            isWritable: false,
            pubkey: ENDPOINT_EVENT_AUTHORITY,
          },
          {
            isSigner: false,
            isWritable: false,
            pubkey: ENDPOINT_PROGRAM_ID,
          },
        ])
        .signers([owner])
        .rpc()
        .catch((e) => {
          console.log(e);
        });

    console.log("Your transaction signature", init_tx);

  });
});
