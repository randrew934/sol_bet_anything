SOL BET ANYTHING(Capstone Project)

Your Game. Your Rules

Bet anything from Sports games to Event forecasts to Token Predictions to X(Twitter) Banters. All you need is Solana.


RUST PROGRAM INFORMATION:

Program ID: BtYYc5eyu3Eg1WPsJTE3mh1yXFeknwH4xyqhKL8qRUzW

USER ROLES:

User
Maker
Judge
Admin
Signer (Dual Roles: Admin and Judge)

STATUS CODE:

0: Review - This is the first status code after creating a bet.

1: Open - After the Judge validates the bet(accepts their role as a judge), the status code is set to 1, and the bet is open. Setting the Judge as an SBA Judge(admin) automatically sets the status code to 1. 

For SBA Judge, the maker is required to submit an advisement which is the same as an appeal after the bet ends.

2: Judgement - After the bet period is reached the game ends and the status code is set to 2 if an SBA Judge is not set as the judge.

3: Appeal - This is the status code if a game is appealed(The players disagree with the judge declaration)

4: Declared - This is the status code after a Judge declares the winner of the bet

5: Amended - This is the status code after an SBA Judge(Admin) declares the winner of the bet

6. Closed - The status code for the game being closed can’t be appealed or amended anymore. Winners can then withdraw their payouts.

7. Advisement - Same as Appeal but it is set so users can help the SBA Judge determine the winner.

FUNCTIONS:

inititalize: Initialize the code. Set the Admin and other admin information.

createGame: Creates the game.

getGame: Get Game Information.

validateGame: Judge provided accepts its role in the game. If SBA Judge is set, the process is handled automatically. There’s no need to call validateGame.

placeBet: Users can get to place a bet on the game.

endGame: For the admin to end the game once a bet period has been reached.

declareWinner: For the judge or admin to declare the winner of the game.

makeAppeal: For the user to appeal the declaration of the winner before the game is finally closed.

getAppeal: Get Appeal Information.

closeGame: After the appeal period has been exceeded. The game is closed by the admin. 

payWinner: This is used to pay the winner of the game.

changeAdmin: To change the current admin account.

changeAdminFees: To change the fees of the Admin.

withdraw: To withdraw from the Treasury to the Admin Account


STATE

ADMIN CONFIG STATE

 admin: Pubkey,     		// Admin account's public key
 payout_fee: u8,    	 	// Fee percentage of the payout (or other fee config)
 appeal_fee: u64,   		// Fee charged for Appeal in Lamports
 next_bet_key: u64, 	// Counter for the next unique bet key
 treasury_bump: u8,		// Bump for the treasury PDA (for storing SOL)
 bump: u8,   			// Bump for this admin config PDA

LiST STATE(GAME)

name: String,                		// Name of the list
description: String,         		// Description of the list
bet_key: u64,                	   	// Unique key for bets
options: Vec<String>,        	   	// Flexible number of options (2 to 4)
option_counts: Vec<u32>,     	// Counter for each option (number of bets per option)
amount: u64,                 		// Individual bet amount (default per bet)
pool_amount: u64,            		// Total SOL staked across all bets
pool_no: u16,                		// Number of bets placed
bet_period: u64,             		// Bet period in seconds
creation_timestamp: i128,   	// Timestamp of list creation
judge: Pubkey,               		// Judge's public key
maker: Pubkey,               		// Maker's public key
status: u8,                  		// Status of the list
winner: u8,                  		// Winner (option_a or option_b)
payout: u64,                 		// Winner Payout
appealed: u8,               		// Check if decision has been appealed 
declaration_timestamp: i128, 	// Timestamp of winner declaration
ended_timestamp: i128,       	// Timestamp of game ended
close_timestamp: i128,       	// Timestamp of game closed
treasury_bump: u8,           		// Bump for the treasury PDA (for storing SOL)
pub bump: u8,                    		// Bump for this list PDA



BET STATE

account: Pubkey, 		// Bettor's public key
bet_key: u64,    		// Linked game (List) key
option: u8,      		// Option chosen (0 for A, 1 for B)
bump: u8,        		// Bump for this PDA



APPEAL STATE

account: Pubkey,     		// Account filing the appeal
bet_key: u64,        		// Bet's associated key
description: String, 		// Description of the appeal
appeal_url: String,  		// Supporting documentation or URL
bump: u8,            		// Bump for this PDA

