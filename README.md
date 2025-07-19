# Stable Coin

## Overview

This project implements a decentralized stablecoin protocol on the Solana blockchain using the Anchor framework. The stablecoin is backed by SOL collateral and uses real-time price feeds (via Pyth) to ensure proper collateralization, health factor enforcement, and liquidation mechanisms. The protocol allows users to deposit SOL as collateral, mint stablecoins, redeem collateral by burning stablecoins, and participate in liquidations of under-collateralized positions.

---

## Table of Contents
- [Architecture](#architecture)
- [Smart Contract Logic](#smart-contract-logic)
  - [Accounts & State](#accounts--state)
  - [Instructions](#instructions)
    - [Admin](#admin)
    - [Deposit](#deposit)
    - [Withdraw/Redeem](#withdrawredeem)
    - [Liquidation](#liquidation)
- [Health Factor & Liquidation](#health-factor--liquidation)
- [Price Feeds](#price-feeds)
- [Error Handling](#error-handling)
- [Testing & Usage](#testing--usage)
- [Development Setup](#development-setup)
- [Dependencies](#dependencies)

---

## Architecture

- **Solana Program (Rust, Anchor):**
  - Main logic in `programs/stablecoin/src/`.
  - Modularized into instructions: `admin`, `deposit`, `withdrawl`, and shared `utils`.
  - State managed via Anchor accounts: `Config` and `Collateral`.
- **Client/Tests (TypeScript):**
  - Example usage and integration tests in `tests/stablecoin.ts`.
  - Uses Anchor and Pyth client libraries.

---

## Smart Contract Logic

### Accounts & State

- **Config**: Stores protocol-wide settings (authority, mint, liquidation thresholds, bonuses, min health factor, etc).
- **Collateral**: Stores per-user collateral and minted stablecoin info (depositor, balances, bump seeds, etc).

### Instructions

#### Admin
- **initialize_config**: Initializes the protocol config and stablecoin mint. Sets authority, mint, liquidation parameters, and bumps.
- **update_config**: Allows updating the minimum health factor (risk parameter) for the protocol.

#### Deposit
- **deposit_collateral_and_mint_tokens**: User deposits SOL as collateral and mints stablecoins. Updates collateral account, checks health factor, and mints tokens to the user.

#### Withdraw/Redeem
- **reddem_collateral_and_burn_tokens**: User burns stablecoins to redeem a proportional amount of their SOL collateral. Updates collateral account, checks health factor, burns tokens, and transfers SOL back to the user.

#### Liquidation
- **liquidate**: If a user's health factor falls below the minimum, anyone can liquidate their position. The liquidator burns stablecoins, receives the equivalent SOL collateral plus a liquidation bonus, and the user's position is updated.

---

## Health Factor & Liquidation

- **Health Factor**: Ratio of (collateral value * liquidation threshold) to amount minted. Must stay above `min_health_factor`.
- **Liquidation**: If health factor drops below the minimum, the account can be liquidated. The liquidator receives a bonus (configurable) on top of the collateral value equivalent to the burned stablecoins.

---

## Price Feeds

- Uses Pyth price feeds to fetch real-time SOL/USD prices.
- Ensures that collateralization and health factor calculations are based on up-to-date market data.
- Price feed account is passed as an argument to relevant instructions.

---

## Error Handling

Custom errors are defined for:
- Invalid price data
- Health factor violations (below minimum)
- Attempting to liquidate a healthy account

---

## Testing & Usage

- Example tests in `tests/stablecoin.ts` demonstrate:
  - Initializing the protocol
  - Depositing collateral and minting stablecoins
  - Redeeming collateral and burning stablecoins
  - Updating config parameters
  - Liquidating unhealthy accounts

### Example Test Flow
```typescript
it("Deposit Collateral and Mint USDC", async () => {
  const amountCollateral = 1_000_000_000;
  const amountToMint = 1_000_000_000;
  const tx = await program.methods.
    depositCollateralMintTokens(new anchor.BN(amountCollateral), new anchor.BN(amountToMint))
    .accounts({ priceUpdate: solUsdPriceFeedAccount }).rpc({ skipPreflight: true, commitment: "confirmed" })
  console.log("✅ collateral deposited and tokens minted : ", tx)
})
```

---

## Development Setup

### Prerequisites
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://book.anchor-lang.com/chapter_2/installation.html)
- Node.js & npm/yarn

### Build & Test
```bash
# Install dependencies
npm install
# Build the Solana program
anchor build
# Run tests
anchor test
```

---

## Dependencies

- **Rust/Anchor**: `anchor-lang`, `anchor-spl`, `pyth-solana-receiver-sdk`
- **TypeScript**: `@coral-xyz/anchor`, `@pythnetwork/pyth-solana-receiver`, `@solana/web3.js`, `mocha`, `chai`, `typescript`

---

## Notes
- The protocol is for educational/demo purposes and has not been audited.
- All parameters (liquidation threshold, bonus, min health factor) are configurable via the admin instructions.
- The program is designed to be extensible for additional collateral types or risk parameters. 