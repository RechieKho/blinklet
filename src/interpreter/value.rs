pub mod closure;
pub mod null;
pub mod represent;
pub mod scope;
pub mod table;

use self::null::Null;

use super::context::Context;
use super::signal::Signal;
use crate::backtrace::Backtrace;
use crate::parser::command::Atom;
use closure::Closure;
use represent::Represent;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;
use table::Table;

#[macro_export]
macro_rules! mutex_lock_unwrap {
    ($mutex:expr, $mark:expr) => {
        match $mutex.lock() {
            Ok(guard) => guard,
            Err(_) => {
                crate::raise_bug!($mark, "Thread is poisoned while locking mutex.");
            }
        }
    };
}

#[derive(Clone)]
pub enum Value {
    NULL(Null),
    BOOL(bool),
    NUMBER(f64),
    STRING(String),
    LIST(Vec<Value>),
    TABLE(Arc<Mutex<dyn Table>>),
    FUNCTION(Arc<dyn Fn(&mut Context, &[Atom]) -> Result<Signal, Backtrace>>),
    CLOSURE(Arc<Mutex<Closure>>),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::NULL(null) => f.write_fmt(format_args!("{:?}", null)),
            Value::BOOL(boolean) => f.write_fmt(format_args!("{:?}", boolean)),
            Value::NUMBER(number) => f.write_fmt(format_args!("{:?}", number)),
            Value::STRING(string) => f.write_str(string),
            Value::LIST(_) => f.write_str("list"),
            Value::TABLE(_) => f.write_str("table"),
            Value::FUNCTION(_) => f.write_str("function"),
            Value::CLOSURE(_) => f.write_str("closure"),
        }
    }
}

impl Represent for Value {
    fn represent(&self) -> Result<String, Backtrace> {
        match self {
            Value::NULL(null) => Ok(format!("{}", null)),
            Value::BOOL(boolean) => Ok(format!("{}", boolean)),
            Value::NUMBER(number) => Ok(format!("{}", number)),
            Value::STRING(string) => Ok(string.clone()),
            Value::LIST(list) => {
                let representations = list
                    .iter()
                    .map(|x| match x {
                        Value::STRING(string) => Ok(format!("\"{}\"", string)),
                        _ => x.represent(),
                    })
                    .collect::<Result<Vec<String>, Backtrace>>()?;
                Ok(format!("[{}]", representations.join(", ")))
            }
            Value::TABLE(table) => {
                let table = mutex_lock_unwrap!(table, None);
                table.represent()
            }
            Value::FUNCTION(_) => Ok(String::from("function")),
            Value::CLOSURE(closure) => {
                let closure = mutex_lock_unwrap!(closure, None);
                closure.represent()
            }
        }
    }
}
