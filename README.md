# Simple Router Example

This is a minimal Solana program example for a request router. It's set up as an onchain entrypoint that routes to any specified program.

> [!WARNING]
> This code has not been audited. Use at own risk.

## Pre-requisites

In order to run, follow installation instructions from the Solana [docs](https://solana.com/docs/intro/installation).

Before proceeding, you should have:

-   Rust
-   Solana CLI
-   Anchor CLI

## Getting Started

1. Clone the repo
2. Install dependencies

```bash
yarn install
```

3. Build the programs

```bash
anchor build
```

4. Run test

```bash
anchor test --skip-build
```
