## Week 3 - Final exam

Audit the following smart contracts written in Rust.

### Security analysis - Solana program

*Note: This example is taken from a Solana program of the Grass competition on Cantina.*

Audit the `slashing_handler` logic. Find at least 2 issues in the following code.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Mint, Token, TokenAccount, Transfer};
use crate::errors::ErrorCode;
use crate::{stake_pool_signer_seeds, state::StakePool };  
  
#[derive(Accounts)] 
pub struct Slashing<'info> {
    // Payer to actually stake the mint tokens
    #[account(mut)]
    pub authority: Signer<'info>,  

    /// Vault of the StakePool token will be transfer to
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub stake_mint: Account<'info, Mint>,

    /// StakePool owning the vault that holds the deposit
    #[account(
        mut,
        has_one = vault @ ErrorCode::InvalidStakePoolVault,
        has_one = stake_mint @ ErrorCode::InvalidAuthority,
    )]
    pub stake_pool: AccountLoader<'info, StakePool>,
    pub token_program: Program<'info, Token>,
}
 
pub fn slashing_handler<'info>(
    ctx: Context<Slashing>,
    amount: u64,
    router: u8,
    is_locked: u8 
) -> Result<()> {
    {    
        let stake_pool = &mut ctx.accounts.stake_pool.load_mut()?;
        let pool = &mut stake_pool.reward_pools[usize::from(router)];
        pool.is_locked = is_locked;

        let cpi_ctx = CpiContext {
            program: ctx.accounts.token_program.to_account_info(),
            accounts: Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.stake_pool.to_account_info(),
            },
            remaining_accounts: Vec::new(),
            signer_seeds: &[stake_pool_signer_seeds!(stake_pool)],
        };
        let _ = token::transfer(cpi_ctx, amount);

        Ok(())
    } 
}
```

- Additional ressources
    - `StakePool` structure
        
        ```rust
        #[assert_size(568)]
        #[account(zero_copy)]
        #[repr(C)]
        pub struct StakePool { 
            /// The original creator of the StakePool. Necessary for signer seeds
            pub creator: Pubkey,
            /** Pubkey that can make updates to StakePool */
            pub authority: Pubkey,
            /** Pubkey that can lock any reward pool */
            pub locker: Pubkey,
            /** Total amount staked that accounts for the lock up period weighting.
            Note, this is not equal to the amount of SPL Tokens staked. */
            pub total_weighted_stake: u128,
            /** Token Account to store the staked SPL Token */
            pub vault: Pubkey,
            /** Mint of the token being staked */
            pub mint: Pubkey,
            /** Mint of the token representing effective stake */
            pub stake_mint: Pubkey,
            /// Array of RewardPools that apply to the stake pool.
            /// Unused entries are Pubkey default. In arbitrary order, and may have gaps.
            pub reward_pools: [RewardPool; MAX_REWARD_POOLS],
            /// The minimum weight received for staking. In terms of 1 / SCALE_FACTOR_BASE.
            /// Examples:
            /// * `min_weight = 1 x SCALE_FACTOR_BASE` = minmum of 1x multiplier for > min_duration staking
            /// * `min_weight = 2 x SCALE_FACTOR_BASE` = minmum of 2x multiplier for > min_duration staking
            pub base_weight: u64,
            /// Maximum weight for staking lockup (i.e. weight multiplier when locked
            /// up for max duration). In terms of 1 / SCALE_FACTOR_BASE. Examples:
            /// * A `max_weight = 1 x SCALE_FACTOR_BASE` = 1x multiplier for max staking duration
            /// * A `max_weight = 2 x SCALE_FACTOR_BASE` = 2x multiplier for max staking duration
            pub max_weight: u64,
            /** Minimum duration for lockup. At this point, the staker would receive the base weight. In seconds. */
            pub min_duration: u64,
            /** Maximum duration for lockup. At this point, the staker would receive the max weight. In seconds. */
            pub max_duration: u64,
            /** Nonce to derive multiple stake pools from same mint */
            pub nonce: u8,
            /** Bump seed for stake_mint */
            pub bump_seed: u8,
            // padding to next 8-byte
            _padding0: [u8; 6],
            _reserved0: [u8; 256]
        }
        
        ```
        
    - `token` documentation: https://docs.rs/anchor-spl/latest/anchor_spl/token
    - CpiContext explanations
        
        The CpiContext structure initialized in the code snippet is used to make Cross-Program Invocation (CPI) in Solana.
        
        This allows calling other programs from a program. Here, the Slashing program will call the Token program.
        

### Security analysis - Soroban contract

*Note: This example is based on an audit.*

The content of this exercise is available at https://github.com/zigtur/vulnerable-NFT-soroban.

- Audit the NFT smart contract written for the Stellar blockchain (Soroban environment).
- Write an audit report about this audit (explain the issue(s) you have found).
- Write at least one Proof-of-Concept as a unit test (in `test.rs`).
- Soroban help
    
    The Soroban environment has one concept that is uncommon to other blockchains.
    
    - Storage entries can have different type: `Temporary`, `Instance` and `Persistent`
    - `Temporary` variables may be erased
    - A Time To Live (TTL) is attached to storage entries. This must be extended to not expire.
    - See: https://developers.stellar.org/docs/learn/encyclopedia/storage/persisting-data

### Security analysis - Node implementation

The content of this exercise is available at https://github.com/zigtur/vulnerable-HTTP-server.

For this exercise:

- Research and explain what do lines 57 to 60 do? https://github.com/zigtur/vulnerable-HTTP-server/blob/main/src/main.rs#L57-L60
- Find 2 vulnerabilities that crash the node
- Write a PoC for each vulnerability. Prefer writing your PoC in Rust :)
    - reqwest crate may be useful