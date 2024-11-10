#[derive(Debug)]
pub enum EngineToCommMessage {
	ReadyOk
}

#[derive(Debug)]
pub enum CommToEngineMessage {
	Quit,
	Debug,
	IsReady
}