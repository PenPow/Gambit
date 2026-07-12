use gambit_movegen::generator::legal::generate;
use gambit_movegen::list::MoveList;
use gambit_movegen::state::State;
use gambit_notation::fen::FenLike;
use gambit_notation::fen::parsers::Fen;

pub struct TestCase {
    pub name: &'static str,
    pub fen: &'static str,
    pub depths: &'static [(u32, u64)],
}

pub fn perft(state: &mut State, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut list = MoveList::new();
    generate(state, &mut list);

    if depth == 1 {
        return list.len() as u64;
    }

    let mut nodes = 0;
    for &mv in list.iter() {
        let undo = state.make_move(mv);
        nodes += perft(state, depth - 1);
        state.unmake_move(mv, undo);
    }

    nodes
}

pub fn state_from_fen(fen: &str) -> State {
    let fen = Fen::parse(fen).expect("expected valid FEN");
    State::from_position(fen.position())
}

macro_rules! perft_tests {
    ($($name:ident: $fen:expr, [$($depth:expr => $expected:expr),+ $(,)?];)+) => {
        pub const KNOWN_POSITIONS: &[TestCase] = &[
            $(
                TestCase {
                    name: stringify!($name),
                    fen: $fen,
                    depths: &[$(($depth, $expected)),+],
                },
            )+
        ];
    };
}

perft_tests! {
    startpos: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", [
        1 => 20,
        2 => 400,
        3 => 8902,
        4 => 197281,
        5 => 4865609,
        6 => 119060324,
        7 => 3195901860,
        8 => 84998978956,
        9 => 2439530234167
    ];

    kiwipete: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", [
        1 => 48,
        2 => 2039,
        3 => 97862,
        4 => 4085603,
        5 => 193690690,
        6 => 8031647685
    ];

    cpw_position_3: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", [
        1 => 14,
        2 => 191,
        3 => 2812,
        4 => 43238,
        5 => 674624,
        6 => 11030083,
        7 => 178633661,
        8 => 3009794393
    ];

    cpw_position_4: "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", [
        1 => 6,
        2 => 264,
        3 => 9467,
        4 => 422333,
        5 => 15833292,
        6 => 706045033
    ];

    cpw_position_4_mirrored: "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1", [
        1 => 6,
        2 => 264,
        3 => 9467,
        4 => 422333,
        5 => 15833292,
        6 => 706045033
    ];

    cpw_position_5: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", [
        1 => 44,
        2 => 1486,
        3 => 62379,
        4 => 2103487,
        5 => 89941194
    ];

    cpw_position_6: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ", [
        1 => 46,
        2 => 2079,
        3 => 89890,
        4 => 3894594,
        5 => 164075551,
        6 => 6923051137,
        7 => 287188994746,
        // Unreasonably long
        // 8 => 11923589843526,
        // 9 => 490154852788714
    ];
}
