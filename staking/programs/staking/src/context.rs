
use anchor_lang::prelude::*;
use anchor_spl::{
    token::{TokenAccount, Token, Mint, Transfer}
};
use crate::state::*;

pub const AUTHORITY_SEED: &str = "authority";
pub const CUSTODY_SEED: &str = "custody";
pub const STAKE_ACCOUNT_METADATA_SEED : &str = "stake_metadata";
pub const CONFIG_SEED: &str = "config";

#[derive(Accounts)]
#[instruction(config_data : global_config::GlobalConfig)]
pub struct InitConfig<'info>{
    // Native payer
    #[account(mut)]
    pub payer : Signer<'info>,
    #[account(
        init,
        seeds = [CONFIG_SEED.as_bytes()],
        bump,
        payer = payer,
    )]
    // Stake program accounts:
    pub config_account : Account<'info, global_config::GlobalConfig>,
    // Primitive accounts:
    pub rent: Sysvar<'info, Rent>,
    pub system_program : Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(owner : Pubkey, lock : vesting::VestingSchedule)]
pub struct CreateStakeAccount<'info>{
    // Native payer:
    #[account(mut)]
    pub payer : Signer<'info>,
    // Stake program accounts:
    #[account(zero)]
    pub stake_account_positions : AccountLoader<'info, positions::PositionData>,
    #[account(
        init,
        seeds = [CUSTODY_SEED.as_bytes(), stake_account_positions.key().as_ref()],
        bump,
        payer = payer,
        token::mint = mint,
        token::authority = custody_authority,
    )]
    pub stake_account_custody : Account<'info, TokenAccount>,
    #[account(init, payer = payer, seeds = [STAKE_ACCOUNT_METADATA_SEED.as_bytes(), stake_account_positions.key().as_ref()], bump)]
    pub stake_account_metadata : Account<'info, stake_account::StakeAccountMetadata>,
    /// CHECK : This AccountInfo is safe because it's a checked PDA
    #[account(seeds = [AUTHORITY_SEED.as_bytes(), stake_account_positions.key().as_ref()], bump)]
    pub custody_authority : AccountInfo<'info>,
    #[account(seeds = [CONFIG_SEED.as_bytes()], bump = config.bump)]
    pub config : Account<'info, global_config::GlobalConfig>,
    // Pyth token mint:
    #[account(address = config.pyth_token_mint)]
    pub mint: Account<'info, Mint>,
    // Primitive accounts :
    pub rent: Sysvar<'info, Rent>,
    pub token_program : Program<'info, Token>,
    pub system_program : Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount : u64)]
pub struct WithdrawStake<'info>{
    // Native payer:
    #[account( address = stake_account_metadata.owner)]
    pub payer : Signer<'info>,
    // Destination
    #[account(mut)]
    pub destination : Account<'info, TokenAccount>,
    // Stake program accounts:
    pub stake_account_positions : AccountLoader<'info, positions::PositionData>,
    #[account(seeds = [STAKE_ACCOUNT_METADATA_SEED.as_bytes(), stake_account_positions.key().as_ref()], bump = stake_account_metadata.metadata_bump)]
    pub stake_account_metadata : Account<'info, stake_account::StakeAccountMetadata>,
    #[account(
        mut,
        seeds = [CUSTODY_SEED.as_bytes(), stake_account_positions.key().as_ref()],
        bump = stake_account_metadata.custody_bump,
    )]
    pub stake_account_custody : Account<'info, TokenAccount>,
    /// CHECK : This AccountInfo is safe because it's a checked PDA
    #[account(seeds = [AUTHORITY_SEED.as_bytes(), stake_account_positions.key().as_ref()], bump = stake_account_metadata.authority_bump)]
    pub custody_authority : AccountInfo<'info>,
    #[account(seeds = [CONFIG_SEED.as_bytes()], bump = config.bump)]
    pub config : Account<'info, global_config::GlobalConfig>,
    // Primitive accounts :
    pub token_program : Program<'info, Token>,
}

impl<'a, 'b, 'c, 'info> From<&WithdrawStake<'info>> for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
    fn from(accounts: &WithdrawStake<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.stake_account_custody.to_account_info(),
            to: accounts.destination.to_account_info(),
            authority: accounts.custody_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}


#[derive(Accounts)]
#[instruction(product : Pubkey, publisher : Pubkey, amount : u64)]
pub struct CreatePostion<'info>{
    // Native payer:
    #[account( address = stake_account_metadata.owner)]
    pub payer : Signer<'info>,
    // Stake program accounts:
    #[account(mut)]
    pub stake_account_positions : AccountLoader<'info, positions::PositionData>,
    #[account(seeds = [STAKE_ACCOUNT_METADATA_SEED.as_bytes(), stake_account_positions.key().as_ref()], bump = stake_account_metadata.metadata_bump)]
    pub stake_account_metadata : Account<'info, stake_account::StakeAccountMetadata>,
    #[account(
        seeds = [CUSTODY_SEED.as_bytes(), stake_account_positions.key().as_ref()],
        bump = stake_account_metadata.custody_bump,
    )]
    pub stake_account_custody : Account<'info, TokenAccount>,
    #[account(seeds = [CONFIG_SEED.as_bytes()], bump = config.bump)]
    pub config : Account<'info, global_config::GlobalConfig>,
}


#[derive(Accounts)]
pub struct SplitPosition<'info>{
    pub payer : Signer<'info>,
}

#[derive(Accounts)]
pub struct ClosePosition<'info>{
    pub payer : Signer<'info>,
}

#[derive(Accounts)]
pub struct CleanupPostions<'info>{
    pub payer : Signer<'info>,
}

