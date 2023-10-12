pub mod boolean;
pub mod closure;
pub mod command;
pub mod float;
pub mod list;
pub mod null;
pub mod represent;
pub mod strand;
pub mod table;
pub mod variant_ops;

use crate::backtrace::Backtrace;
use crate::mark::Mark;
use boolean::Boolean;
use closure::Closure;
use command::Command;
use float::Float;
use list::List;
use null::Null;
use represent::Represent;
use std::fmt::Debug;
use strand::Strand;
use table::Table;
use variant_ops::{VariantAdd, VariantDiv, VariantMul, VariantSub};

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
    FLOAT(Float),
    STRAND(Strand),
    LIST(List),
    TABLE(Table),
    COMMAND(Command),
    CLOSURE(Closure),
}

impl VariantAdd for Variant {
    fn add(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match self {
            Variant::NULL(null) => null.add(rhs, mark),
            Variant::BOOL(boolean) => boolean.add(rhs, mark),
            Variant::FLOAT(float) => float.add(rhs, mark),
            Variant::STRAND(strand) => strand.add(rhs, mark),
            Variant::LIST(list) => list.add(rhs, mark),
            Variant::TABLE(table) => table.add(rhs, mark),
            Variant::COMMAND(command) => command.add(rhs, mark),
            Variant::CLOSURE(closure) => closure.add(rhs, mark),
        }
    }
}

impl VariantSub for Variant {
    fn sub(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match self {
            Variant::NULL(null) => null.sub(rhs, mark),
            Variant::BOOL(boolean) => boolean.sub(rhs, mark),
            Variant::FLOAT(float) => float.sub(rhs, mark),
            Variant::STRAND(strand) => strand.sub(rhs, mark),
            Variant::LIST(list) => list.sub(rhs, mark),
            Variant::TABLE(table) => table.sub(rhs, mark),
            Variant::COMMAND(command) => command.sub(rhs, mark),
            Variant::CLOSURE(closure) => closure.sub(rhs, mark),
        }
    }
}

impl VariantMul for Variant {
    fn mul(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match self {
            Variant::NULL(null) => null.mul(rhs, mark),
            Variant::BOOL(boolean) => boolean.mul(rhs, mark),
            Variant::FLOAT(float) => float.mul(rhs, mark),
            Variant::STRAND(strand) => strand.mul(rhs, mark),
            Variant::LIST(list) => list.mul(rhs, mark),
            Variant::TABLE(table) => table.mul(rhs, mark),
            Variant::COMMAND(command) => command.mul(rhs, mark),
            Variant::CLOSURE(closure) => closure.mul(rhs, mark),
        }
    }
}

impl VariantDiv for Variant {
    fn div(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match self {
            Variant::NULL(null) => null.div(rhs, mark),
            Variant::BOOL(boolean) => boolean.div(rhs, mark),
            Variant::FLOAT(float) => float.div(rhs, mark),
            Variant::STRAND(strand) => strand.div(rhs, mark),
            Variant::LIST(list) => list.div(rhs, mark),
            Variant::TABLE(table) => table.div(rhs, mark),
            Variant::COMMAND(command) => command.div(rhs, mark),
            Variant::CLOSURE(closure) => closure.div(rhs, mark),
        }
    }
}

impl Debug for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::NULL(null) => f.write_fmt(format_args!("{:?}", null)),
            Variant::BOOL(boolean) => f.write_fmt(format_args!("{:?}", boolean)),
            Variant::FLOAT(float) => f.write_fmt(format_args!("{:?}", float)),
            Variant::STRAND(strand) => f.write_fmt(format_args!("{:?}", strand)),
            Variant::LIST(list) => f.write_fmt(format_args!("{:?}", list)),
            Variant::TABLE(table) => f.write_fmt(format_args!("{:?}", table)),
            Variant::COMMAND(command) => f.write_fmt(format_args!("{:?}", command)),
            Variant::CLOSURE(closure) => f.write_fmt(format_args!("{:?}", closure)),
        }
    }
}

impl Represent for Variant {
    fn represent(&self) -> Result<String, Backtrace> {
        match self {
            Variant::NULL(null) => null.represent(),
            Variant::BOOL(boolean) => boolean.represent(),
            Variant::FLOAT(float) => float.represent(),
            Variant::STRAND(strand) => strand.represent(),
            Variant::LIST(list) => list.represent(),
            Variant::TABLE(table) => table.represent(),
            Variant::COMMAND(command) => command.represent(),
            Variant::CLOSURE(closure) => closure.represent(),
        }
    }
}
