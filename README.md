**SOL BET ANYTHING(Capstone Project)**

Your Game. Your Rules

Bet anything from Sports games to Event forecasts to Token Predictions to X(Twitter) Banters. All you need is Solana.

**RUST PROGRAM INFORMATION:**

**Program ID:** BtYYc5eyu3Eg1WPsJTE3mh1yXFeknwH4xyqhKL8qRUzW **USER ROLES:**

User

Maker

Judge

Admin

Signer (Dual Roles: Admin and Judge)

**STATUS CODE:**

**0: Review** - This is the first status code after creating a bet.

**1: Open** - After the Judge validates the bet(accepts their role as a judge), the status code is set to 1, and the bet is open. Setting the Judge as an SBA Judge(admin) automatically sets the status code to 1.

For SBA Judge, the maker is required to submit an advisement which is the same as an appeal after the bet ends.

**2: Judgement** - After the bet period is reached the game ends and the status code is set to 2 if an SBA Judge is not set as the judge.

**3: Appeal** - This is the status code if a game is appealed(The players disagree with the judge declaration)

**4: Declared** - This is the status code after a Judge declares the winner of the bet

**5: Amended** - This is the status code after an SBA Judge(Admin) declares the winner of the bet

6. **Closed** - The status code for the game being closed can’t be appealed or amended anymore. Winners can then withdraw their payouts.
6. **Advisement** - Same as Appeal but it is set so users can help the SBA Judge determine the winner.

   **FUNCTIONS:**

   **inititalize**: Initialize the code. Set the Admin and other admin information. **createGame**: Creates the game.

   **getGame**: Get Game Information.

   **validateGame**: Judge provided accepts its role in the game. If SBA Judge is set, the process is handled automatically. There’s no need to call validateGame.

   **placeBet**: Users can get to place a bet on the game.

   **endGame**: For the admin to end the game once a bet period has been reached. **declareWinner**: For the judge or admin to declare the winner of the game.

   **makeAppeal**: For the user to appeal the declaration of the winner before the game is finally closed.

   **getAppeal**: Get Appeal Information.

   **closeGame**: After the appeal period has been exceeded. The game is closed by the admin.

   **payWinner**: This is used to pay the winner of the game. **changeAdmin**: To change the current admin account. **changeAdminFees**: To change the fees of the Admin.

   **withdraw**: To withdraw from the Treasury to the Admin Account

   **STATE**

   ADMIN CONFIG STATE

admin: Pubkey, payout\_fee: u8, appeal\_fee: u64, next\_bet\_key: u64, treasury\_bump: u8, bump: u8,

// Admin account's public key

// Fee percentage of the payout (or other fee config) // Fee charged for Appeal in Lamports

// Counter for the next unique bet key

// Bump for the treasury PDA (for storing SOL)

// Bump for this admin config PDA

LiST STATE(GAME)

name: String,

description: String,

bet\_key: u64,

options: Vec<String>, option\_counts: Vec<u32>, amount: u64,

pool\_amount: u64,

pool\_no: u16,

bet\_period: u64, creation\_timestamp: i128, judge: Pubkey,

maker: Pubkey,

status: u8,

winner: u8,

payout: u64,

appealed: u8, declaration\_timestamp: i128, ended\_timestamp: i128, close\_timestamp: i128, treasury\_bump: u8,

pub bump: u8,

// Name of the list

// Description of the list

// Unique key for bets

// Flexible number of options (2 to 4)

// Counter for each option (number of bets per option) // Individual bet amount (default per bet)

// Total SOL staked across all bets

// Number of bets placed

// Bet period in seconds

// Timestamp of list creation

// Judge's public key

// Maker's public key

// Status of the list

// Winner (option\_a or option\_b)

// Winner Payout

// Check if decision has been appealed

// Timestamp of winner declaration

// Timestamp of game ended

// Timestamp of game closed

// Bump for the treasury PDA (for storing SOL)

// Bump for this list PDA

BET STATE

account: Pubkey, // Bettor's public key

bet\_key: u64, // Linked game (List) key

option: u8, // Option chosen (0 for A, 1 for B) bump: u8, // Bump for this PDA

APPEAL STATE

account: Pubkey, // Account filing the appeal

bet\_key: u64, // Bet's associated key

description: String, // Description of the appeal appeal\_url: String, // Supporting documentation or URL bump: u8, // Bump for this PDA
