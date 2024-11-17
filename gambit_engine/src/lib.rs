#![deny(clippy::all, clippy::pedantic, clippy::cargo)]

#![doc = include_str!(concat!("../", env!("CARGO_PKG_README")))]

/// The current version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Re-export of gambit
pub use gambit as internal;