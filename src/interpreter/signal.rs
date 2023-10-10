use super::variant::Variant;
use crate::mark::Mark;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Signal {
    COMPLETE(Variant),
    RETURN(Variant, Arc<Mark>),
    BREAK(Arc<Mark>),
    CONTINUE(Arc<Mark>),
}
