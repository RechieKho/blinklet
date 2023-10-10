use super::{represent::Represent, Variant};

pub trait Table: Represent {
    fn insert(&mut self, key: String, value: Variant) -> Option<Variant>;
    fn remove(&mut self, key: &String) -> Option<Variant>;
    fn get<'a>(&'a self, key: &String) -> Option<&'a Variant>;
    fn contains_key(&self, key: &String) -> bool;
}
