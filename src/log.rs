use super::mark::Mark;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum LogMessage {
    TRACE,
    ERROR(String),
    BUG(String),
}

#[derive(Debug, Clone)]
pub struct Log {
    pub message: LogMessage,
    pub mark: Option<Mark>,
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = match self.message {
            LogMessage::TRACE => format!("Traceback: \n"),
            LogMessage::ERROR(ref error) => format!("Error: {}\n", error.clone()),
            LogMessage::BUG(ref bug) => format!(
                "Internal Bug: {} (Please report to https://github.com/RechieKho/blinklet/issues/new)\n",
                bug.clone()
            ),
        };
        if self.mark.is_none() {
            f.write_str(&header)
        } else {
            let mark = self.mark.clone().unwrap();
            let rendering = format!("{header}\n{}\n", mark);
            f.write_str(&rendering)
        }
    }
}

impl Log {
    pub fn trace(mark: Mark) -> Log {
        Log {
            message: LogMessage::TRACE,
            mark: Some(mark),
        }
    }

    pub fn error(message: String, mark: Option<Mark>) -> Log {
        Log {
            message: LogMessage::ERROR(message),
            mark,
        }
    }

    pub fn bug(message: String, mark: Option<Mark>) -> Log {
        Log {
            message: LogMessage::BUG(message),
            mark,
        }
    }
}
