use anchor_lang::prelude::*;
use crate::error::BetError;
use crate::state::{List, AdminConfig};

#[derive(Accounts)]
pub struct DeclareWinner<'info> {
    #[account(mut)]
    pub signer: Signer<'info>, // The caller (Judge or Admin)

    #[account(
        mut,
        seeds = [b"list", list.maker.key().as_ref(), &list.bet_key.to_le_bytes()],
        bump = list.bump,
        constraint = list.status == 2 @ BetError::InvalidGameStatus // Game must be ended (status == 2)
    )]
    pub list: Account<'info, List>, // The game (list) for which the winner is being declared

    #[account(
        seeds = [b"admin_config"],
        bump = admin_config.bump
    )]
    pub admin_config: Account<'info, AdminConfig>, // Admin config holding the admin public key
}


impl<'info> DeclareWinner<'info> {
    pub fn declare_winner(&mut self, winner: u8) -> Result<()> {
        let list = &mut self.list;

        // Ensure the caller is authorized (Judge, Maker, or Admin)
        let caller = self.signer.key();
        if caller != list.judge && caller != self.admin_config.admin {
            return Err(error!(BetError::UnauthorizedAccess)); // Custom error
        }

        // Ensure the game status is "Ended" (2)
        if list.status != 2 && list.status != 3 {
            return Err(error!(BetError::InvalidGameStatus)); // Game status must be "Ended" or under appeal
        }

        // Set the winner (the winning option)
        if winner < 1 || winner > list.options.len() as u8 {
            return Err(error!(BetError::InvalidWinnerOption)); // Invalid winner option (must be within range)
        }

        list.winner = winner; // Set the winning option (1-based index)
        list.declaration_timestamp = Clock::get()?.unix_timestamp as i128; // Set the timestamp of winner declaration

        // Change the status to "Completed" (status = 4)
        if caller == self.admin_config.admin {
            list.status = 5;
        }else{
            list.status = 4;
        }
        

        Ok(())
    }
}