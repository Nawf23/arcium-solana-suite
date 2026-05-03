'use client';

import React, { useState, useEffect } from 'react';
import './globals.css';

export default function ChronosVault() {
  const [timeLeft, setTimeLeft] = useState(2592000); // 30 days in seconds
  const [isPinging, setIsPinging] = useState(false);

  useEffect(() => {
    const timer = setInterval(() => {
      setTimeLeft((prev) => (prev > 0 ? prev - 1 : 0));
    }, 1000);
    return () => clearInterval(timer);
  }, []);

  const formatTime = (seconds: number) => {
    const d = Math.floor(seconds / (3600 * 24));
    const h = Math.floor((seconds % (3600 * 24)) / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    return `${d}d ${h}h ${m}m ${s}s`;
  };

  const handlePing = () => {
    setIsPinging(true);
    setTimeout(() => {
      setTimeLeft(2592000);
      setIsPinging(false);
    }, 1000);
    // In a real app:
    // 1. Send transaction to Solana Anchor program (ping_heartbeat)
    // 2. Wait for confirmation
  };

  return (
    <div className="main-container">
      <header>
        <div className="logo">
          <div className="logo-icon"></div>
          CHRONOS VAULT
        </div>
        <div style={{ display: 'flex', gap: '1rem' }}>
           <button className="btn btn-outline" style={{ width: 'auto' }}>Heir Portal</button>
           <button className="btn btn-primary" style={{ width: 'auto' }}>0x82...f2a</button>
        </div>
      </header>

      <main className="dashboard-grid">
        {/* Heartbeat Status */}
        <section className="card heartbeat-display">
          <div className="timer-circle">
            <span className="timer-label">Grace Period</span>
            <span className="timer-value">{timeLeft > 0 ? timeLeft : 'EXPIRED'}</span>
            <span className="timer-label">Remaining</span>
          </div>
          <h2 style={{ marginBottom: '0.5rem' }}>Next Release In</h2>
          <p style={{ color: 'var(--text-secondary)', marginBottom: '2rem' }}>
            {formatTime(timeLeft)}
          </p>
          <button 
            className="btn btn-primary" 
            onClick={handlePing}
            disabled={isPinging}
          >
            {isPinging ? 'Sending Life Signal...' : 'Send Heartbeat (I am Alive)'}
          </button>
          <p style={{ fontSize: '0.75rem', marginTop: '1rem', color: 'var(--text-secondary)' }}>
            Last heartbeat recorded: 12 minutes ago
          </p>
        </section>

        {/* Vault Management */}
        <section className="card">
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '2rem' }}>
            <h2>Your Vaults</h2>
            <button className="btn btn-outline" style={{ width: 'auto', fontSize: '0.8rem' }}>+ Create New</button>
          </div>

          <div className="vault-item">
            <span className="vault-icon">🔑</span>
            <div className="vault-details">
              <h3>Hardware Wallet Recovery</h3>
              <p>IPFS Hash: QmXy...z72v</p>
            </div>
            <span className="confidential-tag">MXE SECURED</span>
          </div>

          <div className="vault-item">
            <span className="vault-icon">📄</span>
            <div className="vault-details">
              <h3>Estate Instructions</h3>
              <p>IPFS Hash: QmR9...b41k</p>
            </div>
            <span className="confidential-tag">MXE SECURED</span>
          </div>

          <div className="vault-item">
            <span className="vault-icon">🖼️</span>
            <div className="vault-details">
              <h3>Private Art Collection</h3>
              <p>IPFS Hash: QmB2...a93n</p>
            </div>
            <span className="confidential-tag">MXE SECURED</span>
          </div>

          <div style={{ marginTop: '2.5rem', padding: '1.5rem', background: 'rgba(0, 242, 255, 0.05)', borderRadius: '20px', border: '1px solid var(--accent-glow)' }}>
            <h4 style={{ color: var(--accent-color), marginBottom: '0.5rem' }}>Heir: 0x52...92k</h4>
            <p style={{ fontSize: '0.8rem', color: 'var(--text-secondary)' }}>
              This address will receive the Arcium decryption authority if your heartbeat expires.
            </p>
          </div>
        </section>
      </main>

      <footer style={{ marginTop: '4rem', color: 'var(--text-secondary)', fontSize: '0.8rem', textAlign: 'center' }}>
        <p>Protected by Arcium MXE #8291 • Solana Mainnet Settlement</p>
      </footer>
    </div>
  );
}
