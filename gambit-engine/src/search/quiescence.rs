use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use gambit_movegen::generator::legal::{generate, generate_captures};
use gambit_movegen::list::MoveList;
use gambit_movegen::state::State;
use crate::eval::evaluate;
use crate::MATE_VALUE;
use crate::search::{should_abort, INFINITY, NODE_TIME_CHECK_INTERVAL};
use crate::time_controller::TimeController;
use crate::tt::TranspositionTable;

#[allow(clippy::too_many_arguments)]
pub fn quiesce(
    state: &mut State,
    tt: &mut TranspositionTable,
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

    let in_check = gambit_movegen::is_in_check(state, state.side_to_move());
    let stand_pat = evaluate(state);

    if !in_check {
        if stand_pat >= beta {
            return stand_pat;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }
    }

    let mut list = MoveList::new();
    if in_check {
        generate(state, &mut list);
    } else {
        generate_captures(state, &mut list);
    }

    if list.is_empty() {
        return if in_check { -(MATE_VALUE - ply as i32) } else { stand_pat };
    }

    let mut best_score = if in_check { -INFINITY } else { stand_pat };

    for mv in list.iter().copied() {
        let undo = state.make_move(mv);
        let score = -quiesce(
            state, tt, ply + 1, -beta, -alpha,
            stop_flag, time_ctrl, nodes, aborted, seldepth,
        );
        state.unmake_move(mv, undo);

        if *aborted {
            return best_score;
        }

        if score > best_score {
            best_score = score;
        }

        if best_score > alpha {
            alpha = best_score;
        }
        
        if alpha >= beta {
            break;
        }
    }

    best_score
}
