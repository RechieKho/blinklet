use super::value::Value;

pub enum Signal<'name, 'code> {
    COMPLETE(Value<'name, 'code>),
    RETURN(Value<'name, 'code>),
}
