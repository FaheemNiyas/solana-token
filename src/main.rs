use solana_program::{
    account_info::{AccountInfo, next_account_info},
    borsh::try_from_slice_unchecked,
    pubkey::Pubkey,
    sysvar::Sysvar,
    program_error::ProgramError,
    msg,
};

// Example constants for token supply and tax percentages
const TOKEN_SUPPLY: u64 = 1_000_000;
const SELL_TAX_PERCENT: u8 = 2;
const BUY_TAX_PERCENT: u8 = 1;

// Struct representing the token data
#[derive(Debug, Default, PartialEq)]
pub struct Token {
    pub supply: u64,
    pub sell_tax_percent: u8,
    pub buy_tax_percent: u8,
}

impl Token {
    pub fn new(supply: u64, sell_tax_percent: u8, buy_tax_percent: u8) -> Self {
        Token {
            supply,
            sell_tax_percent,
            buy_tax_percent,
        }
    }
}

// Entry point function to initialize the token
fn initialize_token(
    program_id: &Pubkey,
    token_account: &mut AccountInfo,
) -> Result<(), ProgramError> {
    // Initialize token data
    let token_data = Token::new(TOKEN_SUPPLY, SELL_TAX_PERCENT, BUY_TAX_PERCENT);

    // Serialize token data into account
    let mut data = token_account.data.borrow_mut();
    token_data.pack_into_slice(&mut data);

    Ok(())
}

fn main() {
    // Replace with your actual program ID
    let program_id = Pubkey::new_from_array([
        0x4w, 0xGf, 0xS4, 0xDU, 0YF, 0xY1, 0G2, 0i, 0UP, 0x5w, 0td, 0x91, 0Eg, 0gd, 0dp, 0jE, 0qk, 0Sb, 0Saz, 0vd, 0jgc, 0s,
    ]);

    // Example accounts for testing (replace with actual accounts)
    let mut token_account = AccountInfo::new(Pubkey::default(), false, false);

    // Initialize token
    match initialize_token(&program_id, &mut token_account) {
        Ok(_) => {
            msg!("Token initialized successfully!");
            // Example: Print token data
            let token_data = Token::try_from_slice_unchecked(&token_account.data.borrow()).unwrap();
            msg!("Token Data: {:?}", token_data);
        },
        Err(err) => {
            msg!("Error initializing token: {:?}", err);
        }
    }
}
