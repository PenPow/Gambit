use crate::format::format_event;
use crate::parse::parse_line;
use gambit_protocol::{Command, EngineHandle};
use std::io::{BufRead, Write};
use std::{io, thread};

pub fn run(handle: EngineHandle) {
    let EngineHandle { commands, events } = handle;

    let writer = thread::Builder::new()
        .name("gambit-uci-writer".to_string())
        .spawn(move || {
            let stdout = io::stdout();
            let mut out = stdout.lock();

            while let Ok(event) = events.recv() {
                for line in format_event(&event) {
                    let _ = writeln!(out, "{line}");
                }
                let _ = out.flush();
            }
        })
        .expect("failed to spawn gambit-uci writer thread");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };

        match parse_line(&line) {
            Ok(Some(cmd)) => {
                let is_quit = matches!(cmd, Command::Quit);
                if commands.send(cmd).is_err() {
                    break;
                }

                if is_quit {
                    break;
                }
            }
            Ok(None) => {}
            Err(err) => eprintln!("info string parse error: {err}"),
        }
    }

    drop(commands);
    let _ = writer.join();
}
