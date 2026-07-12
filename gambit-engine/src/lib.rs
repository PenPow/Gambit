mod eval;
mod search;
mod time_controller;
mod tt;
mod worker;

use gambit_protocol::EngineHandle;
use std::sync::mpsc;
use std::thread;

pub use crate::search::MATE_VALUE;
pub use crate::tt::MATE_THRESHOLD;

pub fn spawn() -> EngineHandle {
    let (cmd_tx, cmd_rx) = mpsc::channel();
    let (evt_tx, evt_rx) = mpsc::channel();

    thread::Builder::new()
        .name("gambit-engine".into())
        .spawn(move || {
            worker::Worker::new(cmd_rx, evt_tx).run();
        })
        .expect("failed to spawn gambit-engine thread");

    EngineHandle {
        commands: cmd_tx,
        events: evt_rx,
    }
}
