use anchor_lang::prelude::*;
use crate::error::BetError;
use crate::state::{List, AdminConfig};

#[derive(Accounts)]
pub struct CloseGame<'info> {
    #[account(mut)]
    pub signer: Signer<'info>, // The caller (Judge or SBA Admin)

    #[account(
        mut,
        seeds = [b"list", list.maker.key().as_ref(), &list.bet_key.to_le_bytes()],
        bump = list.bump,
        constraint = list.status == 1 @ BetError::InvalidGameStatus // Game must be active (status == 1)
    )]
    pub list: Account<'info, List>, // The game (list) being ended

    #[account(
        seeds = [b"admin_config"],
        bump = admin_config.bump
    )]
    pub admin_config: Account<'info, AdminConfig>, // Admin config holding the admin public key
}

impl<'info> CloseGame<'info> {
    pub fn close_game(&mut self) -> Result<()> {
        let list = &mut self.list; // Access the List account
    
        // Check if the game has either status 4 (ended but not closed) or 5 (appealed)
        if list.status != 4 && list.status != 5 {
            return Err(error!(BetError::InvalidGameStatus)); // Custom error if the game is not in a valid status to close
        }
    
        // Update the close timestamp to the current time
        list.close_timestamp = Clock::get()?.unix_timestamp as i128;
    
        // Calculate payout for each option
        let total_payout = list.pool_amount; // Total pool amount
        let total_bets = list.pool_no as u64; // Total number of bets placed
    
        // Ensure there are bets placed before calculating the payout
        if total_bets == 0 {
            return Err(error!(BetError::NoBetsPlaced)); // Custom error if no bets were placed
        }
    
        // Calculate the payout for the winning option
        let payout = if list.option_counts.len() > list.winner as usize {
            let winning_option_bets = list.option_counts[list.winner as usize];
            
            if winning_option_bets == 0 {
                return Err(error!(BetError::NoBetsPlaced)); // Custom error if no bets were placed on the winning option
            }
    
            // Calculate the payout: pool amount * amount per bet / number of bets on winning option
            let payout_per_bet = (total_payout * list.amount) / (winning_option_bets as u64);
            payout_per_bet
        } else {
            return Err(error!(BetError::InvalidWinnerOption)); // Custom error if the winner option index is invalid
        };
    
        // Set the payout for the List state (payout for the winning option)
        list.payout = payout;
    
        // Optionally, update the status to "closed" (status 6) or any other status as needed
        list.status = 6; // Mark the game as closed
    
        Ok(())
    }    

}


