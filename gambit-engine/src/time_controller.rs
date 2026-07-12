use gambit_models::piece::colour::Colour;
use gambit_movegen::state::State;
use gambit_protocol::GoParams;
use std::time::Instant;

pub struct TimeController {
    go: GoParams,
    start_time: Instant,
    allocated_ms: u64,
}

impl TimeController {
    pub fn new(go: &GoParams, state: &State) -> Self {
        let mut allocated_ms: u64 = u64::MAX;

        if let Some(mt) = go.move_time {
            allocated_ms = mt.as_millis() as u64;
        } else if go.nodes.is_some() || go.depth.is_some() || go.mate.is_some() || go.infinite {
            allocated_ms = u64::MAX;
        } else if let Some(time) = if state.side_to_move() == Colour::White {
            go.wtime
        } else {
            go.btime
        } {
            let inc = if state.side_to_move() == Colour::White {
                go.winc.unwrap_or_default()
            } else {
                go.binc.unwrap_or_default()
            };

            allocated_ms = match go.moves_to_go {
                Some(moves) => {
                    (time.as_millis() as u64 / moves as u64) + (inc.as_millis() as u64 / 2)
                }
                None => (time.as_millis() as u64 / 10) + (inc.as_millis() as u64 / 2),
            };
        }

        Self {
            go: go.clone(),
            start_time: Instant::now(),
            allocated_ms,
        }
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }

    pub fn has_time_limit(&self) -> bool {
        self.allocated_ms != u64::MAX
    }

    #[inline]
    pub fn should_stop(&self) -> bool {
        if self.allocated_ms == u64::MAX {
            return false;
        }

        self.elapsed_ms() >= self.allocated_ms
    }

    pub fn should_stop_mid(&self, nodes_searched: u64) -> bool {
        self.go.nodes.is_some_and(|limit| nodes_searched >= limit)
    }
}
