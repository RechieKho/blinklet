use super::float::Float;
use super::variant_ops::{
    VariantAdd, VariantDiv, VariantEq, VariantG, VariantGe, VariantL, VariantLe, VariantMul,
    VariantSub,
};
use super::{represent::Represent, Variant};
use crate::mark::Mark;
use crate::mutex_lock_unwrap;
use crate::{backtrace::Backtrace, raise_error};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct List(Arc<Mutex<Vec<Variant>>>);

impl VariantAdd for List {
    fn add(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark.clone(),
                    "`{}` cannot be added with `{}`.",
                    self.represent(mark.clone())?,
                    rhs.represent(mark.clone())?
                );
            }
        }
    }
}

impl VariantSub for List {
    fn sub(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark.clone(),
                    "`{}` cannot be subtracted with `{}`.",
                    self.represent(mark.clone())?,
                    rhs.represent(mark.clone())?
                );
            }
        }
    }
}

impl VariantMul for List {
    fn mul(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark.clone(),
                    "`{}` cannot be multiplied with `{}`.",
                    self.represent(mark.clone())?,
                    rhs.represent(mark.clone())?
                );
            }
        }
    }
}

impl VariantDiv for List {
    fn div(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark.clone(),
                    "`{}` cannot be divided with `{}`.",
                    self.represent(mark.clone())?,
                    rhs.represent(mark.clone())?
                );
            }
        }
    }
}

impl VariantEq for List {
    fn eq(&self, rhs: &Variant, mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::LIST(list) => {
                let self_guard = mutex_lock_unwrap!(self.0, mark.clone());
                let other_guard = mutex_lock_unwrap!(list.0, mark.clone());

                if self_guard.len() != other_guard.len() {
                    return Ok(false);
                }

                for i in 0..self_guard.len() {
                    let self_element = self_guard.get(i).unwrap();
                    let other_element = other_guard.get(i).unwrap();
                    if !self_element.eq(other_element, mark.clone())? {
                        return Ok(false);
                    }
                }

                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl VariantGe for List {
    fn ge(&self, rhs: &Variant, mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::LIST(list) => {
                let self_guard = mutex_lock_unwrap!(self.0, mark.clone());
                let other_guard = mutex_lock_unwrap!(list.0, mark.clone());

                if self_guard.len() != other_guard.len() {
                    return Ok(false);
                }

                for i in 0..self_guard.len() {
                    let self_element = self_guard.get(i).unwrap();
                    let other_element = other_guard.get(i).unwrap();
                    if !self_element.ge(other_element, mark.clone())? {
                        return Ok(false);
                    }
                }

                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl VariantG for List {
    fn g(&self, rhs: &Variant, mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::LIST(list) => {
                let self_guard = mutex_lock_unwrap!(self.0, mark.clone());
                let other_guard = mutex_lock_unwrap!(list.0, mark.clone());

                if self_guard.len() != other_guard.len() {
                    return Ok(false);
                }

                for i in 0..self_guard.len() {
                    let self_element = self_guard.get(i).unwrap();
                    let other_element = other_guard.get(i).unwrap();
                    if !self_element.g(other_element, mark.clone())? {
                        return Ok(false);
                    }
                }

                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl VariantLe for List {
    fn le(&self, rhs: &Variant, mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::LIST(list) => {
                let self_guard = mutex_lock_unwrap!(self.0, mark.clone());
                let other_guard = mutex_lock_unwrap!(list.0, mark.clone());

                if self_guard.len() != other_guard.len() {
                    return Ok(false);
                }

                for i in 0..self_guard.len() {
                    let self_element = self_guard.get(i).unwrap();
                    let other_element = other_guard.get(i).unwrap();
                    if !self_element.le(other_element, mark.clone())? {
                        return Ok(false);
                    }
                }

                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl VariantL for List {
    fn l(&self, rhs: &Variant, mark: Option<Mark>) -> Result<bool, Backtrace> {
        match rhs {
            Variant::LIST(list) => {
                let self_guard = mutex_lock_unwrap!(self.0, mark.clone());
                let other_guard = mutex_lock_unwrap!(list.0, mark.clone());

                if self_guard.len() != other_guard.len() {
                    return Ok(false);
                }

                for i in 0..self_guard.len() {
                    let self_element = self_guard.get(i).unwrap();
                    let other_element = other_guard.get(i).unwrap();
                    if !self_element.l(other_element, mark.clone())? {
                        return Ok(false);
                    }
                }

                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("list")
    }
}

impl Represent for List {
    fn represent(&self, mark: Option<Mark>) -> Result<String, Backtrace> {
        let guard = mutex_lock_unwrap!(self.0, mark.clone());
        let representations = guard
            .iter()
            .map(|x| match x {
                Variant::STRAND(strand) => Ok(format!("\"{}\"", strand.as_str())),
                _ => x.represent(mark.clone()),
            })
            .collect::<Result<Vec<String>, Backtrace>>()?;
        Ok(format!("[{}]", representations.join(", ")))
    }
}

impl From<Vec<Variant>> for List {
    fn from(value: Vec<Variant>) -> Self {
        List(Arc::new(Mutex::new(value)))
    }
}

impl List {
    pub fn push(&mut self, variant: Variant, mark: Option<Mark>) -> Result<(), Backtrace> {
        let mut guard = mutex_lock_unwrap!(self.0, mark);
        guard.push(variant);
        Ok(())
    }

    pub fn pop(&mut self, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        let mut guard = mutex_lock_unwrap!(self.0, mark);
        let variant = guard.pop();
        Ok(if variant.is_none() {
            raise_error!(mark.clone(), "List given is empty.");
        } else {
            variant.unwrap()
        })
    }

    pub fn length(&self, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        let guard = mutex_lock_unwrap!(self.0, mark);
        Ok(Variant::FLOAT(Float::from(guard.len() as f64)))
    }

    pub fn get(&self, index: Float, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        let guard = mutex_lock_unwrap!(self.0, mark);
        let element = guard.get(Into::<f64>::into(index) as usize);
        if element.is_none() {
            raise_error!(mark, "Accessing index out of bound.");
        }
        let element = element.unwrap();
        Ok(element.clone())
    }
}
