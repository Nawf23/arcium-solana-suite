# 🛡️ Arcium Solana Privacy Suite
**A collection of Decentralized Confidential Computing (DeCC) solutions for the Solana Ecosystem.**

This repository contains two production-ready projects built for the Arcium Hackathon, demonstrating how Arcium's Multi-Party Computation (MPC) nodes can provide privacy and cryptographic access control to Solana applications.

---

## 🔮 1. Arcane Vote (Confidential DAO Governance)
**Problem**: Strategic voting and bandwagon effects in DAOs.
**Solution**: Arcium-powered confidential tallies where individual votes are hidden until the deadline.
*   **Location**: `/arcane-vote`
*   **Key Feature**: 10-minute safety buffer against Solana clock drift.

## ⏳ 2. Chronos Vault (Digital Heritage & Dead Man's Switch)
**Problem**: Loss of digital assets due to owner inactivity or death.
**Solution**: A trustless "guardian" that reveals secrets to heirs only after a verified period of inactivity.
*   **Location**: `/chronos-vault`
*   **Key Feature**: Social Heartbeat support and Censorship-resistant grace periods.

---

## 🛠️ Technical Stack
- **Confidential Layer**: Arcium Network (Arcis Circuits)
- **Settlement Layer**: Solana (Anchor Program)
- **Frontend**: Next.js 14, Web3.js, Arcium SDK
- **Security**: Full Audit conducted on all smart contracts.

## 🧪 Security & Audit
We take privacy and security seriously. All contracts in this suite have been audited for:
- ✅ Re-entrancy and Double-Settlement guards.
- ✅ Unauthorized caller protection.
- ✅ Logic validation in confidential circuits.
- ✅ Buffer overflow and account space safety.

*Full Audit Report can be found in the project root.*

---
Built with ❤️ for the Arcium Ecosystem.
