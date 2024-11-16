#![deny(clippy::all, clippy::pedantic, clippy::cargo)]

/// The current version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
	let mut builder = gambit_engine::internal::board::moves::builder::MoveBuilder::new();

	builder.piece(gambit_engine::internal::piece::PieceType::None);

	dbg!(builder.to_move());
}