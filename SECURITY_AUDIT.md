# 🛡️ Security Audit Report

## Arcane Vote — 4 Vulnerabilities Found

### 🔴 CRITICAL #1: `settle_tally` has NO caller restriction (Line 145-151)
**Attack**: Anyone can call `settle_tally` with a crafted `TallyResult`. The comment on line 149 literally says *"In a real implementation, we would verify the caller."* A black hat would read this and immediately exploit it.
**Fix**: Add a `settler` signer and verify it matches the proposal creator or a whitelisted relayer.

### 🔴 CRITICAL #2: `settle_tally` can be called MULTIPLE times (No re-entrancy guard)
**Attack**: Even after a proposal is settled (`settled = true`), nothing prevents calling `settle_tally` again with different numbers. An attacker could override a "NO" result with a "YES" result after the fact.
**Fix**: Add `require!(!proposal.settled, ErrorCode::AlreadySettled)`.

### 🟡 MEDIUM #3: `record_vote` has NO time check (Lines 54-67)
**Attack**: Votes can be recorded AFTER the voting period ends. A black hat could wait to see the Arcium tally result, then stuff extra votes before the next tally attempt.
**Fix**: Add `require!(Clock::get()?.unix_timestamp < proposal.end_time, ErrorCode::VotingClosed)`.

### 🟡 MEDIUM #4: Vote choice validation is too loose in tally circuit (Line 50-53)
**Attack**: The `_ => abstain` catch-all means ANY integer (e.g., `999`, `-50`) gets counted as "Abstain." While votes are encrypted, a malicious client could submit garbage data that inflates the abstain count and skews participation metrics.
**Fix**: Explicitly reject invalid vote values instead of silently counting them.

---

## Chronos Vault — 3 Vulnerabilities Found

### 🔴 CRITICAL #5: `trigger_reveal` can be called by ANYONE (Lines 52-66, 91-95)
**Attack**: A random stranger (not the heir) can call `trigger_reveal` and flip the vault status to `Unlocked`. While they can't decrypt the Arcium data directly, this prematurely signals to the MXE that the vault is ready, creating a race condition.
**Fix**: Restrict `trigger_reveal` to only the heir.

### 🔴 CRITICAL #6: Account space is too small (Line 74)
**Attack**: The `space` calculation `8 + 32 + 32 + 8 + 8 + 32 + 1 = 121` does NOT account for the `Option<Pubkey>` (secondary_pinger = 1 + 32 = 33 bytes). This will cause a **buffer overflow** at runtime, silently corrupting data or crashing the program.
**Fix**: Correct the space calculation to include `Option<Pubkey>`.

### 🟡 MEDIUM #7: Owner cannot REVOKE or change their heir
**Attack**: If the owner's relationship with the heir deteriorates, or if the heir's wallet is compromised, the owner has NO way to update the heir address or destroy the vault. The compromised heir just waits for the grace period.
**Fix**: Add `update_vault` and `close_vault` instructions.
