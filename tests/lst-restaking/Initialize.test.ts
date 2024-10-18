import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { LstRestaking } from "../../target/types/lst_restaking";
// import { IDL as EndpointIdl } from "../../target/types/endpoint";
import { IDL as LstRestakingIdl } from "../../target/types/lst_restaking";
import {
  airdrop,
  ENDPOINT_EVENT_AUTHORITY,
  ENDPOINT_PROGRAM_ID,
  getConfig, getDefaultReceiveConfig, getDefaultSendConfig,
  getDefaultSendLibraryConfig,
  getLZReceiveTypes, getMessageLib, getMessageLibInfo,
  getMessages,
  getNonce,
  getOApp,
  getOAppRegistry,
  getPendingNonce,
  getReceiveConfig,
  getReceiveLibraryConfig,
  getSendConfig,
  getSendLibraryConfig,
  getTokens,
  LST_RESTAKING_PROGRAM_ID,
  remoteEid,
  remoteOapp, remotePeer,
  SEND_LIBRARY_INFO,
  SYSTEM_PROGRAM_ID,
  testKeys,
  ULN302_PROGRAM_ID,
  ULN_SETTINGS,
} from "../utils";
import { assert } from "chai";
import {Connection, Keypair, PublicKey, Signer, TransactionInstruction} from "@solana/web3.js";
import BN from "bn.js";
import {EndpointId} from "@layerzerolabs/lz-definitions";
import {
  buildVersionedTransaction, EndpointProgram,
  ExecutorPDADeriver,
  SetConfigType,
  UlnProgram
} from "@layerzerolabs/lz-solana-sdk-v2";
import {arrayify, hexZeroPad} from "@ethersproject/bytes";

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

  // const endpoint_program = new Program(
  //   EndpointIdl,
  //   ENDPOINT_PROGRAM_ID,
  //   provider
  // );

  const endpointProgram = new EndpointProgram.Endpoint(new PublicKey('76y77prsiCMvXMjuoZ5VRrhG5qYBrUMYTE5WgHqgjEn6')) // endpoint program id, mainnet and testnet are the same
  const ulnProgram = new UlnProgram.Uln(new PublicKey('7a4WjyR8VZ7yZz5XJAKm39BUGn5iT9CKcv2pmG9tdXVH')) // uln program id, mainnet and testnet are the same
  const executorProgram = new PublicKey('6doghB248px58JSSwG4qejQ46kFMW4AMj7vzJnWZHNZn') // executor program id, mainnet and testnet are the same

  it("Is initialized!", async () => {
    const [owner, new_owner, user, invalid_owner, another_user, delegate, dev] =
      await testKeys();

    const [config] = await getConfig();

    const [OApp] = await getOApp();  // ok

    const [tokens] = await getTokens();

    const [oappRegistry] = await getOAppRegistry(OApp);  // ok

    const [nonce] = await getNonce(OApp);  // ok

    const [pendingInboundNonce] = await getPendingNonce(OApp); // ok

    const [sendLibraryConfig] = await getSendLibraryConfig(OApp); // ok

    const [defaultSendLibraryConfig] = await getDefaultSendLibraryConfig(); // ok

    const [sendConfig] = await getSendConfig(OApp);

    const [receiveConfig] = await getReceiveConfig(OApp);

    const [defaultSendConfig] = await getDefaultSendConfig();

    const [defaultReceiveConfig] = await getDefaultReceiveConfig();

    const [receiveLibraryConfig] = await getReceiveLibraryConfig(OApp);

    const [messages] = await getMessages(config);

    const [lzReceiveTypes] = await getLZReceiveTypes(OApp);

    const [messageLib] = await getMessageLib();

    const [messageLibInfo] = await getMessageLibInfo();

    console.log(`delegate pubkey: ${delegate.publicKey}`);

    const conn = anchor.getProvider().connection as unknown as Connection;


    // await airdrop(conn, owner.publicKey);
    // await airdrop(conn, dev.publicKey);
    // await airdrop(conn, delegate.publicKey);

    // const init_tx = await lst_program.methods
    //   .initConfig({
    //     dstEid: remoteEid,
    //     receiver: remoteOapp,
    //   })
    //   .accounts({
    //     owner: owner.publicKey,
    //     config,
    //     messages,
    //     lzReceiveTypes,
    //     tokens,
    //     operator: delegate.publicKey,
    //     delegate: delegate.publicKey,
    //     endpointProgram: ENDPOINT_PROGRAM_ID,
    //   })
    //   .remainingAccounts([
    //     {
    //       isSigner: false,
    //       isWritable: false,
    //       pubkey: ENDPOINT_PROGRAM_ID,
    //     },
    //     {
    //       isSigner: true,
    //       isWritable: true,
    //       pubkey: owner.publicKey,
    //     },
    //     {
    //       isSigner: false,
    //       isWritable: false,
    //       pubkey: config,
    //     },
    //     {
    //       isSigner: false,
    //       isWritable: true,
    //       pubkey: oappRegistry,
    //     },
    //     {
    //       isSigner: false,
    //       isWritable: false,
    //       pubkey: SYSTEM_PROGRAM_ID,
    //     },
    //     {
    //       isSigner: false,
    //       isWritable: false,
    //       pubkey: ENDPOINT_EVENT_AUTHORITY,
    //     },
    //     {
    //       isSigner: false,
    //       isWritable: false,
    //       pubkey: ENDPOINT_PROGRAM_ID,
    //     },
    //   ])
    //   .signers([owner])
    //   .rpc()
    //   .catch((e) => {
    //     console.log(e);
    //   });
    //
    // console.log("Your transaction signature", init_tx);

    // let tx = new web3.Transaction();
    // // init nonce ok
    // const instr1 = await endpoint_program.methods
    //   .initNonce({
    //     localOapp: OApp,
    //     remoteEid,
    //     remoteOapp,
    //   })
    //   .accounts({
    //     delegate: delegate.publicKey,
    //     oappRegistry,
    //     nonce,
    //     pendingInboundNonce,
    //   })
    //   .instruction();
    //
    // tx.add(instr1);
    // // init_send_library ok
    // const instr2 = await endpoint_program.methods
    //   .initSendLibrary({
    //     sender: OApp,
    //     eid: remoteEid,
    //   })
    //   .accounts({
    //     delegate: delegate.publicKey,
    //     oappRegistry,
    //     sendLibraryConfig,
    //   })
    //   .instruction();
    //
    // tx.add(instr2);
    // // // init_receive_library ok
    // const instr3 = await endpoint_program.methods
    //   .initReceiveLibrary({
    //     receiver: OApp,
    //     eid: remoteEid,
    //   })
    //   .accounts({
    //     delegate: delegate.publicKey,
    //     oappRegistry,
    //     receiveLibraryConfig,
    //   })
    //   .instruction();
    //
    // tx.add(instr3);
    //
    //
    //
    // const instr4 = await endpoint_program.methods.setSendLibrary(
    //     {
    //       sender: OApp,
    //       eid: remoteEid,
    //       newLib: messageLib
    //     }
    // ).accounts({
    //   signer: delegate.publicKey,
    //   oappRegistry,
    //   sendLibraryConfig,
    //   messageLibInfo
    // }).instruction();
    //
    // tx.add(instr4);
    //
    // const instr5 = await endpoint_program.methods.setReceiveLibrary({
    //   receiver: OApp,
    //   eid: remoteEid,
    //   newLib: messageLib,
    //   gracePeriod: new BN(0)
    // }).accounts({
    //   signer: delegate.publicKey,
    //   oappRegistry,
    //   receiveLibraryConfig,
    //   messageLibInfo
    // }).instruction();
    //
    // tx.add(instr5);
    //
    // // init config
    // const instr6 = await endpoint_program.methods
    //     .initConfig({
    //       oapp: OApp,
    //       eid: remoteEid,
    //     })
    //     .accounts({
    //       delegate: delegate.publicKey,
    //       oappRegistry,
    //       messageLibInfo,
    //       messageLib,
    //       messageLibProgram: ULN302_PROGRAM_ID,
    //     })
    //     .remainingAccounts([
    //       {
    //         isSigner: true,
    //         isWritable: true,
    //         pubkey: delegate.publicKey,
    //       },
    //       {
    //         isSigner: false,
    //         isWritable: false,
    //         pubkey: ULN_SETTINGS,
    //       },
    //       {
    //         isSigner: false,
    //         isWritable: true,
    //         pubkey: sendConfig,
    //       },
    //       {
    //         isSigner: false,
    //         isWritable: true,
    //         pubkey: receiveConfig,
    //       },
    //       {
    //         isSigner: false,
    //         isWritable: false,
    //         pubkey: SYSTEM_PROGRAM_ID,
    //       },
    //     ])
    //     .instruction();
    //
    // tx.add(instr6);

    // const instr7 = await endpoint_program.methods.setConfig({
    //   eid: remoteEid,
    //   configType: 1,
    //   config: ,
    //   oapp: OApp
    // })
    //     .accounts({
    //       signer: delegate.publicKey,
    //       messageLibInfo,
    //       messageLib,
    //       messageLibProgram: ULN302_PROGRAM_ID,
    //       oappRegistry,
    //     })
    //     .remainingAccounts(
    //       [
    //         {
    //           isSigner: false,
    //           isWritable: false,
    //           pubkey: ULN_SETTINGS
    //         },
    //         {
    //           isSigner: false,
    //           isWritable: false,
    //           pubkey: sendConfig
    //         },
    //         {
    //           isSigner: false,
    //           isWritable: false,
    //           pubkey: receiveConfig
    //         },
    //         {
    //           isSigner: false,
    //           isWritable: false,
    //           pubkey: defaultSendConfig
    //         },
    //         {
    //           isSigner: false,
    //           isWritable: false,
    //           pubkey: defaultReceiveConfig
    //         }
    //       ]
    //     )
    //     .instruction();
    // tx.add(instr7);

    // const transactionSignature = await provider.connection.sendTransaction(tx, [
    //   delegate,
    // ]);

    // console.log(`Your transaction signature: ${transactionSignature}`);

    const remotePeerBytes = arrayify(hexZeroPad(remotePeer, 32))

    await initSendLibrary(conn, delegate as unknown as Keypair, EndpointId.EXOCORE_V2_TESTNET)
    await initReceiveLibrary(conn, delegate as unknown as Keypair, EndpointId.EXOCORE_V2_TESTNET)
    await initOappNonce(conn, delegate as unknown as Keypair, EndpointId.EXOCORE_V2_TESTNET, remotePeerBytes)
    await setSendLibrary(conn, delegate as unknown as Keypair, EndpointId.EXOCORE_V2_TESTNET)
    await setReceiveLibrary(conn, delegate as unknown as Keypair, EndpointId.EXOCORE_V2_TESTNET)
    await initUlnConfig(conn, delegate as unknown as Keypair, delegate as unknown as Keypair, EndpointId.EXOCORE_V2_TESTNET)
    await setOappExecutor(conn, delegate as unknown as Keypair, EndpointId.EXOCORE_V2_TESTNET);

    // let defaultSendLibraryConfigState =
    //   await endpoint_program.account.sendLibraryConfig.fetch(
    //     defaultSendLibraryConfig
    //   );
    //
    // console.log(
    //   `defaultSendLibraryConfig: ${defaultSendLibraryConfigState.messageLib}`
    // );
    //
    // const configState = await lst_program.account.config.fetch(config);
    //
    // assert.equal(
    //   configState.owner.toString(),
    //   owner.publicKey.toString(),
    //   "Initialize failed"
    // );
    //
    // const lzReceiveTypesState =
    //   await lst_program.account.lzReceiveTypesAccount.fetch(lzReceiveTypes);
    //
    // console.log(
    //   `lzReceiveTypesState config: ${lzReceiveTypesState.config}, messages: ${lzReceiveTypesState.messages}`
    // );
  });

  async function initSendLibrary(connection: Connection, admin: Keypair, remote: EndpointId): Promise<void> {
    const [id] = await getOApp();
    const ix = await endpointProgram.initSendLibrary(connection, admin.publicKey, id, remote)
    if (ix == null) {
      return Promise.resolve()
    }
    await sendAndConfirm(connection, [admin], [ix])
  }

  async function initReceiveLibrary(connection: Connection, admin: Keypair, remote: EndpointId): Promise<void> {
    const [id] = await getOApp();
    const ix = await endpointProgram.initReceiveLibrary(connection, admin.publicKey, id, remote)
    if (ix == null) {
      return Promise.resolve()
    }
    await sendAndConfirm(connection, [admin], [ix])
  }

  async function initOappNonce(
      connection: Connection,
      admin: Keypair,
      remote: EndpointId,
      remotePeer: Uint8Array
  ): Promise<void> {
    const [id] = await getOApp();
    const ix = await endpointProgram.initOAppNonce(connection, admin.publicKey, remote, id, remotePeer)
    if (ix === null) return Promise.resolve()
    const current = false
    try {
      const nonce = await endpointProgram.getNonce(connection, id, remote, remotePeer)
      if (nonce) {
        console.log('nonce already set')
        return Promise.resolve()
      }
    } catch (e) {
      /*nonce not init*/
    }
    await sendAndConfirm(connection, [admin], [ix])
  }

  async function setSendLibrary(connection: Connection, admin: Keypair, remote: EndpointId): Promise<void> {
    const [id] = await getOApp();
    const sendLib = await endpointProgram.getSendLibrary(connection, id, remote)
    const current = sendLib ? sendLib.msgLib.toBase58() : ''
    const [expectedSendLib] = ulnProgram.deriver.messageLib()
    const expected = expectedSendLib.toBase58()
    if (current === expected) {
      console.log(`==`)
      return Promise.resolve()
    }
    const ix = await endpointProgram.setSendLibrary(admin.publicKey, id, ulnProgram.program, remote)
    await sendAndConfirm(connection, [admin], [ix])
  }

  async function setReceiveLibrary(connection: Connection, admin: Keypair, remote: EndpointId): Promise<void> {
    const [id] = await getOApp();
    const receiveLib = await endpointProgram.getReceiveLibrary(connection, id, remote)
    const current = receiveLib ? receiveLib.msgLib.toBase58() : ''
    const [expectedMessageLib] = ulnProgram.deriver.messageLib()
    const expected = expectedMessageLib.toBase58()
    if (current === expected) {
      console.log(`==`)
      return Promise.resolve()
    }
    const ix = await endpointProgram.setReceiveLibrary(admin.publicKey, id, ulnProgram.program, remote)
    await sendAndConfirm(connection, [admin], [ix])
  }

  async function initUlnConfig(
      connection: Connection,
      payer: Keypair,
      admin: Keypair,
      remote: EndpointId
  ): Promise<void> {
    const [id] = await getOApp();

    const current = await ulnProgram.getSendConfigState(connection, id, remote)
    if (current) {
      return Promise.resolve()
    }
    const ix = await endpointProgram.initOappConfig(admin.publicKey, ulnProgram, payer.publicKey, id, remote)
    await sendAndConfirm(connection, [admin], [ix])
  }
  async function setOappExecutor(connection: Connection, admin: Keypair, remote: EndpointId): Promise<void> {


    const [id] = await getOApp();
    const defaultOutboundMaxMessageSize = 10000

    const [executorPda] = new ExecutorPDADeriver(executorProgram).config()
    const expected: UlnProgram.types.ExecutorConfig = {
      maxMessageSize: defaultOutboundMaxMessageSize,
      executor: executorPda,
    }

    const current = (await ulnProgram.getSendConfigState(connection, id, remote))?.executor
    const ix = await endpointProgram.setOappConfig(connection, admin.publicKey, id, ulnProgram.program, remote, {
      configType: SetConfigType.EXECUTOR,
      value: expected,
    })
    if (
        current &&
        current.executor.toBase58() === expected.executor.toBase58() &&
        current.maxMessageSize === expected.maxMessageSize
    ) {
      return Promise.resolve()
    }
    await sendAndConfirm(connection, [admin], [ix])
  }

  async function sendAndConfirm(
      connection: Connection,
      signers: Signer[],
      instructions: TransactionInstruction[]
  ): Promise<void> {
    const tx = await buildVersionedTransaction(connection, signers[0].publicKey, instructions, 'confirmed')
    tx.sign(signers)
    const hash = await connection.sendTransaction(tx, { skipPreflight: true })
    await connection.confirmTransaction(hash, 'confirmed')
  }
});
