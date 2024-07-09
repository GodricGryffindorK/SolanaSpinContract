
pub const DEV_WALLET_KEY: &str = "74abUnzELqrKsJip68xVQdvduUUzQanjsdoN6jnRzqfp";

pub const INITIALIZER_KEY: &str = "3ttYrBAp5D2sTG2gaBjg8EtrZecqBQSBuFRhsqHWPYxX";
// pub const FRONK_MINT: &str = "5yxNbU8DgYJZNi3mPD9rs4XLh9ckXrhPjJ5VCujUWg5H"; //mainnet
pub const FRONK_MINT: &str = "EBjBZHvnhCyQXFQJrjcu66PqBbhh6bHhjA5z7Cjyb5oD"; //devnet

pub const ESCROW_PDA_SEED: &str = "sw_game_vault_auth";
pub const USER_STATE_SEED: &[u8] = b"USER_STATE_SEED";
pub const ADMIN_LIST_SEED: &[u8] = b"ADMIN_LIST_SEED";
pub const VAULT_SEED: &[u8] = b"SOL_VAULT";
pub const LAST_USERS_SEED: &str = "LAST_USERS_SEED";


pub const SPIN_ITEM_COUNT: usize = 15;
pub const REWARD_TOKEN_COUNT_PER_ITEM: usize = 10;
pub const ADMIN_MAX_COUNT: usize = 15;
pub const MAX_LATEST_USER_COUNT: usize = 10;
pub const MAX_REWARD_TOKEN_COUNT: usize = 150; // REWARD_TOKEN_COUNT_PER_ITEM * SPIN_ITEM_COUNT;

pub const REWARD_TOKEN_DECIMAL: u8 = 5;
pub const PERCENT_MULTIPLIER: u64 = 1000;
