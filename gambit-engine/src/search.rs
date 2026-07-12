mod pv;
mod quiescence;
pub mod repetition;

use crate::MATE_THRESHOLD;
use crate::search::pv::reconstruct_pv;
use crate::search::quiescence::quiesce;
use crate::search::repetition::RepetitionTable;
use crate::time_controller::TimeController;
use crate::tt::{EntryType, TranspositionTable};
use gambit_models::location::square::Square;
use gambit_models::moves::Move;
use gambit_movegen::generator::legal::generate;
use gambit_movegen::list::MoveList;
use gambit_movegen::state::State;
use gambit_protocol::{GoParams, SearchInfo};
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub const MATE_VALUE: i32 = 30_000;
const INFINITY: i32 = MATE_VALUE + 1;
const NODE_TIME_CHECK_INTERVAL: u64 = 2048;

pub fn search(
    mut state: State,
    params: GoParams,
    stop_flag: Arc<AtomicBool>,
    tt: &mut TranspositionTable,
    mut history: RepetitionTable,
    mut info: impl FnMut(SearchInfo),
) -> Option<Move> {
    let time_ctrl = TimeController::new(&params, &state);
    tt.new_search();

    let mut max_depth = params.depth.unwrap_or(u32::MAX);
    if let Some(mate) = params.mate {
        max_depth = max_depth.min((2 * mate) - 1);
    }

    let search_moves: Option<HashSet<Square>> = params.search_moves.as_ref().map(|s| {
        s.split(',')
            .map(|lan| gambit_notation::lan::parse(lan.trim(), &state).unwrap())
            .map(|mv| mv.to())
            .collect()
    });

    let mut root_list = MoveList::new();
    generate(&state, &mut root_list);

    if root_list.is_empty() {
        return None;
    }

    let mut best_move: Option<Move> = root_list.iter().next().copied();

    let mut nodes: u64 = 0;
    let mut depth = 1;
    let mut seldepth: u32 = 0;

    while depth <= max_depth {
        if should_abort(nodes, &stop_flag, &time_ctrl) {
            break;
        }

        let mut alpha = -INFINITY;
        let beta = INFINITY;

        let mut best_score = -INFINITY;
        let mut best_mv: Option<Move> = None;
        let mut aborted = false;

        let tt_move = tt.probe_move(state.hash());

        for mv in ordered_moves(&root_list, tt_move) {
            if should_abort(nodes, &stop_flag, &time_ctrl) {
                aborted = true;
                break;
            }

            if let Some(allowed) = &search_moves {
                if !allowed.contains(&mv.to()) {
                    continue;
                }
            }

            let score = make_move_and_search(
                &mut state,
                tt,
                &mut history,
                mv,
                depth - 1,
                1,
                alpha,
                beta,
                &stop_flag,
                &time_ctrl,
                &mut nodes,
                &mut aborted,
                &mut seldepth,
            );

            if aborted {
                break;
            }

            if score > best_score {
                best_score = score;
                best_mv = Some(mv);
            }

            if score > alpha {
                alpha = score;
            }
        }

        if !aborted || best_mv.is_none() {
            if let Some(mv) = best_mv {
                best_move = Some(mv);

                tt.store(
                    state.hash(),
                    depth as u16,
                    0,
                    best_score,
                    mv,
                    EntryType::Exact,
                );

                let pv = reconstruct_pv(&mut state, tt, depth);

                info(SearchInfo {
                    depth: Some(depth),
                    nodes: Some(nodes),
                    time: Some(time_ctrl.elapsed_ms()),
                    score: Some(best_score),
                    hashfull: Some(tt.hashfull()),
                    seldepth: Some(seldepth),
                    pv: Some(pv),
                    ..Default::default()
                });

                if best_score.abs() >= MATE_THRESHOLD {
                    break;
                }
            }

            depth += 1;
        }

        if aborted || should_abort(nodes, &stop_flag, &time_ctrl) {
            break;
        }
    }

    best_move
}

#[allow(clippy::too_many_arguments)]
fn negamax(
    state: &mut State,
    tt: &mut TranspositionTable,
    history: &mut RepetitionTable,
    depth: u32,
    ply: u32,
    mut alpha: i32,
    beta: i32,
    stop_flag: &Arc<AtomicBool>,
    time_ctrl: &TimeController,
    nodes: &mut u64,
    aborted: &mut bool,
    seldepth: &mut u32,
) -> i32 {
    *nodes += 1;
    *seldepth = (*seldepth).max(ply);

    if *nodes % NODE_TIME_CHECK_INTERVAL == 0 && should_abort(*nodes, stop_flag, time_ctrl) {
        *aborted = true;
        return -INFINITY;
    }

    debug_assert!(
        depth <= u16::MAX as u32,
        "depth exceeds TT's u16 field width"
    );
    debug_assert!(ply <= u16::MAX as u32, "ply exceeds TT's u16 field width");

    let hash = state.hash();
    let halfmove_clock = state.position().halfmove_clock;

    if halfmove_clock.value() >= 4
        && (halfmove_clock.is_fifty_move_draw() || history.is_repeated(hash))
    {
        return 0;
    }

    if let Some((score, _)) = tt.lookup(hash, depth as u16, ply as u16, alpha, beta) {
        return score;
    }

    let original_alpha = alpha;

    let mut list = MoveList::new();
    generate(state, &mut list);

    if list.is_empty() {
        return if gambit_movegen::is_in_check(state, state.side_to_move()) {
            -(MATE_VALUE - ply as i32)
        } else {
            0
        };
    }

    if depth == 0 {
        return quiesce(
            state, tt, ply, alpha, beta, stop_flag, time_ctrl, nodes, aborted, seldepth,
        );
        // return evaluate(state)
    }

    let tt_move = tt.probe_move(hash);
    let mut best_score = -INFINITY;
    let mut best_move: Option<Move> = None;

    for mv in ordered_moves(&list, tt_move) {
        let score = make_move_and_search(
            state,
            tt,
            history,
            mv,
            depth - 1,
            ply + 1,
            alpha,
            beta,
            stop_flag,
            time_ctrl,
            nodes,
            aborted,
            seldepth,
        );

        if *aborted {
            return best_score;
        }

        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }

        if best_score > alpha {
            alpha = best_score;
        }

        if alpha >= beta {
            tt.store(
                hash,
                depth as u16,
                ply as u16,
                best_score,
                mv,
                EntryType::LowerBound,
            );

            return best_score;
        }
    }

    let best_move = best_move.expect("at least one move was searched");

    let entry_type = if best_score <= original_alpha {
        EntryType::UpperBound
    } else if best_score >= beta {
        EntryType::LowerBound
    } else {
        EntryType::Exact
    };

    tt.store(
        hash,
        depth as u16,
        ply as u16,
        best_score,
        best_move,
        entry_type,
    );

    best_score
}

#[allow(clippy::too_many_arguments)]
fn make_move_and_search(
    state: &mut State,
    tt: &mut TranspositionTable,
    history: &mut RepetitionTable,
    mv: Move,
    depth: u32,
    ply: u32,
    alpha: i32,
    beta: i32,
    stop_flag: &Arc<AtomicBool>,
    time_ctrl: &TimeController,
    nodes: &mut u64,
    aborted: &mut bool,
    seldepth: &mut u32,
) -> i32 {
    let undo = state.make_move(mv);
    let hash = state.hash();
    history.push(hash);

    let score = -negamax(
        state, tt, history, depth, ply, -beta, -alpha, stop_flag, time_ctrl, nodes, aborted,
        seldepth,
    );

    history.pop(hash);
    state.unmake_move(mv, undo);

    score
}

fn ordered_moves<'a>(list: &'a MoveList, tt_move: Option<Move>) -> impl Iterator<Item = Move> + 'a {
    let tt_move = tt_move.filter(|mv| list.iter().any(|m| m == mv));

    tt_move
        .into_iter()
        .chain(list.iter().copied().filter(move |mv| Some(*mv) != tt_move))
}

#[inline]
fn should_abort(nodes: u64, stop_flag: &Arc<AtomicBool>, time_ctrl: &TimeController) -> bool {
    stop_flag.load(Ordering::Relaxed)
        || (time_ctrl.has_time_limit() && time_ctrl.should_stop())
        || time_ctrl.should_stop_mid(nodes)
}
