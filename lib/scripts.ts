import { BN, Program } from "@coral-xyz/anchor";
import {
    Connection,
    PublicKey,
} from "@solana/web3.js";

import { TapestryExplorerStatkingContract } from "../target/types/tapestry_explorer_statking_contract";
import {
    SEED_CONFIG,
    TOKEN_ADDRESS,
} from "./constant";

import { getAssociatedTokenAccount } from "./util";

export const createConfigTx = async (
    admin: PublicKey,
    newConfig: any,
    connection: Connection,
    program: Program<TapestryExplorerStatkingContract>
) => {
    const [configPda, _] = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    );

    console.log("configPda: ", configPda.toBase58());
    const tx = await program.methods
        .configure(newConfig)
        .accounts({
            payer: admin,
        })
        .transaction();

    console.log("configPda after: ", configPda.toBase58());

    tx.feePayer = admin;
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    return tx;
}

export const createDepositTx = async (
    admin: PublicKey,
    invester: PublicKey,
    token: PublicKey,

    connection: Connection,
    program: Program<TapestryExplorerStatkingContract>
) => {
    const [configPda, _] = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    );
    console.log("configPda: ", configPda.toBase58());

    let globalTokenAccount = await getAssociatedTokenAccount(configPda, TOKEN_ADDRESS);
    console.log("🚀 ~ globalTokenAccount:", globalTokenAccount)

    let investerTokenAccount = await getAssociatedTokenAccount(invester, TOKEN_ADDRESS);
    console.log("🚀 ~ investerTokenAccount:", investerTokenAccount)

    const tx = await program.methods.depositFund().accounts({ admin: admin, tokenMint: token, invester: invester, investerTokenAccount }).transaction();

    console.log("configPda after: ", configPda.toBase58());

    tx.feePayer = admin;
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

    return tx;
}

export const createStakeTx = async (
    user: PublicKey,
    amount: number,

    connection: Connection,
    program: Program<TapestryExplorerStatkingContract>
) => {
    const [configPda, _] = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    );
    console.log("configPda: ", configPda.toBase58());

    let globalTokenAccount = await getAssociatedTokenAccount(configPda, TOKEN_ADDRESS);
    console.log("🚀 ~ globalTokenAccount:", globalTokenAccount)

    let userTokenAccount = await getAssociatedTokenAccount(user, TOKEN_ADDRESS);
    console.log("🚀 ~ userTokenAccount:", userTokenAccount)

    const tx = await program.methods.stake(new BN(amount)).accounts({ user: user, userTokenAccount: userTokenAccount }).transaction();

    console.log("configPda after: ", configPda.toBase58());

    tx.feePayer = user;
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

    return tx;

}

export const createUnStakeTx = async (
    user: PublicKey,

    connection: Connection,
    program: Program<TapestryExplorerStatkingContract>
) => {
    const [configPda, _] = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    );
    console.log("configPda: ", configPda.toBase58());

    let globalTokenAccount = await getAssociatedTokenAccount(configPda, TOKEN_ADDRESS);
    console.log("🚀 ~ globalTokenAccount:", globalTokenAccount)

    let userTokenAccount = await getAssociatedTokenAccount(user, TOKEN_ADDRESS);
    console.log("🚀 ~ userTokenAccount:", userTokenAccount)

    const tx = await program.methods.unstake().accounts({ globalTokenAccount, user, userTokenAccount }).transaction();

    console.log("configPda after: ", configPda.toBase58());

    tx.feePayer = user;
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

    return tx;
}

export const createClaimTx = async (
    user: PublicKey,

    connection: Connection,
    program: Program<TapestryExplorerStatkingContract>
) => {
    const [configPda, _] = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    );
    console.log("configPda: ", configPda.toBase58());

    let globalTokenAccount = await getAssociatedTokenAccount(configPda, TOKEN_ADDRESS);
    console.log("🚀 ~ globalTokenAccount:", globalTokenAccount)

    let userTokenAccount = await getAssociatedTokenAccount(user, TOKEN_ADDRESS);
    console.log("🚀 ~ userTokenAccount:", userTokenAccount)

    const tx = await program.methods.claimReward().accounts({ globalTokenAccount, user, userTokenAccount }).transaction();

    console.log("configPda after: ", configPda.toBase58());

    tx.feePayer = user;
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

    return tx;
}

export const createPauseTx = async (
    admin: PublicKey,
    is_stop: number,

    connection: Connection,
    program: Program<TapestryExplorerStatkingContract>
) => {
    console.log("🚀 ~ is_stop:", is_stop)

    const tx = await program.methods.pause(is_stop).accounts({ admin }).transaction();

    tx.feePayer = admin;
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

    return tx;
}