# Arcane Vote
**Confidential DAO Governance on Solana powered by Arcium**

Arcane Vote solves the "Strategic Voting" problem in decentralized governance. By using Arcium's Decentralized Confidential Computing (DeCC), votes are cast and tallied inside a secure, encrypted multi-party environment. The results are only revealed once the voting period ends, ensuring a fair and unbiased outcome for every proposal.

## Key Features
- **Private Ballots**: Individual votes are encrypted locally and never exposed on-chain.
- **Confidential Tallying**: Arcium MPC nodes sum the votes without seeing the individual choices.
- **On-Chain Settlement**: Results are published to Solana with a cryptographic proof of correctness.
- **Strategic Integrity**: Prevents "bandwagon effects" where voters wait for a leader to emerge.

## Architecture
- **Solana (Anchor)**: Acts as the settlement layer for proposals and final tallies.
- **Arcium (Arcis)**: A confidential circuit handles the heavy lifting of encrypted math.
- **IPFS/S3**: Stores the encrypted voting blobs referenced by Solana.

## Getting Started

### 1. Arcium Circuit
Compile the Arcis circuit for the MXE:
```bash
arcium-cli compile circuit/tally.arcis
arcium-cli deploy --mxe-id <ID>
```

### 2. Solana Program
Deploy the Anchor program to Devnet/Mainnet:
```bash
anchor build
anchor deploy
```

### 3. Frontend
Install dependencies and start the dashboard:
```bash
cd app
npm install
npm run dev
```

## Privacy Benefits
- **End-to-End Encryption**: Data is encrypted using the voter's key and re-encrypted for the Arcium MXE.
- **Zero Knowledge of Inputs**: No single Arcium node can see a voter's choice.
- **Trustless Enforcement**: The "Reveal" condition is enforced by code, not humans.

---
Built for the Arcium Hackathon.
