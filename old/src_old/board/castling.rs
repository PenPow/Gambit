pub type Castling = u8;
pub struct CastlingPermissions {}
impl CastlingPermissions {
	pub const WHITE_KING: Castling = 1;
	pub const WHITE_QUEEN: Castling = 2;
	pub const BLACK_KING: Castling = 4;
	pub const BLACK_QUEEN: Castling = 8;
}