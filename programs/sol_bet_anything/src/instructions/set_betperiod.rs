use anchor_lang::prelude::*;
use crate::state::List;

#[derive(Accounts)]
pub struct SetBetPeriod<'info> {
    #[account(mut)]
    pub signer: Signer<'info>, // The caller (Judge or SBA Admin)

    #[account(
        mut
    )]
    pub list: Account<'info, List>, // The game (list) being ended

}

impl<'info> SetBetPeriod<'info> {
    pub fn set_period(&mut self, status: u8) -> Result<()> {
        let list = &mut self.list; // Access the List account
    
        list.status = status;
    
        Ok(())
    }    

}


