use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::{token::{TokenAccount, Token, Mint, Transfer as SplTransfer}, associated_token::AssociatedToken};

declare_id!("BaRtR9LkkzMWysBndwLf1wwWyHTHCYEGiPPa2ooXmodW");


#[program]
pub mod security_series {
    use anchor_lang::system_program;

    use super::*;

    pub fn init_player(ctx: Context<InitPlayerAccounts>) -> Result<()> {
        msg!("setup player account");
        // ctx.accounts.player_account.player =ctx.accounts.player.key;
        // ctx.accounts.player_account.bump = *ctx.bumps.get("player_account").unwrap();
        // ctx.accounts.player_account.points = 0;
        // ctx.accounts.player_account.lucky_number = 0;

        Ok(())
    }

    pub fn do_nothing(ctx: Context<InitPlayerAccounts>) -> Result<()> {
        msg!("let's chill");
        Ok(())
    }

}



#[derive(Accounts)]
pub struct InitPlayerAccounts<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(init, payer = player, space = 8+32+1+4+4, seeds=[b"player", player.key().as_ref()], bump)]
    pub player_account: Account<'info, PlayerAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PlayerAccount {
    pub player: Pubkey,
    pub bump: u8,
    pub points: u32,
    pub lucky_number: u32
}