# SSE Token Staking Contract

# Prerequites

Install Rust, Solana, and AVM: https://solana.com/docs/intro/installation

Remember to install anchor v0.30.1.


# Quick Start

## Build the program
```bash
# build the program
RUSTUP_TOOLCHAIN="nightly-2024-11-19" anchor build

```

## Test program on devnet

Set the cluster as devnet in `Anchor.toml`:
```bash
[provider]
cluster = "<DEVNET_RPC>"
```

Deploy program:
```bash
anchor deploy
```

## Features

- **Initialize Program**: Allows the admin to configure and set up the program parameters.  
- **Deposit Funds**: Admin deposits tokens to fund rewards for users.  
- **Stake Tokens**: Users can stake their tokens to participate in the reward system.  
- **Unstake Tokens**: Users can withdraw their staked tokens along with earned rewards.

## Use CLI to test the program

### Set cli/commnad.ts:
```bash
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
      "devnet rpc url"
    )
    .option(
      "-k, --keypair <string>",
      "Solana wallet Keypair Path",
      "./keys/*.json"               //*.json: Your Keypair.json
    );
}
```

Initialize program: Admin sets the configuration status of the program.
```bash
yarn script config


   const newConfig = {
        authority: payer.publicKey,
        pendingAuthority: payer.publicKey,//payer.publicKey,//PublicKey.default,

        tokenMintConfig: TOKEN_ADDRESS,
        claimPeriod: new BN(30),
        totalRate: new BN(0),
        totalStakers: new BN(0),
        lastReward_time: new BN(0),
        rewardMultiplier: new BN(0),
        depositTime: new BN(0),
        totalDeposit: new BN(0),
        purchaseAmt: new BN(1_209_600_000_000),

        initialized: true,
    };
```

Deposit Fund: Admin deposits tokens to fund rewards for users.
```bash
yarn script deposit
```

Stake Tokens: Users can stake their tokens to participate in the reward system.
```bash
yarn script stake -a TOKEN_AMOUNT
```

Unstake Tokens: Users can withdraw their staked tokens along with earned rewards.
```bash
yarn script unstake
```


## Staking & Rewards System  

- **`purchaseAmt`** represents the total reward token allocation for a **2-week period**.  
- Users can stake their tokens into the contract’s vault to earn rewards.  

### Example Scenario  

1. **User A stakes 100 tokens** → Their staking information is saved on-chain.  
   - If User A is the only staker, they receive **100% of the rewards** for the staking period.  

2. **User B stakes 400 tokens after User A** → Their staking information is also saved on-chain.  
   - Now, the total stake pool is **500 tokens**.  
   - User B holds **4/5 (80%)** of the total stake and receives **80% of the rewards**.  
   - User A now holds **1/5 (20%)** and receives **20% of the rewards**.  

3. **Unstaking**:  
   - Users can **unstake** at any time to withdraw their **original staked tokens** along with their **earned reward tokens**.  

if user wanna unstack, user can unstack and get origin stake token and reward token.









