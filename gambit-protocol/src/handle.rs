use crate::command::Command;
use crate::event::Event;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub struct EngineHandle {
    pub commands: Sender<Command>,
    pub events: Receiver<Event>,
}
