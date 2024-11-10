use super::location::Squares;

pub type CastlingAvailability = u8;
pub struct CastlingPermissions;
impl CastlingPermissions {
	pub const WHITE_KING: CastlingAvailability = 1;
	pub const WHITE_QUEEN: CastlingAvailability = 2;
	pub const BLACK_KING: CastlingAvailability = 4;
	pub const BLACK_QUEEN: CastlingAvailability = 8;

	pub const ALL: CastlingAvailability = Self::WHITE_KING | Self::WHITE_QUEEN | Self::BLACK_KING | Self::BLACK_QUEEN;
	pub const NONE: CastlingAvailability = 0;

	pub const PER_SQUARE: [CastlingAvailability; Squares::COUNT] = {
		let mut permissions = [Self::ALL; Squares::COUNT];

		permissions[Squares::A1] &= !Self::WHITE_QUEEN;
		permissions[Squares::E1] &= !Self::WHITE_KING & !Self::WHITE_QUEEN;
		permissions[Squares::H1] &= !Self::WHITE_KING;

		permissions[Squares::A8] &= !Self::BLACK_QUEEN;
		permissions[Squares::E8] &= !Self::BLACK_KING & !Self::BLACK_QUEEN;
		permissions[Squares::H8] &= !Self::BLACK_KING;

		permissions
	};
	
	pub const COUNT: usize = 16;
}