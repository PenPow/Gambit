use std::ops::RangeInclusive;

pub type Side = usize;
pub struct Sides {}
impl Sides {
	pub const WHITE: Side = 0;
	pub const BLACK: Side = 1;
}

pub type Piece = usize;
pub struct PieceType {}
impl PieceType {
	pub const PAWN: Piece = 0;
	pub const KNIGHT: Piece = 1;
	pub const BISHOP: Piece = 2;
	pub const ROOK: Piece = 3;
	pub const QUEEN: Piece = 4;
	pub const KING: Piece = 5;
	pub const NONE: Piece = 6;

	pub const ALL_PIECES_RANGE: RangeInclusive<usize> = PieceType::PAWN..=PieceType::KING;

	pub fn to_str(piece: Piece) -> &'static str {
		assert!(0 <= piece && piece < 7);

		match piece {
			0 => "Pawn",
			1 => "Knight",
			2 => "Bishop",
			3 => "Rook",
			4 => "Queen",
			5 => "King",
			6 => "None",
			_ => panic!("Unknown piece type")
		}
	}
}

pub enum Direction {
	North,
	NorthEast,
	East,
	SouthEast,
	South,
	SouthWest,
	West,
	NorthWest
}