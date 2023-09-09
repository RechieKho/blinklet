use crate::parser::command::Atom;

use super::{object::Object, value::Value};

macro_rules! assert_argument {
    ($argument:expr; is $type:pat) => {
        if !matches!($argument, $type) { panic!("The argument doesn't match the type '{}'.", stringify!($type)); }
    };

    ($argument:expr; in $range:expr) => {
        if !$range.contains(&$argument.len()) {
            panic!(
                "The argument count is not between {} to {}.",
                $range.start, $range.end
            );
        }
    };
}

pub fn greet<'code>(_context: &mut Object, _body: &[Atom<'code>]) -> Value<'code> {
    println!("Hello world");
    assert_argument!(_body; in 2..3);
    //assert_argument!(_body[1]; is Atom::STRING(_, _));
    Value::NULL
}
