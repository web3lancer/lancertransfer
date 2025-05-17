use anchor_lang::prelude::*;

#[account]
pub struct Milestone {
    pub job_id: u64,
    pub client: Pubkey,
    pub freelancer: Pubkey,
    pub amount: u64,
    pub released: bool,
    pub approved: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[error_code]
pub enum MilestoneError {
    #[msg("Milestone already released")] 
    AlreadyReleased,
    #[msg("Milestone not approved")] 
    NotApproved,
}

// Add milestone instructions in the main lib.rs as needed
