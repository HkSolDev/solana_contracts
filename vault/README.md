# 🏦 Anchor Vault — Solana Smart Contract

A simple vault program built on Solana using the **Anchor** framework.  
This contract securely holds SOL and allows users to deposit and withdraw.

---

## 🛠️ Features

### 1️⃣ Deposit
Users can transfer SOL into their vault PDA. The vault ensures the minimum rent balance is met and accurately transfers the funds securely using CPI.

### 2️⃣ Withdraw
Users can withdraw their deposited SOL from the vault PDA. The escrow PDA signs for the transaction to unlock the funds.

---

## 🧠 Core Concept

The vault is a **Program Derived Address (PDA)** derived from:
- the string `"vault"`
- the signer's public key

This ensures each user has their own unique, secure vault that only the smart contract can sign for.

---

## 🔗 Main Project

Head back to the main project and Escrow Contract here:  
➡️ [Anchor Escrow Contract](../anchor_escrow/README.md)
