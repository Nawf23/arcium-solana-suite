# ⏳ Chronos Vault
**Decentralized Digital Heritage & Dead Man's Switch on Solana powered by Arcium**

Chronos Vault ensures that your most sensitive digital assets—private keys, recovery phrases, and legal instructions—are never lost. By combining Solana's transparency with Arcium's confidentiality, we've created a "Zero-Trust" heritage platform. Your secrets stay encrypted and hidden until a pre-defined period of inactivity is detected on-chain.

## 🌟 Key Features
- **Decentralized Heartbeat**: Signal your activity via a simple Solana transaction.
- **Confidential Guardian**: Arcium MPC nodes protect your master key without ever seeing it.
- **Automatic Inheritance**: Your heir can claim the secrets only after your grace period expires.
- **Trustless & Serverless**: No central authority can revoke access or peak at your data.

## 🛠️ Architecture
- **Solana (Anchor)**: Manages the "Heartbeat" logic and provides the ground-truth for inactivity.
- **Arcium (Arcis)**: Enforces the "Conditional Reveal" policy in a confidential execution environment.
- **Local Encryption**: Files are encrypted on your device; only the master key is stored in Arcium.

## 🚀 Getting Started

### 1. Arcium Circuit
Deploy the guardian logic:
```bash
arcium-cli compile circuit/guardian.arcis
arcium-cli deploy --mxe-id <ID>
```

### 2. Solana Program
Deploy the heartbeat program:
```bash
anchor build
anchor deploy
```

### 3. Frontend
Start the dashboard:
```bash
cd app
npm install
npm run dev
```

## 🔒 Privacy Benefits
- **Secret Sharing**: Your master key is split across multiple Arcium nodes.
- **Policy-Locked**: Decryption is mathematically impossible until the Solana condition is met.
- **Owner Control**: You can update your heir or reset the clock at any time.

---
Built for the Arcium Hackathon.
