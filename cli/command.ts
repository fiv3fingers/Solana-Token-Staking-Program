import { program } from "commander";
import {
    configProject,
    Deposit,
    Stake,
    UnStake,
    Claim,
    setClusterConfig,
    Pause,
    GetUserInfo,
} from "./scripts";

program.version("0.0.1");

programCommand("config")
    .action(async (directory, cmd) => {
        const { env, keypair, rpc } = cmd.opts();

        console.log("Solana Cluster:", env);
        console.log("Keypair Path:", keypair);
        console.log("RPC URL:", rpc);

        await setClusterConfig(env, keypair, rpc);
        await configProject();
    });

programCommand("deposit")
    .action(async (directory, cmd) => {
        const { env, keypair, rpc } = cmd.opts();

        console.log("Solana Cluster:", env);
        console.log("Keypair Path:", keypair);
        console.log("RPC URL:", rpc);

        await setClusterConfig(env, keypair, rpc);
        await Deposit();
    });


programCommand("stake")
    .option("-a, --amount <number>", "swap amount")
    .action(async (directory, cmd) => {
        const { env, keypair, rpc, amount } = cmd.opts();

        console.log("Solana Cluster:", env);
        console.log("Keypair Path:", keypair);
        console.log("RPC URL:", rpc);

        await setClusterConfig(env, keypair, rpc);
        await Stake(amount);
    });

programCommand("unstake")
    .action(async (directory, cmd) => {
        const { env, keypair, rpc, userPubkey } = cmd.opts();

        console.log("Solana Cluster:", env);
        console.log("Keypair Path:", keypair);
        console.log("RPC URL:", rpc);

        await setClusterConfig(env, keypair, rpc);
        await UnStake();
    });

programCommand("claim")
    .action(async (directory, cmd) => {
        const { env, keypair, rpc } = cmd.opts();

        console.log("Solana Cluster:", env);
        console.log("Keypair Path:", keypair);
        console.log("RPC URL:", rpc);

        await setClusterConfig(env, keypair, rpc);
        await Claim();
    });

programCommand("pause")
    .option("-s, --stop <number>", "stop")
    .action(async (directory, cmd) => {
        const { env, keypair, rpc, stop } = cmd.opts();

        console.log("Solana Cluster:", env);
        console.log("Keypair Path:", keypair);
        console.log("RPC URL:", rpc);

        await setClusterConfig(env, keypair, rpc);
        await Pause(stop);
    });

programCommand("getuserinfo")
    .option("-s, --stop <number>", "stop")
    .action(async (directory, cmd) => {
        const { env, keypair, rpc } = cmd.opts();

        console.log("Solana Cluster:", env);
        console.log("Keypair Path:", keypair);
        console.log("RPC URL:", rpc);

        await setClusterConfig(env, keypair, rpc);
        await GetUserInfo();
    });


function programCommand(name: string) {
    return program
        .command(name)
        .option(
            //  mainnet-beta, testnet, devnet
            "-e, --env <string>",
            "Solana cluster env name",
            "devnet"
        )
        .option(
            "-r, --rpc <string>",
            "Solana cluster RPC name",
            "https://api.devnet.solana.com"
        )
        .option(
            "-k, --keypair <string>",
            "Solana wallet Keypair Path",
            "./keys/EgBcC7KVQTh1QeU3qxCFsnwZKYMMQkv6TzgEDkKvSNLv.json"  //DQ8fi6tyN9MPD5bpSpUXxKd9FVRY2WcnoniVEgs6StEW//EgBcC7KVQTh1QeU3qxCFsnwZKYMMQkv6TzgEDkKvSNLv //EcagE8oN5WLAbEUBmALRqRA7H5auvRLbt8ve8Nf3atX4
        )
}

program.parse(process.argv);