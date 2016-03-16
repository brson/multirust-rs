use std::time::{Duration, SystemTime};
use std::path::PathBuf;
use std::cell::RefCell;

impl Telemeter {
    pub fn new(logdir: PathBuf) -> Telemeter {
        Telemeter {
            logdir: logdir,
            events: RefCell::new(Vec::new()),
        }
    }

    pub fn log(&self, deets: EventDetails) {
        self.events.borrow_mut().push(Event {
            version: 0,
            time: SystemTime::now(),
            details: deets,
        });
    }
}

#[derive(Debug)]
pub enum EventDetails {
    InitialInstall,
    RustcRun(Duration, i32),
    CargoRun(Duration, i32),
    RustcErrorCode(String),
    ComponentDownload {
        name: String,
        target: String,
        version: String,
    }
}

#[derive(Debug)]
struct Event {
    version: u8,
    time: SystemTime,
    details: EventDetails,
}

#[derive(Debug)]
pub struct Telemeter {
    logdir: PathBuf,
    events: RefCell<Vec<Event>>,
}

impl Drop for Telemeter {
    fn drop(&mut self) {
        // TODO
    }
}

