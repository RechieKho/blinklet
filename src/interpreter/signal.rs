use super::value::Value;

pub enum Signal {
    COMPLETE(Value),
    RETURN(Value),
}
