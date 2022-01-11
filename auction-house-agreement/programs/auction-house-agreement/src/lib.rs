use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod auction_house_agreement {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }

    pub fn create_agreement(ctx: Context<CreateAgreement>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateAgreement<'info> {
    #[account(init, payer = requester, space = 32 + 32 + 1)]
    pub agreement: Account<'info, Agreement>,
    #[account()]
    pub auction_house: Account<'info, AuctionHouse>,
    #[account(mut)]
    pub requester: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AuctionHouse {}

#[account]
pub struct Agreement {
    pub authorizer: Pubkey,
    pub auction_house: Pubkey,
    pub approved: bool,
}
