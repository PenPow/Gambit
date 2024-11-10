#![deny(clippy::all, clippy::pedantic, clippy::cargo)]

/// The current version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Re-export of gambit
pub use gambit as internal;