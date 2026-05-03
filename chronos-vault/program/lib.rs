use anchor_lang::prelude::*;

declare_id!("ChrVau1111111111111111111111111111111111111");

#[program]
pub mod chronos_vault {
    use super::*;

    const MIN_GRACE_PERIOD: i64 = 86400; // 24 Hours (Censorship Buffer)

    /// Register a new heritage vault.
    pub fn create_vault(
        ctx: Context<CreateVault>,
        heir: Pubkey,
        secondary_pinger: Option<Pubkey>,
        grace_period: i64, 
        mxe_id: [u8; 32],
    ) -> Result<()> {
        // SECURITY GUARD: Prevent tiny grace periods (Censorship protection)
        require!(
            grace_period >= MIN_GRACE_PERIOD,
            ErrorCode::GracePeriodTooShort
        );

        // SECURITY GUARD: Owner cannot be the heir (self-inheritance makes no sense)
        require!(
            ctx.accounts.owner.key() != heir,
            ErrorCode::OwnerCannotBeHeir
        );

        let vault = &mut ctx.accounts.vault;
        vault.owner = ctx.accounts.owner.key();
        vault.heir = heir;
        vault.secondary_pinger = secondary_pinger;
        vault.grace_period = grace_period;
        vault.last_heartbeat = Clock::get()?.unix_timestamp;
        vault.mxe_id = mxe_id;
        vault.status = VaultStatus::Active;
        
        Ok(())
    }

    /// Reset the inactivity clock. 
    /// SECURITY GUARD: Allows both owner AND secondary pinger (Social Heartbeat).
    pub fn ping_heartbeat(ctx: Context<PingHeartbeat>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let signer = ctx.accounts.pinger.key();

        // SECURITY GUARD: Only vault must be Active to receive heartbeats
        require!(
            vault.status == VaultStatus::Active,
            ErrorCode::VaultNotActive
        );

        require!(
            signer == vault.owner || (vault.secondary_pinger.is_some() && signer == vault.secondary_pinger.unwrap()),
            ErrorCode::UnauthorizedPinger
        );

        vault.last_heartbeat = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// Mark the vault as ready for reveal. 
    /// SECURITY FIX #5: Only the HEIR can trigger the reveal.
    pub fn trigger_reveal(ctx: Context<TriggerReveal>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let now = Clock::get()?.unix_timestamp;

        // SECURITY FIX #5: Only heir can trigger
        require!(
            ctx.accounts.claimer.key() == vault.heir,
            ErrorCode::UnauthorizedClaimer
        );

        // Must still be Active (not already Unlocked or Settled)
        require!(
            vault.status == VaultStatus::Active,
            ErrorCode::VaultNotActive
        );
        
        require!(
            now >= vault.last_heartbeat + vault.grace_period,
            ErrorCode::GracePeriodNotOver
        );

        vault.status = VaultStatus::Unlocked;
        msg!("Vault Unlocked. Arcium MXE can now be settled.");
        Ok(())
    }

    /// SECURITY FIX #7: Allow owner to update heir or close the vault.
    pub fn update_vault(
        ctx: Context<OwnerOnly>,
        new_heir: Option<Pubkey>,
        new_secondary_pinger: Option<Option<Pubkey>>,
        new_grace_period: Option<i64>,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        // Can only update while Active
        require!(
            vault.status == VaultStatus::Active,
            ErrorCode::VaultNotActive
        );

        if let Some(heir) = new_heir {
            require!(vault.owner != heir, ErrorCode::OwnerCannotBeHeir);
            vault.heir = heir;
        }
        if let Some(pinger) = new_secondary_pinger {
            vault.secondary_pinger = pinger;
        }
        if let Some(period) = new_grace_period {
            require!(period >= MIN_GRACE_PERIOD, ErrorCode::GracePeriodTooShort);
            vault.grace_period = period;
        }

        // Reset heartbeat on any update (owner is clearly alive)
        vault.last_heartbeat = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// SECURITY FIX #7: Allow owner to permanently destroy the vault.
    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        // Anchor's `close` attribute handles the lamport refund.
        // The vault PDA is wiped and rent is returned to the owner.
        msg!("Vault closed and destroyed by owner.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(
        init, 
        payer = owner, 
        // SECURITY FIX #6: Correct space calculation
        // 8 (discriminator) + 32 (owner) + 32 (heir) + 33 (Option<Pubkey>) 
        // + 8 (last_heartbeat) + 8 (grace_period) + 32 (mxe_id) + 1 (status) = 154
        space = 8 + 32 + 32 + 33 + 8 + 8 + 32 + 1,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PingHeartbeat<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub pinger: Signer<'info>,
}

// SECURITY FIX #5: TriggerReveal now requires a signer (the heir)
#[derive(Accounts)]
pub struct TriggerReveal<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub claimer: Signer<'info>,
}

#[derive(Accounts)]
pub struct OwnerOnly<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, Vault>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(mut, has_one = owner, close = owner)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub heir: Pubkey,
    pub secondary_pinger: Option<Pubkey>,
    pub last_heartbeat: i64,
    pub grace_period: i64,
    pub mxe_id: [u8; 32],
    pub status: VaultStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum VaultStatus {
    Active,
    Unlocked,
    Settled,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The grace period has not yet expired. The owner might still be active.")]
    GracePeriodNotOver,
    #[msg("The grace period must be at least 24 hours to prevent censorship attacks.")]
    GracePeriodTooShort,
    #[msg("Only the owner or the designated secondary pinger can reset the clock.")]
    UnauthorizedPinger,
    #[msg("Only the designated heir can trigger the vault reveal.")]
    UnauthorizedClaimer,
    #[msg("This vault is no longer active (already unlocked or settled).")]
    VaultNotActive,
    #[msg("The owner cannot designate themselves as the heir.")]
    OwnerCannotBeHeir,
}
