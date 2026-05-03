use anchor_lang::prelude::*;
use arcium_solana_sdk::prelude::*;

declare_id!("ArcVot1111111111111111111111111111111111111");

#[program]
pub mod arcane_vote {
    use super::*;

    /// Initialize the global configuration (Admin only)
    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.admin = ctx.accounts.admin.key();
        Ok(())
    }

    /// Add a trusted Arcium MXE to the whitelist
    pub fn whitelist_mxe(ctx: Context<ManageWhitelist>, mxe_id: [u8; 32]) -> Result<()> {
        let config = &mut ctx.accounts.config;
        if !config.whitelisted_mxes.contains(&mxe_id) {
            config.whitelisted_mxes.push(mxe_id);
        }
        Ok(())
    }

    /// Initialize a new confidential proposal.
    pub fn initialize_proposal(
        ctx: Context<InitializeProposal>,
        proposal_id: u64,
        description_hash: [u8; 32],
        end_time: i64,
        mxe_id: [u8; 32], 
    ) -> Result<()> {
        // SECURITY GUARD: Ensure the MXE is trusted
        require!(
            ctx.accounts.config.whitelisted_mxes.contains(&mxe_id),
            ErrorCode::UntrustedMxe
        );

        // SECURITY GUARD: end_time must be in the future
        require!(
            end_time > Clock::get()?.unix_timestamp,
            ErrorCode::InvalidEndTime
        );

        let proposal = &mut ctx.accounts.proposal;
        proposal.id = proposal_id;
        proposal.description_hash = description_hash;
        proposal.end_time = end_time;
        proposal.mxe_id = mxe_id;
        proposal.creator = ctx.accounts.creator.key();
        proposal.vote_count = 0;
        proposal.settled = false;
        
        Ok(())
    }

    /// Record an encrypted vote blob. 
    /// The actual vote content is encrypted for the Arcium MXE.
    pub fn record_vote(
        ctx: Context<RecordVote>,
        encrypted_vote_blob: Vec<u8>,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;

        // SECURITY FIX #3: Reject votes after the voting period ends
        require!(
            Clock::get()?.unix_timestamp < proposal.end_time,
            ErrorCode::VotingClosed
        );

        let vote_record = &mut ctx.accounts.vote_record;
        
        vote_record.voter = ctx.accounts.voter.key();
        vote_record.encrypted_data = encrypted_vote_blob;
        
        proposal.vote_count += 1;
        
        Ok(())
    }

    /// Callback function triggered by the Arcium Network after confidential tallying.
    /// This function verifies the Arcium proof and settles the proposal.
    pub fn settle_tally(
        ctx: Context<SettleTally>,
        tally_result: TallyResult,
        proof: ArciumProof,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        
        // SECURITY FIX #2: Prevent double-settlement
        require!(!proposal.settled, ErrorCode::AlreadySettled);

        // SECURITY FIX #1: Verify the settler is the proposal creator
        require!(
            ctx.accounts.settler.key() == proposal.creator,
            ErrorCode::UnauthorizedSettler
        );

        // Verification: Ensure the proof matches the MXE ID and current state
        arcium_solana_sdk::verify_mxe_proof(
            &proposal.mxe_id,
            &tally_result,
            &proof
        ).map_err(|_| error!(ErrorCode::InvalidArciumProof))?;

        proposal.yes_votes = tally_result.yes_count;
        proposal.no_votes = tally_result.no_count;
        proposal.abstain_votes = tally_result.abstain_count;
        proposal.settled = true;

        msg!("Proposal {} settled: YES={}, NO={}, ABSTAIN={}", 
             proposal.id, proposal.yes_votes, proposal.no_votes, proposal.abstain_votes);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(init, payer = admin, space = 8 + 32 + (4 + 32 * 10))]
    pub config: Account<'info, GlobalConfig>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ManageWhitelist<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, GlobalConfig>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct InitializeProposal<'info> {
    pub config: Account<'info, GlobalConfig>,
    #[account(
        init, 
        payer = creator, 
        space = 8 + 8 + 32 + 8 + 32 + 32 + 8 + 1 + (4 * 4),
        seeds = [b"proposal", proposal_id.to_le_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(
        init,
        payer = voter,
        space = 8 + 32 + 256,
        seeds = [b"vote", proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>,
    #[account(mut)]
    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// SECURITY FIX #1: SettleTally now requires a signer
#[derive(Accounts)]
pub struct SettleTally<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub settler: Signer<'info>,
}

#[account]
pub struct GlobalConfig {
    pub admin: Pubkey,
    pub whitelisted_mxes: Vec<[u8; 32]>,
}

#[account]
pub struct Proposal {
    pub id: u64,
    pub creator: Pubkey,
    pub description_hash: [u8; 32],
    pub end_time: i64,
    pub mxe_id: [u8; 32],
    pub vote_count: u64,
    pub settled: bool,
    pub yes_votes: i32,
    pub no_votes: i32,
    pub abstain_votes: i32,
}

#[account]
pub struct VoteRecord {
    pub voter: Pubkey,
    pub encrypted_data: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct TallyResult {
    pub yes_count: i32,
    pub no_count: i32,
    pub abstain_count: i32,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The Arcium proof provided is invalid or does not match the proposal.")]
    InvalidArciumProof,
    #[msg("The provided Arcium MXE ID is not in the trusted whitelist.")]
    UntrustedMxe,
    #[msg("Voting period has ended. No more votes can be recorded.")]
    VotingClosed,
    #[msg("This proposal has already been settled. Results cannot be overwritten.")]
    AlreadySettled,
    #[msg("Only the proposal creator can settle the tally.")]
    UnauthorizedSettler,
    #[msg("The end time must be in the future.")]
    InvalidEndTime,
}
