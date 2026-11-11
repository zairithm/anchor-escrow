# anchor-escrow

[![CI](https://github.com/zairithm/anchor-escrow/actions/workflows/ci.yml/badge.svg)](https://github.com/zairithm/anchor-escrow/actions/workflows/ci.yml)

A Solana program, written with Anchor, that holds a maker's tokens until a taker pays the agreed amount, so neither side has to trust the other.

## What this project covers

- Anchor accounts, constraints and PDAs
- SPL token transfers through CPI
- Program tests with LiteSVM or Bankrun
- Deploying to devnet

## Getting started

```bash
# needs the Solana CLI and Anchor: https://www.anchor-lang.com/docs/installation
anchor build
anchor test
```

## Roadmap

Each open item is tracked as a [GitHub issue](https://github.com/zairithm/anchor-escrow/issues).

- [x] Anchor workspace and the `Escrow` account layout
- [ ] `make`: create the escrow PDA and deposit token A into a vault
- [ ] `take`: send token B to the maker and release token A
- [ ] `refund`: maker cancels and gets token A back
- [ ] Tests for the happy path and every failure case
- [ ] Deploy to devnet and add the program id to the README

## License

MIT
