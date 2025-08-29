import * as anchor from "@coral-xyz/anchor";
import { BN, Program, web3 } from "@coral-xyz/anchor";
import fs from "fs";

import { Keypair, Connection, PublicKey, Transaction } from "@solana/web3.js";

import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

import { TapestryExplorerStatkingContract } from "../target/types/tapestry_explorer_statking_contract";
import {
    createConfigTx,
    createDepositTx,
    createStakeTx,
    createUnStakeTx,
    createClaimTx,
    createPauseTx,
} from "../lib/scripts";

import {
    TOKEN_ADDRESS
} from "../lib/constant"

import { execTx } from "../lib/util";
import {
    SEED_CONFIG,
    SEED_USERINFO,
} from "../lib/constant";

let solConnection: Connection = null;
let program: Program<TapestryExplorerStatkingContract> = null;
let payer: NodeWallet = null;

/**
 * Set cluster, provider, program
 * If rpc != null use rpc, otherwise use cluster param
 * @param cluster - cluster ex. mainnet-beta, devnet ...
 * @param keypair - wallet keypair
 * @param rpc - rpc
 */
export const setClusterConfig = async (
    cluster: web3.Cluster,
    keypair: string,
    rpc?: string,
) => {
    if (!rpc) {
        solConnection = new web3.Connection(web3.clusterApiUrl(cluster));
    } else {
        solConnection = new web3.Connection(rpc);
    }

    const walletKeypair = Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(fs.readFileSync(keypair, "utf-8"))),
        { skipValidation: true }
    );
    payer = new NodeWallet(walletKeypair);

    console.log("Wallet Address: ", payer.publicKey.toBase58());

    anchor.setProvider(
        new anchor.AnchorProvider(solConnection, payer, {
            skipPreflight: true,
            commitment: "confirmed",
        })
    );
    // Generate the program client from IDL.
    program = anchor.workspace.tapestry_explorer_statking_contract as Program<TapestryExplorerStatkingContract>;

    console.log("ProgramId: ", program.programId.toBase58());
};


export const configProject = async (
) => {
    // Create a dummy config object to pass as argument.
    const newConfig = {
        authority: payer.publicKey,
        pendingAuthority: payer.publicKey,//payer.publicKey,//PublicKey.default,

        tokenMintConfig: TOKEN_ADDRESS,
        claimPeriod: new BN(20),
        totalRate: 0,
        totalStakers: new BN(0),
        lastReward_time: new BN(0),
        rewardMultiplier: new BN(0),
        depositTime: new BN(0),
        totalDeposit: new BN(0),
        purchaseAmt: new BN(1_209_600_000_000),
        is_stop: 0,

        initialized: true,
    };
    console.log("🚀 ~ newConfig:", newConfig)
    console.log("🚀 ~ configProject ~ newConfig.purchase_amt:", newConfig.purchaseAmt)
    console.log("🚀 ~ configProject ~ newConfig.TOKEN_ADDRESS:", newConfig.tokenMintConfig)

    const tx = await createConfigTx(
        payer.publicKey,
        newConfig,
        solConnection,
        program
    );

    await execTx(tx, solConnection, payer);

    const [configPda, _] = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    );
    const configAccount = await program.account.config.fetch(configPda);
    console.log("🚀 ~ configProject ~ configAccount:", configAccount)
}



export const Deposit = async () => {
    const [configPda, _] = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    );
    const configAccount = await program.account.config.fetch(configPda);
    console.log("🚀 ~ Deposit ~ configAccount:", configAccount)

    const tx = await createDepositTx(
        payer.publicKey,
        payer.publicKey,
        TOKEN_ADDRESS,
        solConnection,
        program
    );

    await execTx(tx, solConnection, payer);

    const newConfigAccount = await program.account.config.fetch(configPda);
    console.log("🚀 ~ Deposit ~ newConfigAccount:", newConfigAccount)
}

export const Stake = async (amount: number) => {
    const configPda = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    )[0];
    const configAccount = await program.account.config.fetch(configPda);
    console.log("🚀 ~ Stake ~ configAccount:", configAccount)

    const tx = await createStakeTx(
        payer.publicKey,
        amount,

        solConnection,
        program
    );

    await execTx(tx, solConnection, payer);

    const newConfigAccount = await program.account.config.fetch(configPda);
    console.log("🚀 ~ Stake ~ newConfigAccount:", newConfigAccount)

    const [userInfoPda, _] = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_USERINFO), payer.publicKey.toBytes()],
        program.programId
    );

    console.log("🚀 ~ Stake ~ userInfoPda:", userInfoPda.toBase58())
    const userInfoAccount = await program.account.user.fetch(userInfoPda);
    console.log("🚀 ~ Stake ~ userInfoAccount:", userInfoAccount)
}


export const UnStake = async () => {
    const configPda = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    )[0];
    const configAccount = await program.account.config.fetch(configPda);
    console.log("🚀 ~ UnStake ~ configAccount:", configAccount)

    const tx = await createUnStakeTx(
        payer.publicKey,

        solConnection,
        program
    );
    await execTx(tx, solConnection, payer);

    const newConfigAccount = await program.account.config.fetch(configPda);
    console.log("🚀 ~ UnStake ~ newConfigAccount:", newConfigAccount)

    const [userInfoPda, _] = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_USERINFO), payer.publicKey.toBytes()],
        program.programId
    );

    console.log("🚀 ~ Stake ~ userInfoPda:", userInfoPda.toBase58())
    const userInfoAccount = await program.account.user.fetch(userInfoPda);
    console.log("🚀 ~ Stake ~ userInfoAccount:", userInfoAccount)
}

export const Claim = async () => {
    const configPda = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    )[0];
    const configAccount = await program.account.config.fetch(configPda);
    console.log("🚀 ~ Claim ~ configAccount:", configAccount)

    const tx = await createClaimTx(
        payer.publicKey,

        solConnection,
        program
    );
    await execTx(tx, solConnection, payer);

    const newConfigAccount = await program.account.config.fetch(configPda);
    console.log("🚀 ~ Claim ~ newConfigAccount:", newConfigAccount)

    const [userInfoPda, _] = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_USERINFO), payer.publicKey.toBytes()],
        program.programId
    );

    console.log("🚀 ~ Stake ~ userInfoPda:", userInfoPda.toBase58())
    const userInfoAccount = await program.account.user.fetch(userInfoPda);
    console.log("🚀 ~ Stake ~ userInfoAccount:", userInfoAccount)
}

export const Pause = async (is_stop: number) => {
    console.log("🚀 ~ Pause ~ is_stop:", is_stop)
    const configPda = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    )[0];
    const configAccount = await program.account.config.fetch(configPda);
    console.log("🚀 ~ Pause ~ configAccount:", configAccount)

    const tx = await createPauseTx(payer.publicKey, is_stop, solConnection, program);
    await execTx(tx, solConnection, payer);

    const newConfigAccount = await program.account.config.fetch(configPda);
    console.log("🚀 ~ Pause ~ newConfigAccount:", newConfigAccount)
}

export const GetUserInfo = async () => {
    const configPda = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_CONFIG)],
        program.programId
    )[0];
    const configAccount = await program.account.config.fetch(configPda);
    console.log("🚀 ~ Pause ~ configAccount:", configAccount)

    const [userInfoPda, _] = PublicKey.findProgramAddressSync(
        [Buffer.from(SEED_USERINFO), payer.publicKey.toBytes()],
        program.programId
    );

    console.log("🚀 ~ Stake ~ userInfoPda:", userInfoPda.toBase58())
    const userInfoAccount = await program.account.user.fetch(userInfoPda);
    console.log("🚀 ~ Stake ~ userInfoAccount:", userInfoAccount)

    const { provider } = program;
    const slot = await provider.connection.getSlot('confirmed');
    const blockTime = await provider.connection.getBlockTime(slot);

    if (blockTime !== null) {
        console.log("blockTime: ", blockTime); // Output: Block time as bigint

        const userDeposit = userInfoAccount.deposit.toNumber();
        console.log("userDeposit: ", userDeposit);
        const userDebt = userInfoAccount.debt.toNumber();
        console.log("userDebt: ", userDebt);
        const totalRate = configAccount.totalRate;
        console.log("totalRate: ", totalRate);
        const claimPeriod = configAccount.claimPeriod.toNumber();
        console.log("claimPeriod: ", claimPeriod);
        const lastRewardTime = configAccount.lastRewardTime.toNumber();
        console.log("lastRewardTime: ", lastRewardTime);
        const totalDeposit = configAccount.totalDeposit.toNumber();
        console.log("totalDeposit: ", totalDeposit);
        const rewardMultiplier = configAccount.rewardMultiplier.toNumber();
        console.log("rewardMultiplier: ", rewardMultiplier);

        calcReward(userDeposit, userDebt, blockTime, totalRate, claimPeriod, lastRewardTime, totalDeposit, rewardMultiplier)
    } else {
        console.log("Block time is null");
    }
}


function getMultiplier(from: number, to: number, rewardMultiplier: number): number {
    console.log("getMultiplier:: from:", from, "to:", to);

    if (to < from) {
        throw new Error("Invalid value: 'to' must be greater than or equal to 'from'");
    }

    const duration: number = to - from;
    const result: number = duration * rewardMultiplier;

    console.log(
        "getMultiplier:: duration:", duration,
        "rewardMultiplier:", rewardMultiplier,
        "result:", result
    );

    return result;
}


function calcReward(
    userDeposit: number,
    userDebt: number,
    timeStamp: number,
    totalRate: number,
    claimPeriod: number,
    lastRewardTime: number,
    totalDeposit: number,
    rewardMultiplier: number
): string {
    let accPerShare: number = totalRate;

    console.log(
        "calcReward: accPerShare:", accPerShare,
        "totalRate:", totalRate
    );

    const nTimeStamp = Math.floor(timeStamp / claimPeriod) * claimPeriod;

    console.log(
        "nTimeStamp:", nTimeStamp,
        "claimPeriod:", claimPeriod,
        "accPerShare:", accPerShare
    );

    if (nTimeStamp > lastRewardTime && totalDeposit !== 0) {
        const nMultiplier = getMultiplier(lastRewardTime, nTimeStamp, rewardMultiplier);

        console.log(
            "nMultiplier:", nMultiplier,
            "lastRewardTime:", lastRewardTime,
            "nTimeStamp:", nTimeStamp
        );

        accPerShare += nMultiplier / totalDeposit;

        console.log("Updated accPerShare:", accPerShare);
    }

    const result = userDeposit * accPerShare - userDebt;

    console.log(
        "Result:", result,
        "User Deposit:", userDeposit,
        "User Debt:", userDebt
    );

    const divisor = 10 ** 6;
    const resultNoLamport = (result / divisor).toString();
    console.log("resultNoLamport: ", resultNoLamport);

    const depositNoLamport = (userDeposit / divisor).toString();
    console.log("depositNoLamport: ", depositNoLamport);

    return resultNoLamport;
}
