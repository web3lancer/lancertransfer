use anchor_lang::prelude::*;

#[account]
pub struct Review {
    pub contract_id: u64,
    pub reviewer: Pubkey,
    pub reviewee: Pubkey,
    pub rating: u8, // 0-5
    pub comment: String,
    pub created_at: i64,
}

#[error_code]
pub enum ReviewError {
    #[msg("Invalid rating (must be 0-5)")]
    InvalidRating,
    #[msg("Review already exists for this contract")]
    AlreadyReviewed,
}

// Add review instructions in the main lib.rs as needed
