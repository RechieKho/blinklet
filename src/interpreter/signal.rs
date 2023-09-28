use crate::mark::Mark;
use std::sync::Arc;
use super::value::Value;

#[macro_export]
macro_rules! signal_no_loop_control {
    ($signal:expr) => {
        use crate::interpreter::signal::Signal;

        match $signal {
            Signal::BREAK(ref mark) | Signal::CONTINUE(ref mark) => {
                raise_error!(Some(mark.clone()), "Loop control structure is forbidden.");
            }
            _ => {}
        }
    };
}

#[derive(Debug, Clone)]
pub enum Signal {
    COMPLETE(Value),
    RETURN(Value, Arc<Mark>),
    BREAK(Arc<Mark>),
    CONTINUE(Arc<Mark>),
}
