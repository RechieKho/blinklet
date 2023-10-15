use super::{Resource, ResourcePath};
use crate::backtrace::Backtrace;
use crate::raise_error;
use std::fs::read_to_string;
use std::path::PathBuf;

pub struct SystemResource {
    prefix: ResourcePath,
}

impl Default for SystemResource {
    fn default() -> Self {
        SystemResource {
            prefix: ResourcePath::default(),
        }
    }
}

impl Resource for SystemResource {
    fn get_code(&mut self, mut path: ResourcePath) -> Result<String, Backtrace> {
        let mut resolved = self.prefix.clone();
        let _ = resolved.append(&mut path);
        let result = read_to_string(Into::<PathBuf>::into(resolved.clone()));
        if result.is_err() {
            raise_error!(
                None,
                "Unable to fetch code '{}'.",
                Into::<String>::into(resolved)
            );
        } else {
            Ok(result.unwrap())
        }
    }

    fn get_prefix<'a>(&'a self) -> &'a ResourcePath {
        return &self.prefix;
    }

    fn set_prefix(&mut self, path: ResourcePath) {
        self.prefix = path;
    }
}

impl From<ResourcePath> for SystemResource {
    fn from(value: ResourcePath) -> Self {
        SystemResource { prefix: value }
    }
}
