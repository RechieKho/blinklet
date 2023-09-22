use super::value::Value;

#[derive(Debug, Clone)]
pub enum Signal {
    COMPLETE(Value),
    RETURN(Value),
}
