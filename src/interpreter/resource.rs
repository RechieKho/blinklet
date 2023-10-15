pub mod system_resource;

use crate::{backtrace::Backtrace, raise_error};
use std::path::{Path, PathBuf, MAIN_SEPARATOR_STR};

pub const SEPERATOR_STR: &'static str = "::";

pub trait Resource {
    fn get_code(&mut self, path: ResourcePath) -> Result<String, Backtrace>;
    fn set_prefix(&mut self, path: ResourcePath);
    fn get_prefix<'a>(&'a self) -> &'a ResourcePath;
}

#[derive(Debug, Clone)]
pub struct ResourcePath(Vec<String>);

impl Default for ResourcePath {
    fn default() -> Self {
        ResourcePath(Vec::new())
    }
}

impl TryFrom<String> for ResourcePath {
    type Error = Backtrace;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut path = ResourcePath(
            value
                .split(SEPERATOR_STR)
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
        );
        path.simplify()?;
        Ok(path)
    }
}

impl TryFrom<&Path> for ResourcePath {
    type Error = Backtrace;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let mut path = ResourcePath(
            value
                .components()
                .into_iter()
                .map(|path| String::from(path.as_os_str().to_str().unwrap_or_default()))
                .collect(),
        );
        path.simplify()?;
        Ok(path)
    }
}

impl TryFrom<PathBuf> for ResourcePath {
    type Error = Backtrace;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let mut path = ResourcePath(
            value
                .to_str()
                .unwrap_or_default()
                .split(MAIN_SEPARATOR_STR)
                .into_iter()
                .map(|string| string.to_string())
                .collect(),
        );
        path.simplify()?;
        Ok(path)
    }
}

impl Into<String> for ResourcePath {
    fn into(self) -> String {
        self.0.join(SEPERATOR_STR).to_string()
    }
}

impl Into<Vec<String>> for ResourcePath {
    fn into(self) -> Vec<String> {
        self.0
    }
}

impl Into<PathBuf> for ResourcePath {
    fn into(self) -> PathBuf {
        PathBuf::from(&self.0.join(MAIN_SEPARATOR_STR))
    }
}

impl ResourcePath {
    pub fn append(&mut self, other: &mut Self) -> Result<(), Backtrace> {
        self.0.append(&mut other.0);
        self.simplify()
    }

    pub fn remove_parent_path(&mut self) -> ResourcePath {
        ResourcePath(if self.0.len() <= 1 {
            Vec::new()
        } else {
            self.0.drain(0..self.0.len() - 1).collect()
        })
    }

    fn simplify(&mut self) -> Result<(), Backtrace> {
        let serialized: String = self.clone().into();
        let mut simplified: Vec<String> = Vec::with_capacity(self.0.capacity());
        for component in self.0.drain(..) {
            if component == "." {
                continue;
            } else if component == ".." {
                if simplified.len() == 0 {
                    raise_error!(None, "Path '{}' attempts to be out of root.", serialized);
                }
                simplified.pop();
            } else {
                simplified.push(component);
            }
        }
        self.0 = simplified;
        Ok(())
    }
}
