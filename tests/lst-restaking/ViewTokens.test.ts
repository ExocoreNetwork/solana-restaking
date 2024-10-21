import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { IDL as LstRestakingIdl } from "../../target/types/lst_restaking";
import {
    getConfig, getToken,
    LST_RESTAKING_PROGRAM_ID,
} from "../utils";

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

    const lst_program = new Program(
        LstRestakingIdl,
        LST_RESTAKING_PROGRAM_ID,
        provider
    );

    it("View Tokens!", async () => {

        const mint = new web3.PublicKey(process.env.MINT_ADDRESS);
        console.log(`mint: ${mint}`);

        const [tokens_address] = await getToken(mint)
        const [config_address] = await getConfig()

        const configData = await lst_program.account.config.fetch(config_address)

        console.log(`config.tokens: ${configData.tokens.toString()}`)


        const tokensData = await lst_program.account.token.fetch(tokens_address)
        console.log(`tokens: ${tokensData.toString()}`)
    });

});