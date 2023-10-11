use super::represent::Represent;
use crate::backtrace::Backtrace;
use crate::interpreter::variant::Variant;
use hashbrown::HashMap;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};
use super::variant_ops::{VariantAdd, VariantSub, VariantMul, VariantDiv};
use crate::mark::Mark;
use crate::raise_error;

#[derive(Clone)]
pub struct Table(HashMap<String, Variant>);

impl Default for Table {
    fn default() -> Self {
        Table(HashMap::default())
    }
}

impl VariantAdd for Table {
    fn add(&self, rhs: &Variant, mark: Option<Arc<Mark>>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark,
                    "`{}` cannot be added with `{}`.",
                    self.represent()?,
                    rhs.represent()?
                );
            }
        }
    }
}

impl VariantSub for Table {
    fn sub(&self, rhs: &Variant, mark: Option<Arc<Mark>>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark,
                    "`{}` cannot be subtracted with `{}`.",
                    self.represent()?,
                    rhs.represent()?
                );
            }
        }
    }
}

impl VariantMul for Table {
    fn mul(&self, rhs: &Variant, mark: Option<Arc<Mark>>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark,
                    "`{}` cannot be multiplied with `{}`.",
                    self.represent()?,
                    rhs.represent()?
                );
            }
        }
    }
}

impl VariantDiv for Table {
    fn div(&self, rhs: &Variant, mark: Option<Arc<Mark>>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark,
                    "`{}` cannot be divided with `{}`.",
                    self.represent()?,
                    rhs.represent()?
                );
            }
        }
    }
}


impl Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("table") // TODO
    }
}

impl Represent for Table {
    fn represent(&self) -> Result<String, Backtrace> {
        Ok(String::from("table")) // TODO
    }
}

impl Table {
    pub fn insert(&mut self, key: String, value: Variant) -> Option<Variant> {
        self.0.insert(key, value)
    }

    pub fn remove(&mut self, key: &String) -> Option<Variant> {
        self.0.remove(key)
    }

    pub fn get<'a>(&'a self, key: &String) -> Option<&'a Variant> {
        self.0.get(key)
    }

    pub fn contains_key(&self, key: &String) -> bool {
        self.0.contains_key(key)
    }

    pub fn wrap_arc_mutex() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Table::default()))
    }
}
