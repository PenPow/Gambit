use std::collections::HashSet;
use gambit_models::moves::Move;
use gambit_movegen::generator::legal::generate;
use gambit_movegen::list::MoveList;
use gambit_movegen::state::State;
use crate::tt::TranspositionTable;

pub fn reconstruct_pv(state: &mut State, tt: &TranspositionTable, max_len: u32) -> Vec<Move> {
    let mut pv = Vec::new();
    let mut visited = HashSet::new();
    let mut undo_stack = Vec::new();

    for _ in 0..max_len {
        let hash = state.hash();
        if !visited.insert(hash) {
            break;
        }

        let Some(mv) = tt.probe_move(hash) else {
            break;
        };

        let mut list = MoveList::new();
        generate(state, &mut list);
        if !list.iter().any(|m| *m == mv) {
            break;
        }

        let undo = state.make_move(mv);
        undo_stack.push((mv, undo));
        pv.push(mv);
    }

    for (mv, undo) in undo_stack.into_iter().rev() {
        state.unmake_move(mv, undo);
    }

    pv
}