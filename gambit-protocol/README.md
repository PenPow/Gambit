<p align="center">
  <img src="../art/logo.svg" alt="Gambit" width="200" />
</p>
<h1 align="center">gambit-protocol</h1>
<p align="center">
  A shared communication protocol for chess engines, designed to be frontend agnostic.
</p>
<p align="center">
  <a href="https://crates.io/crates/gambit-protocol"><img src="https://img.shields.io/crates/v/gambit-protocol.svg" alt="crates.io" /></a>
  <a href="https://docs.rs/gambit-protocol"><img src="https://docs.rs/gambit-protocol/badge.svg" alt="docs.rs" /></a>
  <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg" alt="License" />
</p>

`gambit-protocol` provides a common communication protocol to send commands and receive events with your chess engine.

This allows for protocol layering, letting you switch out the frontend with any compatible communication protocol you like
(such as UCI, XBoard or Lichess).

## License

Licensed under either of the following at your option:

- Apache License, Version 2.0 ([LICENCE-APACHE.md](../LICENCE-APACHE.md) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENCE-MIT.md](../LICENCE-MIT.md) or <http://opensource.org/licenses/MIT>)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
