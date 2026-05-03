'use client';

import React, { useState } from 'react';
import './globals.css';

export default function ArcaneVote() {
  const [selected, setSelected] = useState<string | null>(null);
  const [voted, setVoted] = useState(false);

  const handleVote = (choice: string) => {
    setSelected(choice);
  };

  const submitVote = () => {
    if (!selected) return;
    setVoted(true);
    // In a real app: 
    // 1. Fetch Arcium MXE session
    // 2. Encrypt { choice: selected }
    // 3. Submit encrypted blob to Solana via Anchor
  };

  return (
    <div className="main-container">
      <header>
        <div className="logo">
          <span>🔮</span> ARCANE VOTE
        </div>
        <button className="wallet-btn">Connect Wallet</button>
      </header>

      <main>
        {!voted ? (
          <div className="proposal-card">
            <div className="status-badge">ACTIVE VOTING</div>
            <div className="confidential-seal">
              🔒 Confidential Arcium MXE Active
            </div>
            <h1>Allocate 10k SOL to Security Audit?</h1>
            <p className="description">
              This proposal seeks to allocate 10,000 SOL from the DAO treasury to conduct 
              a comprehensive security audit of our v2 Smart Contracts. Voting is confidential 
              to prevent strategic bandwagoning.
            </p>

            <div className="voting-options">
              <div 
                className={`vote-btn ${selected === 'yes' ? 'selected' : ''}`}
                onClick={() => handleVote('yes')}
              >
                <span className="vote-icon">👍</span>
                <span>YES</span>
              </div>
              <div 
                className={`vote-btn ${selected === 'no' ? 'selected' : ''}`}
                onClick={() => handleVote('no')}
              >
                <span className="vote-icon">👎</span>
                <span>NO</span>
              </div>
              <div 
                className={`vote-btn ${selected === 'abstain' ? 'selected' : ''}`}
                onClick={() => handleVote('abstain')}
              >
                <span className="vote-icon">😶</span>
                <span>ABSTAIN</span>
              </div>
            </div>

            <button 
              className="wallet-btn" 
              style={{ width: '100%', marginTop: '2rem', height: '60px', fontSize: '1.1rem' }}
              onClick={submitVote}
              disabled={!selected}
            >
              Cast Confidential Vote
            </button>

            <div className="footer-info">
              <div className="arcium-tag">
                <div className="glow-icon"></div>
                <span>Secured by Arcium Network</span>
              </div>
              <div className="arcium-tag" style={{ fontSize: '0.8rem' }}>
                Ends in: 14h 22m
              </div>
            </div>
          </div>
        ) : (
          <div className="proposal-card" style={{ textAlign: 'center' }}>
             <div className="vote-icon" style={{ fontSize: '5rem', marginBottom: '1rem' }}>🎉</div>
             <h1>Vote Recorded!</h1>
             <p className="description">
               Your vote has been encrypted and stored in Arcium's shared state. 
               The current tally is hidden to ensure a fair outcome.
             </p>
             <div className="arcium-tag" style={{ justifyContent: 'center' }}>
                <span>Transaction Hash:</span>
                <span style={{ color: 'var(--accent-color)' }}>0x7f...d82a</span>
             </div>
             <button 
              className="wallet-btn" 
              style={{ marginTop: '2rem', background: 'transparent', border: '1px solid var(--border-color)' }}
              onClick={() => setVoted(false)}
            >
              Back to Proposal
            </button>
          </div>
        )}
      </main>
    </div>
  );
}
