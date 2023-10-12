use super::variant::Variant;
use crate::mark::Mark;

#[derive(Debug, Clone)]
pub enum Signal {
    COMPLETE(Variant),
    RETURN(Variant, Mark),
    BREAK(Mark),
    CONTINUE(Mark),
}
