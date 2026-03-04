use anchor_lang::prelude::*;

declare_id!("SolFlux111111111111111111111111111111111");

#[program]
pub mod solflux_staking {
    use super::*;

    // Stake NFT
    pub fn stake_nft(ctx: Context<StakeNFT>) -> Result<()> {
        let stake_account = &mut ctx.accounts.stake_account;
        stake_account.owner = *ctx.accounts.user.key;
        stake_account.staked_at = Clock::get()?.unix_timestamp;

        msg!("NFT Staked Successfully");
        Ok(())
    }

    // Unstake NFT
    pub fn unstake_nft(ctx: Context<StakeNFT>) -> Result<()> {
        msg!("NFT Unstaked Successfully");
        Ok(())
    }

    // Claim Rewards
    pub fn claim_rewards(ctx: Context<StakeNFT>) -> Result<()> {
        msg!("Rewards Claimed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StakeNFT<'info> {
    #[account(init_if_needed, payer = user, space = 8 + 64)]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub staked_at: i64,
}
