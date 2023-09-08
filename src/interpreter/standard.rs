use crate::parser::command::Atom;

use super::{object::Object, value::Value};

macro_rules! assert_argument {
    ($argument:expr; is $type:expr) => {
        match $argument {
            Value::$type(..) => {}
            _ => panic!("Argument is not the type '{}'.", stringify($type)),
        }
    };

    ($argument:expr; in $range:expr) => {
        if !$range.contains($argument.len()) {
            panic!(
                "The argument count is not between {} to {}.",
                $range.start, $range.end
            );
        }
    };
}

pub fn greet<'code>(_context: &mut Object, _body: &[Atom<'code>]) -> Value<'code> {
    println!("Hello world");
    Value::NULL
}
