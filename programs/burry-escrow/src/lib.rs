use anchor_lang::prelude::*;
use instructions::deposit::*;
use instructions::withdraw::*;

pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("GKUk4ByGnL8xKAMmC8ktY6Q9V8318LEXCttkaBLpkdPS");

#[program]
pub mod burry_escrow {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, escrow_amt: u64, unlock_price: u64) -> Result<()> {
        deposit_handler(ctx, escrow_amt, unlock_price)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw_handler(ctx)
    }
}
