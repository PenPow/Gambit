use crate::board::{square::Squares, castling::CastlingPermissions};

pub const CASTLING_PERMISSIONS: [u8; 64] = generate_castling_permissions_per_square();

const fn generate_castling_permissions_per_square() -> [u8; 64] {
	let mut permissions = [15u8; 64];

	permissions[Squares::A1] &= !CastlingPermissions::WHITE_QUEEN;
	permissions[Squares::E1] &= !CastlingPermissions::WHITE_KING & !CastlingPermissions::WHITE_QUEEN;
	permissions[Squares::H1] &= !CastlingPermissions::WHITE_KING;
	permissions[Squares::A8] &= !CastlingPermissions::BLACK_QUEEN;
	permissions[Squares::E8] &= !CastlingPermissions::BLACK_KING & !CastlingPermissions::BLACK_QUEEN;
	permissions[Squares::H8] &= !CastlingPermissions::BLACK_KING;

	permissions
}