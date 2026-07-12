use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    magic::write_magic_file(out_dir);
    pext::write_pext_file(out_dir);
    lines::write_line_file(out_dir);
}

mod common {
    use gambit_models::bitboard::Bitboard;
    use gambit_models::location::square::Square;

    pub const ROOK_DIRS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    pub const BISHOP_DIRS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    #[derive(Debug, Clone)]
    pub struct PackedTable {
        pub entries_src: String,
        pub attacks_src: String,
        pub total_size: usize,
    }

    pub fn sliding_mask(square: Square, directions: &[(i8, i8)]) -> Bitboard {
        let (file, rank) = square.coordinates();

        let mut mask = Bitboard::EMPTY;

        for &(df, dr) in directions {
            let mut current_file = file.offset(df);
            let mut current_rank = rank.offset(dr);

            while let (Some(f), Some(r)) = (current_file, current_rank) {
                let next_file = f.offset(df);
                let next_rank = r.offset(dr);

                if next_file.is_some() && next_rank.is_some() {
                    mask |= Square::from_coordinates((f, r));
                }

                current_file = next_file;
                current_rank = next_rank;
            }
        }

        mask
    }

    pub fn sliding_attacks(
        square: Square,
        occupied: Bitboard,
        directions: &[(i8, i8)],
    ) -> Bitboard {
        let (file, rank) = square.coordinates();

        let mut attacks = Bitboard::EMPTY;

        for &(df, dr) in directions {
            let mut current_file = file.offset(df);
            let mut current_rank = rank.offset(dr);

            while let (Some(f), Some(r)) = (current_file, current_rank) {
                let target = Square::from_coordinates((f, r));
                attacks |= target;

                if occupied.contains(target) {
                    break;
                }

                current_file = f.offset(df);
                current_rank = r.offset(dr);
            }
        }

        attacks
    }
}

mod magic {
    use crate::common;
    use gambit_models::location::map::square::SquareMap;
    use gambit_models::location::square::Square;
    use std::fmt::Write;
    use std::fs;
    use std::path::Path;

    const ROOK_MAGICS: SquareMap<u64> = SquareMap::from_array([
        324259448050975248u64,
        162139001189302336u64,
        4647750006529359880u64,
        144121785691422736u64,
        16176938657641660544u64,
        9367489423970945072u64,
        36051338366288384u64,
        36029147746665088u64,
        3518447965192208u64,
        4614078830617822340u64,
        9241949523864129664u64,
        11540615780106252u64,
        730287067600519297u64,
        144819425575437312u64,
        1225261127674627584u64,
        40814017656160512u64,
        594475700577118276u64,
        283675082228259u64,
        148058037853261952u64,
        14411662294658320384u64,
        2394186703782912u64,
        1157847866488718336u64,
        2306407062973841412u64,
        4576167411597460u64,
        2323857959626489888u64,
        18860477004136448u64,
        621497027752297522u64,
        3027553647748714496u64,
        9241953785514295424u64,
        1970363492082688u64,
        1729664285938024960u64,
        4836870457972064321u64,
        141012374650913u64,
        4652253601601699840u64,
        58687601506263040u64,
        281543780081672u64,
        1157433900411130112u64,
        81628378934806544u64,
        2310366730829959192u64,
        2900476768907429780u64,
        36558770110480u64,
        9042384969023488u64,
        180425597514743824u64,
        5487636764434923528u64,
        5766860422494879764u64,
        9224498487624761348u64,
        41702298761822218u64,
        45599234000551940u64,
        70370891935872u64,
        19210671497487104u64,
        387030266675328u64,
        289215847808893056u64,
        576469550545240192u64,
        1153216449143113729u64,
        9350715278336u64,
        288521763922764288u64,
        282782794268833u64,
        595672521157161122u64,
        436884352794689609u64,
        9241667927690743809u64,
        5188428314494240769u64,
        1157988067282792450u64,
        1152939243166828548u64,
        4611967569673330817u64,
    ]);
    const BISHOP_MAGICS: SquareMap<u64> = SquareMap::from_array([
        2310454429704290569u64,
        37163502750244928u64,
        145330200115150856u64,
        573953659699200u64,
        9845999220824211456u64,
        574016004032512u64,
        10093699283674480640u64,
        2306407060834902016u64,
        2883575003184432136u64,
        1747410678824308864u64,
        9259405249167245312u64,
        936784527773139074u64,
        4629702641998381057u64,
        201028145628315697u64,
        4899992295377881088u64,
        4630405483133404688u64,
        153474299838154784u64,
        2286992943744036u64,
        434597432802681416u64,
        865817269052115456u64,
        9156750026475656u64,
        599823317909770240u64,
        4578375142474880u64,
        2308525819264500224u64,
        18596057879421451u64,
        18331093560345096u64,
        2305880392877736000u64,
        56602859688444160u64,
        5382084129205534724u64,
        5767422822691897608u64,
        283691220206592u64,
        144398865845093376u64,
        1163523824685120u64,
        20267333288223264u64,
        325489801822240u64,
        4755836425302245636u64,
        594475563668865152u64,
        1162496335329427604u64,
        9244765235704371236u64,
        576667461564269056u64,
        146371454722771202u64,
        426679365288452u64,
        13724105480340736u64,
        1152922330050364928u64,
        4620737202526097424u64,
        1316476062695166464u64,
        13981996823661781640u64,
        12430506881068303489u64,
        5193780677221351424u64,
        426612797737280u64,
        37445932288049152u64,
        1171147012042137601u64,
        504403227018657856u64,
        4629845569785954560u64,
        4686013077882208273u64,
        1154056209263894528u64,
        613054853085794304u64,
        9025075185721408u64,
        9571249324951568u64,
        10999715432448u64,
        290408795603472u64,
        10664524198170591488u64,
        5924513492108288u64,
        90511840181764112u64,
    ]);

    fn build_magic_table(
        directions: &[(i8, i8)],
        magics: &SquareMap<u64>,
        label: &str,
    ) -> common::PackedTable {
        let mut entries_src = String::new();
        let mut attacks_src = String::new();

        let mut offset: u32 = 0;

        for square in Square::ALL {
            let mask = common::sliding_mask(square, directions);

            let bits = mask.bits().count_ones();
            let shift = 64 - bits;
            let magic = magics[square];
            let size = 1usize << bits;

            let mut slot = vec![None; size];

            for occupancy in mask.carry_rippler() {
                let attack = common::sliding_attacks(square, occupancy, directions);
                let index = (occupancy.bits().wrapping_mul(magic) >> shift) as usize;

                match slot[index] {
                    Some(existing) if existing != attack => panic!(
                        "{label} magic collision on square {square} at index {index}: both {existing:#018x} and {attack:#018x}. This magic is invalid."
                    ),
                    _ => slot[index] = Some(attack),
                }
            }

            for entry in slot {
                let attack = entry.unwrap().bits();
                write!(attacks_src, "Bitboard::new({attack:#018x}), ").unwrap()
            }

            write!(
                entries_src,
                "Magic {{ mask: {mask:#018x}, magic: {magic:#018x}, shift: {shift}, offset: {offset} }}, "
            ).unwrap();

            offset += size as u32;
        }

        common::PackedTable {
            entries_src,
            attacks_src,
            total_size: offset as usize,
        }
    }

    pub fn write_magic_file(out_dir: &Path) {
        let rook = build_magic_table(&common::ROOK_DIRS, &ROOK_MAGICS, "rook");
        let bishop = build_magic_table(&common::BISHOP_DIRS, &BISHOP_MAGICS, "bishop");

        let src = format!(
            "pub(crate) static ROOK_MAGICS: SquareMap<Magic> = SquareMap::from_array([{}]);\n\
            pub(crate) static ROOK_ATTACKS: [Bitboard; {}] = [{}];\n\
            pub(crate) static BISHOP_MAGICS: SquareMap<Magic> = SquareMap::from_array([{}]);\n\
            pub(crate) static BISHOP_ATTACKS: [Bitboard; {}] = [{}];\n",
            rook.entries_src,
            rook.total_size,
            rook.attacks_src,
            bishop.entries_src,
            bishop.total_size,
            bishop.attacks_src,
        );

        fs::write(out_dir.join("magic_tables.rs"), src).unwrap();
    }
}

mod pext {
    use crate::common;
    use gambit_models::location::square::Square;
    use std::fmt::Write;
    use std::fs;
    use std::path::Path;

    fn software_pext(src: u64, mask: u64) -> u64 {
        let mut result = 0u64;
        let mut bit = 1u64;
        let mut m = mask;

        while m != 0 {
            let lowest = m & m.wrapping_neg();
            if src & lowest != 0 {
                result |= bit;
            }
            m &= m - 1;
            bit <<= 1;
        }

        result
    }

    fn build_pext_table(directions: &[(i8, i8)]) -> common::PackedTable {
        let mut entries_src = String::new();
        let mut attacks_src = String::new();

        let mut offset: u32 = 0;

        for square in Square::ALL {
            let mask = common::sliding_mask(square, directions);

            let bits = mask.bits().count_ones();
            let size = 1usize << bits;

            let mut slot = vec![None; size];

            for occupancy in mask.carry_rippler() {
                let attack = common::sliding_attacks(square, occupancy, directions);
                let index = software_pext(occupancy.bits(), mask.bits()) as usize;

                slot[index] = Some(attack);
            }

            for entry in slot {
                let attack = entry.unwrap().bits();
                write!(attacks_src, "Bitboard::new({attack:#018x}), ").unwrap();
            }

            write!(
                entries_src,
                "PextEntry {{ mask: {mask:#018x}, offset: {offset} }}, "
            )
            .unwrap();

            offset += size as u32;
        }

        common::PackedTable {
            entries_src,
            attacks_src,
            total_size: offset as usize,
        }
    }

    pub fn write_pext_file(out_dir: &Path) {
        let rook = build_pext_table(&common::ROOK_DIRS);
        let bishop = build_pext_table(&common::BISHOP_DIRS);

        let src = format!(
            "pub(crate) static ROOK_PEXT: SquareMap<PextEntry> = SquareMap::from_array([{}]);\n\
            pub(crate) static ROOK_PEXT_ATTACKS: [Bitboard; {}] = [{}];\n\
            pub(crate) static BISHOP_PEXT: SquareMap<PextEntry> = SquareMap::from_array([{}]);\n\
            pub(crate) static BISHOP_PEXT_ATTACKS: [Bitboard; {}] = [{}];\n",
            rook.entries_src,
            rook.total_size,
            rook.attacks_src,
            bishop.entries_src,
            bishop.total_size,
            bishop.attacks_src,
        );

        fs::write(out_dir.join("pext_tables.rs"), src).unwrap();
    }
}

mod lines {
    use gambit_models::bitboard::Bitboard;
    use gambit_models::location::square::Square;
    use std::fmt::Write;
    use std::fs;
    use std::path::Path;

    const DIRECTIONS: [(i8, i8); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    fn cast_ray(start: Square, direction: (i8, i8)) -> Vec<Square> {
        let mut squares = Vec::new();

        let (file, rank) = start.coordinates();
        let mut current_file = file.offset(direction.0);
        let mut current_rank = rank.offset(direction.1);

        while let (Some(f), Some(r)) = (current_file, current_rank) {
            squares.push(Square::from_coordinates((f, r)));
            current_file = f.offset(direction.0);
            current_rank = r.offset(direction.1);
        }

        squares
    }

    pub fn write_line_file(out_dir: &Path) {
        let mut between_src = String::new();
        let mut line_src = String::new();

        for a in Square::ALL {
            let mut between_row = String::new();
            let mut line_row = String::new();

            for b in Square::ALL {
                let mut between = Bitboard::EMPTY;
                let mut line = Bitboard::EMPTY;

                if a != b {
                    for direction in DIRECTIONS {
                        let forward = cast_ray(a, direction);

                        // found correct direction
                        if let Some(pos) = forward.iter().position(|&s| s == b) {
                            for &sq in &forward[..pos] {
                                between |= sq;
                            }

                            let backward = cast_ray(a, (-direction.0, -direction.1));

                            line |= a;

                            for sq in forward {
                                line |= sq;
                            }
                            for sq in backward {
                                line |= sq;
                            }

                            // no other line
                            break;
                        }
                    }
                }

                write!(between_row, "Bitboard::new({:#018x}), ", between.bits()).unwrap();
                write!(line_row, "Bitboard::new({:#018x}), ", line.bits()).unwrap();
            }

            write!(between_src, "SquareMap::from_array([{between_row}]), ").unwrap();
            write!(line_src, "SquareMap::from_array([{line_row}]), ").unwrap();
        }

        let src = format!(
            "pub(crate) static BETWEEN: SquareMap<SquareMap<Bitboard>> = SquareMap::from_array([{between_src}]);\n\
             pub(crate) static LINE_THROUGH: SquareMap<SquareMap<Bitboard>> = SquareMap::from_array([{line_src}]);\n"
        );

        fs::write(out_dir.join("line_tables.rs"), src).unwrap();
    }
}
