<p align="center">
  <img src="/art/logo.svg" alt="Gambit" width="200" />
</p>
<h1 align="center">Gambit</h1>
<p align="center">
  A Rust-based ecosystem for building chess engines
</p>
<p align="center">
  <!-- <a href="https://github.com/PenPow/gambit/actions"><img src="https://github.com/PenPow/gambit/actions/workflows/ci.yml/badge.svg" alt="CI" /></a> -->
  <a href="https://crates.io/crates/gambit"><img src="https://img.shields.io/crates/v/gambit.svg" alt="crates.io" /></a>
  <a href="https://docs.rs/gambit"><img src="https://docs.rs/gambit/badge.svg" alt="docs.rs" /></a>
  <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg" alt="License" />
</p>

Gambit is an ecosystem of modular Rust crates for building chess engines. Each crate is independently usable, so you can pick just the pieces you need.

## Crates

| Crate | Version | Description |
|-------|---------|-------------|
| [`gambit`](./gambit) | [![crates.io](https://img.shields.io/crates/v/gambit.svg)](https://crates.io/crates/gambit) | Binary implementation of the Gambit engine |
| [`gambit-engine`](./gambit-engine) | [![crates.io](https://img.shields.io/crates/v/gambit-engine.svg)](https://crates.io/crates/gambit-engine) | Search, evaluation, transposition tables |
| [`gambit-models`](./gambit-models) | [![crates.io](https://img.shields.io/crates/v/gambit-models.svg)](https://crates.io/crates/gambit-models) | Core types: bitboards, squares, moves, pieces |
| [`gambit-movegen`](./gambit-movegen) | [![crates.io](https://img.shields.io/crates/v/gambit-movegen.svg)](https://crates.io/crates/gambit-movegen) | Move generation and attack tables |
| [`gambit-notation`](./gambit-notation) | [![crates.io](https://img.shields.io/crates/v/gambit-notation.svg)](https://crates.io/crates/gambit-notation) | Parsers for FEN, PGN, EPD, SAN |
| [`gambit-protocol`](./gambit-protocol) | [![crates.io](https://img.shields.io/crates/v/gambit-protocol.svg)](https://crates.io/crates/gambit-protocol) | Generic communication protocol layer |
| [`gambit-uci`](./gambit-uci) | [![crates.io](https://img.shields.io/crates/v/gambit-uci.svg)](https://crates.io/crates/gambit-uci) | UCI protocol implementation |

## License

Licensed under either of the following at your option:

- Apache License, Version 2.0 ([LICENCE-APACHE.md](LICENCE-APACHE.md) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENCE-MIT.md](LICENCE-MIT.md) or <http://opensource.org/licenses/MIT>)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
