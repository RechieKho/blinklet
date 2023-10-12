use super::represent::Represent;
use super::variant_ops::{VariantAdd, VariantDiv, VariantMul, VariantSub};
use crate::backtrace::Backtrace;
use crate::interpreter::variant::Variant;
use crate::mark::Mark;
use crate::{mutex_lock_unwrap, raise_error};
use hashbrown::HashMap;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Table(Arc<Mutex<HashMap<String, Variant>>>);

impl Default for Table {
    fn default() -> Self {
        Table(Arc::new(Mutex::new(HashMap::default())))
    }
}

impl VariantAdd for Table {
    fn add(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
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
    fn sub(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
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
    fn mul(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
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
    fn div(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
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
    pub fn insert(
        &mut self,
        key: String,
        value: Variant,
        mark: Option<Mark>,
    ) -> Result<Option<Variant>, Backtrace> {
        let mut guard = mutex_lock_unwrap!(self.0, mark);
        Ok(guard.insert(key, value))
    }

    pub fn remove(
        &mut self,
        key: &String,
        mark: Option<Mark>,
    ) -> Result<Option<Variant>, Backtrace> {
        let mut guard = mutex_lock_unwrap!(self.0, mark);
        Ok(guard.remove(key))
    }

    pub fn get(&self, key: &String, mark: Option<Mark>) -> Result<Option<Variant>, Backtrace> {
        let guard = mutex_lock_unwrap!(self.0, mark);
        let variant = guard.get(key);
        Ok(if variant.is_none() {
            None
        } else {
            Some(variant.unwrap().clone())
        })
    }

    pub fn contains_key(&self, key: &String, mark: Option<Mark>) -> Result<bool, Backtrace> {
        let guard = mutex_lock_unwrap!(self.0, mark);
        Ok(guard.contains_key(key))
    }
}
