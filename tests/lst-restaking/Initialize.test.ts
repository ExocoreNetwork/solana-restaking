import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { LstRestaking } from "../../target/types/lst_restaking";
import { IDL } from "../../target/types/endpoint";
import {
  airdrop,
  eid,
  ENDPOINT_EVENT_AUTHORITY,
  ENDPOINT_PROGRAM_ID,
  getConfig,
  getDefaultSendLibraryConfig,
  getLZReceiveTypes,
  getMessageList,
  getNonce,
  getOApp,
  getOAppRegistry,
  getPendingNonce,
  getReceiveLibraryConfig,
  getSendLibraryConfig,
  getTokenWhiteList,
  LST_RESTAKING_PROGRAM_ID,
  remoteEid,
  remoteOapp,
  SYSTEM_PROGRAM_ID,
  testKeys,
} from "../utils";
import { assert } from "chai";
import { Connection } from "@solana/web3.js";

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

  const program = anchor.workspace.LstRestaking as Program<LstRestaking>;
  // const endpoint_program = anchor.workspace.Endpoint as Program<Endpoint>;

  const endpoint_program = new Program(IDL, ENDPOINT_PROGRAM_ID, provider);

  it("Is initialized!", async () => {
    const [owner, , , , , delegate] = await testKeys();

    const [config] = await getConfig();

    const [OApp] = await getOApp();

    const [tokenWhiteList] = await getTokenWhiteList();

    const [oappRegistry] = await getOAppRegistry(OApp);

    const [nonce] = await getNonce(OApp, remoteEid, remoteOapp);

    const [pendingInboundNonce] = await getPendingNonce(
      OApp,
      remoteEid,
      remoteOapp
    );

    const [sendLibraryConfig] = await getSendLibraryConfig(OApp, remoteEid);

    const [defaultSendLibraryConfig] = await getDefaultSendLibraryConfig(
      remoteEid
    );

    const [receiveLibraryConfig] = await getReceiveLibraryConfig(OApp, eid);

    const [messageList] = await getMessageList(config);

    const [lzReceiveTypes] = await getLZReceiveTypes(OApp);

    console.log(`delegate pubkey: ${delegate.publicKey}`);

    const conn = anchor.getProvider().connection as unknown as Connection;

    // await airdrop(conn, owner.publicKey);
    // await airdrop(conn, delegate.publicKey);

    const init_tx = await program.methods
      .initialize({
        remoteEid: remoteEid,
        receiver: remoteOapp,
      })
      .accounts({
        owner: owner.publicKey,
        config,
        messageList,
        lzReceiveTypes,
        tokenWhiteList,
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

    let tx = new web3.Transaction();
    // init nonce
    const instr1 = await endpoint_program.methods
      .initNonce({
        localOapp: OApp,
        remoteEid,
        remoteOapp,
      })
      .accounts({
        delegate: delegate.publicKey,
        oappRegistry,
        nonce,
        pendingInboundNonce,
      })
      .instruction();

    tx.add(instr1);
    // init_send_library
    const instr2 = await endpoint_program.methods
      .initSendLibrary({
        sender: OApp,
        eid: remoteEid,
      })
      .accounts({
        delegate: delegate.publicKey,
        oappRegistry,
        sendLibraryConfig,
      })
      .instruction();

    tx.add(instr2);
    // // init_receive_library
    const instr3 = await endpoint_program.methods
      .initReceiveLibrary({
        receiver: OApp,
        eid,
      })
      .accounts({
        delegate: delegate.publicKey,
        oappRegistry,
        receiveLibraryConfig,
      })
      .instruction();

    tx.add(instr3);

    const transactionSignature = await provider.connection.sendTransaction(tx, [
      delegate,
    ]);

    console.log(`Your transaction signature: ${transactionSignature}`);

    let defaultSendLibraryConfigState =
      await endpoint_program.account.sendLibraryConfig.fetch(
        defaultSendLibraryConfig
      );

    console.log(
      `defaultSendLibraryConfig: ${defaultSendLibraryConfigState.messageLib}`
    );

    const configState = await program.account.config.fetch(config);

    assert.equal(
      configState.owner.toString(),
      owner.publicKey.toString(),
      "Initialize failed"
    );

    const lzReceiveTypesState = await program.account.lzReceiveTypes.fetch(
      lzReceiveTypes
    );

    console.log(lzReceiveTypesState);
  });
});
