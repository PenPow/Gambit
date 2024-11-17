use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::fs::File;
use std::io::Write;

fn strip_indent(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    let min_indent = lines.iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0);

    lines.iter()
        .map(|line| {
            if line.trim().is_empty() {
                line.to_string()
            } else {
                line[min_indent..].to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn generate_zobrist_randoms() {
	// WARNING: DO NOT CHANGE
	let seed = 0xdb3df068f3f1cd48;

    let mut rng = Pcg64Mcg::seed_from_u64(seed);
    let mut piece_randoms = [[[0u64; 64]; 6]; 2]; // Adjust the size as needed

	#[allow(clippy::needless_range_loop)]
    for i in 0..2 {
        for j in 0..6 {
            for k in 0..64 {
				piece_randoms[i][j][k] = rng.gen();
			}
        }
    }

	let mut en_passant_randoms = [0u64; 8];

	#[allow(clippy::needless_range_loop)]
	for i in 0..8 {
		en_passant_randoms[i] = rng.gen();
	}

	let mut castling_randoms = [0u64; 16];

	#[allow(clippy::needless_range_loop)]
	for i in 0..16 {
		castling_randoms[i] = rng.gen();
	}

	let side_random: u64 = rng.gen();

    let mut file = File::create("src/board/zobrist/generated_randoms.rs").unwrap();

	let str = format!(r#"
		use crate::{{location::{{Square, File}}, piece::{{Colour, PieceType, CastlingPermissions}}}};
		
		/// A three dimensional array containing a [`ZobristRandom`] for each [`Square`], for each [`PieceType`] for each [`Colour`] to use when calculating piece positions.
		pub const ZOBRIST_PIECE_RANDOMS: [[[ZobristRandom; Square::COUNT]; PieceType::COUNT]; Colour::COUNT] = {piece_randoms:?};
		
		/// An array containing a [`ZobristRandom`] for each [`File`] to use when calculating the en passant files
		pub const ZOBRIST_EN_PASSANT_RANDOMS: [ZobristRandom; File::COUNT] = {en_passant_randoms:?};
		
		/// An array containing a [`ZobristRandom`] for each possible combination of [`CastlingPermissions`] to use when calculating updates to the castling availability
		pub const ZOBRIST_CASTLING_RANDOMS: [ZobristRandom; CastlingPermissions::COUNT] = {castling_randoms:?};
		
		/// A [`ZobristRandom`] to use when changing the side to move
		pub const ZOBRIST_SIDE_RANDOM: ZobristRandom = {side_random:?};
	"#);
    
	writeln!(file, "{}", strip_indent(str.as_str()).trim()).unwrap();
}

fn main() {
    generate_zobrist_randoms();
}