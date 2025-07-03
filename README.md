# Stablecoin Smart Contract on Solana

This is a stablecoin smart contract built with **Rust** and the **Anchor framework** for the **Solana blockchain**. The program allows users to mint and burn stablecoins backed by on-chain collateral. It also supports price oracles and a liquidation mechanism to maintain solvency and protocol safety.

---

## Features

- **Collateralized Minting**  
  Users can deposit approved collateral tokens to mint a stablecoin at a predefined collateralization ratio.

- **Burning & Redemption**  
  Users can burn stablecoins to redeem their locked collateral.

- **Oracle Integration**  
  Real-time price data is fetched via oracles (e.g., Pyth) to accurately value collateral and ensure system integrity.

- **Liquidation Mechanism**  
  Undercollateralized positions can be liquidated by third-party liquidators. A liquidation bonus is awarded to the liquidator as an incentive.

---

## Development Status

- [x] Core stablecoin mint/burn logic  
- [x] Oracle integration for price feeds  
- [x] Liquidation mechanism with incentives  
- [ ] Test coverage (unit and integration tests)  
- [ ] Frontend interface (planned with React/Next.js)  

---

## Getting Started

### Prerequisites

- Rust and Cargo (`rustup` recommended)
- Solana CLI tools
- Anchor CLI (`cargo install --git https://github.com/coral-xyz/anchor anchor-cli`)
- Optional: Docker for local Solana validator

### Build

```bash
anchor build

