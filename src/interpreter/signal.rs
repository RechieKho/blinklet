use super::value::Value;

pub enum Signal<'code> {
    COMPLETE(Value<'code>),
    RETURN(Value<'code>),
}
