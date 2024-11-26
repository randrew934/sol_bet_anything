use anchor_lang::prelude::*;

use crate::state::List;

use crate::error::BetError;

#[derive(Accounts)]
pub struct GetGameWinners<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // User who is querying the game winners

    #[account(mut)]
    pub list: Account<'info, List>, // The game (list) to query
}

impl<'info> GetGameWinners<'info> {
    pub fn get_game_winners(&mut self) -> Result<u32> {
        let list = &mut self.list;

        // Ensure the game has been ended
        if list.status != 6 {
            return Err(error!(BetError::GameNotEnded)); // Custom error if the game hasn't ended
        }

        // Retrieve the winning option
        let winner_option = list.winner;

        // Check if the winner is within the bounds of the options
        if winner_option >= list.option_counts.len() as u8 {
            return Err(error!(BetError::InvalidWinnerOption)); // Custom error for invalid winner option
        }

        // Get the number of bets placed on the winning option
        let winning_option_count = list.option_counts[winner_option as usize];

        Ok(winning_option_count)
    }
}
