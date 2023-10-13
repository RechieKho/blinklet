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

use self::variant_ops::{VariantEq, VariantGe, VariantG, VariantLe, VariantL};

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

impl VariantEq for Variant {
    fn eq(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match self {
            Variant::NULL(null) => null.eq(rhs, mark),
            Variant::BOOL(boolean) => boolean.eq(rhs, mark),
            Variant::FLOAT(float) => float.eq(rhs, mark),
            Variant::STRAND(strand) => strand.eq(rhs, mark),
            Variant::LIST(list) => list.eq(rhs, mark),
            Variant::TABLE(table) => table.eq(rhs, mark),
            Variant::COMMAND(command) => command.eq(rhs, mark),
            Variant::CLOSURE(closure) => closure.eq(rhs, mark),
        }
    }
}

impl VariantGe for Variant {
    fn ge(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match self {
            Variant::NULL(null) => null.ge(rhs, mark),
            Variant::BOOL(boolean) => boolean.ge(rhs, mark),
            Variant::FLOAT(float) => float.ge(rhs, mark),
            Variant::STRAND(strand) => strand.ge(rhs, mark),
            Variant::LIST(list) => list.ge(rhs, mark),
            Variant::TABLE(table) => table.ge(rhs, mark),
            Variant::COMMAND(command) => command.ge(rhs, mark),
            Variant::CLOSURE(closure) => closure.ge(rhs, mark),
        }
    }
}

impl VariantG for Variant {
    fn g(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match self {
            Variant::NULL(null) => null.g(rhs, mark),
            Variant::BOOL(boolean) => boolean.g(rhs, mark),
            Variant::FLOAT(float) => float.g(rhs, mark),
            Variant::STRAND(strand) => strand.g(rhs, mark),
            Variant::LIST(list) => list.g(rhs, mark),
            Variant::TABLE(table) => table.g(rhs, mark),
            Variant::COMMAND(command) => command.g(rhs, mark),
            Variant::CLOSURE(closure) => closure.g(rhs, mark),
        }
    }
}

impl VariantLe for Variant {
    fn le(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match self {
            Variant::NULL(null) => null.le(rhs, mark),
            Variant::BOOL(boolean) => boolean.le(rhs, mark),
            Variant::FLOAT(float) => float.le(rhs, mark),
            Variant::STRAND(strand) => strand.le(rhs, mark),
            Variant::LIST(list) => list.le(rhs, mark),
            Variant::TABLE(table) => table.le(rhs, mark),
            Variant::COMMAND(command) => command.le(rhs, mark),
            Variant::CLOSURE(closure) => closure.le(rhs, mark),
        }
    }
}

impl VariantL for Variant {
    fn l(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match self {
            Variant::NULL(null) => null.l(rhs, mark),
            Variant::BOOL(boolean) => boolean.l(rhs, mark),
            Variant::FLOAT(float) => float.l(rhs, mark),
            Variant::STRAND(strand) => strand.l(rhs, mark),
            Variant::LIST(list) => list.l(rhs, mark),
            Variant::TABLE(table) => table.l(rhs, mark),
            Variant::COMMAND(command) => command.l(rhs, mark),
            Variant::CLOSURE(closure) => closure.l(rhs, mark),
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
    fn represent(&self, mark: Option<Mark>) -> Result<String, Backtrace> {
        match self {
            Variant::NULL(null) => null.represent(mark),
            Variant::BOOL(boolean) => boolean.represent(mark),
            Variant::FLOAT(float) => float.represent(mark),
            Variant::STRAND(strand) => strand.represent(mark),
            Variant::LIST(list) => list.represent(mark),
            Variant::TABLE(table) => table.represent(mark),
            Variant::COMMAND(command) => command.represent(mark),
            Variant::CLOSURE(closure) => closure.represent(mark),
        }
    }
}
