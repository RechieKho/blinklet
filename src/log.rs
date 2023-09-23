use super::mark::Mark;
use std::{fmt::Display, rc::Rc};

#[derive(Debug, Clone)]
pub enum LogMessage {
    TRACE,
    INFO(String),
    WARNING(String),
    ERROR(String),
}

#[derive(Debug, Clone)]
pub struct Log {
    pub message: LogMessage,
    pub mark: Option<Rc<Mark>>,
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = match self.message {
            LogMessage::TRACE => format!("➡️  Traceback: \n"),
            LogMessage::INFO(ref info) => format!("🗨 Info: {}\n", info.clone()),
            LogMessage::WARNING(ref warning) => format!("⚠️  Warning: {}\n", warning.clone()),
            LogMessage::ERROR(ref error) => format!("⛔ Error: {}\n", error.clone()),
        };
        if self.mark.is_none() {
            f.pad(&header)
        } else {
            let mark = self.mark.clone().unwrap();
            let rendering = format!("{header}\n{}\n", mark);
            f.pad(&rendering)
        }
    }
}

impl Log {
    pub fn trace(mark: Option<Rc<Mark>>) -> Log {
        Log {
            message: LogMessage::TRACE,
            mark,
        }
    }

    pub fn info(message: String, mark: Option<Rc<Mark>>) -> Log {
        Log {
            message: LogMessage::INFO(message),
            mark,
        }
    }

    pub fn warning(message: String, mark: Option<Rc<Mark>>) -> Log {
        Log {
            message: LogMessage::WARNING(message),
            mark,
        }
    }

    pub fn error(message: String, mark: Option<Rc<Mark>>) -> Log {
        Log {
            message: LogMessage::ERROR(message),
            mark,
        }
    }
}
