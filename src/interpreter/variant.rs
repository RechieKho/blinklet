pub mod boolean;
pub mod closure;
pub mod command;
pub mod null;
pub mod represent;
pub mod scope;
pub mod strand;
pub mod table;

use crate::backtrace::Backtrace;
use boolean::Boolean;
use closure::Closure;
use command::Command;
use null::Null;
use represent::Represent;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;
use strand::Strand;
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
pub enum Variant {
    NULL(Null),
    BOOL(Boolean),
    NUMBER(f64),
    STRAND(Strand),
    LIST(Vec<Variant>),
    TABLE(Arc<Mutex<dyn Table>>),
    COMMAND(Arc<Command>),
    CLOSURE(Arc<Mutex<Closure>>),
}

impl Debug for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::NULL(null) => f.write_fmt(format_args!("{:?}", null)),
            Variant::BOOL(boolean) => f.write_fmt(format_args!("{:?}", boolean)),
            Variant::NUMBER(number) => f.write_fmt(format_args!("{:?}", number)),
            Variant::STRAND(strand) => f.write_fmt(format_args!("{:?}", strand)),
            Variant::LIST(_) => f.write_str("list"),
            Variant::TABLE(_) => f.write_str("table"),
            Variant::COMMAND(command) => f.write_fmt(format_args!("{:?}", command)),
            Variant::CLOSURE(_) => f.write_str("closure"),
        }
    }
}

impl Represent for Variant {
    fn represent(&self) -> Result<String, Backtrace> {
        match self {
            Variant::NULL(null) => null.represent(),
            Variant::BOOL(boolean) => boolean.represent(),
            Variant::NUMBER(number) => Ok(format!("{}", number)),
            Variant::STRAND(strand) => strand.represent(),
            Variant::LIST(list) => {
                let representations = list
                    .iter()
                    .map(|x| match x {
                        Variant::STRAND(strand) => Ok(format!("\"{}\"", strand.as_str())),
                        _ => x.represent(),
                    })
                    .collect::<Result<Vec<String>, Backtrace>>()?;
                Ok(format!("[{}]", representations.join(", ")))
            }
            Variant::TABLE(table) => {
                let table = mutex_lock_unwrap!(table, None);
                table.represent()
            }
            Variant::COMMAND(command) => command.represent(),
            Variant::CLOSURE(closure) => {
                let closure = mutex_lock_unwrap!(closure, None);
                closure.represent()
            }
        }
    }
}
