use crate::mark::Mark;
use std::rc::Rc;
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
    RETURN(Value, Rc<Mark>),
    BREAK(Rc<Mark>),
    CONTINUE(Rc<Mark>),
}
