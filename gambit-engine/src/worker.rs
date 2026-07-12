use crate::search::search;
use crate::tt::TranspositionTable;
use gambit_models::position::Position;
use gambit_movegen::state::State;
use gambit_notation::lan;
use gambit_protocol::Event::BestMove;
use gambit_protocol::{Command, EngineOption, Event, GoParams, Handshake, SetEnginePosition};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use crate::search::repetition::RepetitionTable;

pub struct Worker {
    commands: Receiver<Command>,
    events: Sender<Event>,
    state: State,
    stop_flag: Arc<AtomicBool>,
    tt: Arc<Mutex<TranspositionTable>>,
    history: RepetitionTable,
    search_thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(commands: Receiver<Command>, events: Sender<Event>) -> Self {
        let state = State::from_position(Position::STARTING_POSITION);
        let mut history = RepetitionTable::new();

        history.push(state.hash());

        Self {
            commands,
            events,
            state,
            stop_flag: Arc::new(AtomicBool::new(false)),
            tt: Arc::new(Mutex::new(TranspositionTable::new())),
            history,
            search_thread: None,
        }
    }

    pub fn run(&mut self) {
        while let Ok(cmd) = self.commands.recv() {
            match cmd {
                Command::Handshake => self.handshake(),
                Command::IsReady => self.send_event(Event::ReadyOk),
                // TODO: Options
                Command::SetOption(_option) => {}
                Command::NewGame => {
                    self.join_search();
                    self.state = State::from_position(Position::STARTING_POSITION);

                    self.clear_tt();
                    self.history = RepetitionTable::new();
                    self.history.push(self.state.hash());
                }
                Command::SetPosition(params) => self.apply_position(params),
                Command::Go(params) => self.start_search(params),
                Command::Stop => self.set_stop(true),
                // TODO: Pondering
                Command::PonderHit => {}
                // TODO: Debug mode
                Command::Debug(_) => {}
                Command::Quit => {
                    self.set_stop(true);
                    self.join_search();

                    break;
                }
            }
        }

        self.set_stop(true);
        self.join_search();
    }

    fn send_event(&self, event: Event) {
        self.events
            .send(event)
            .expect("expected event to be sent correctly, but sending event failed")
    }

    fn handshake(&self) {
        let handshake = Handshake {
            name: "Gambit".to_string(),
            author: "Joshua Clements".to_string(),
            options: Vec::<EngineOption>::new(),
        };

        self.send_event(Event::Handshake(handshake))
    }

    fn apply_position(&mut self, params: SetEnginePosition) {
        let mut state = State::from_position(params.position);
        let mut hashes = vec![state.hash()];

        for mv in params.moves {
            let mv = lan::parse(&mv, &state).expect("expected valid LAN move");

            // discard must_use as these moves will not be unmade
            let _ = state.make_move(mv);
            hashes.push(state.hash());
        }

        self.state = state;
        self.history = RepetitionTable::from_history(hashes);
    }

    fn start_search(&mut self, params: GoParams) {
        self.join_search();
        self.set_stop(false);

        let state = self.state;
        let stop_flag = Arc::clone(&self.stop_flag);
        let history = self.history.clone();
        let tt = Arc::clone(&self.tt);
        let events = self.events.clone();

        self.search_thread = Some(
            thread::Builder::new()
                .name("gambit-search".to_string())
                .spawn(move || {
                    let mut tt_guard = tt.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

                    let info_events = events.clone();

                    let best_move = search(state, params, stop_flag, &mut tt_guard, history, |info| {
                        let _ = info_events.send(Event::Info(info));
                    });

                    events.send(BestMove(best_move)).unwrap();
                })
                .expect("failed to spawn gambit-search thread"),
        )
    }

    fn set_stop(&mut self, value: bool) {
        self.stop_flag.store(value, Ordering::Relaxed);
    }

    fn join_search(&mut self) {
        if let Some(handle) = self.search_thread.take() {
            handle
                .join()
                .expect("expected to be able to join search thread but couldn't");
        }
    }

    fn clear_tt(&mut self) {
        self.tt
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clear();
    }
}
