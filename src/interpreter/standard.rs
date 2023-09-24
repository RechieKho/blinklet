pub mod var;
pub mod set;
pub mod print;

#[macro_export]
macro_rules! assert_atoms_count {
    ($atoms:expr, $count:expr) => {
        let atom = $atoms.first().unwrap();
        if $atoms.len() != $count {
            raise_backtrace_error!(
                atom.mark.clone(),
                "Argument count exceeds maximum, which is {}.",
                $count
            );
        }
    }
}

#[macro_export]
macro_rules! assert_atoms_count_max {
    ($atoms:expr, $max:expr) => {
        let atom = $atoms.first().unwrap();
        if $atoms.len() > $max {
            raise_backtrace_error!(
                atom.mark.clone(),
                "Argument count exceeds maximum, which is {}.",
                $max
            );
        }
    };
}

#[macro_export]
macro_rules! assert_atoms_count_min {
    ($atoms:expr, $min:expr) => {
        let atom = $atoms.first().unwrap();
        if $atoms.len() < $min {
            raise_backtrace_error!(
                atom.mark.clone(),
                "Argument count recede minimum, which is {}.",
                $min
            );
        }
    };
}

#[macro_export]
macro_rules! atom_as_identifier {
    ($atom: expr) => {
        if let AtomValue::IDENTIFIER(ref identifier) = $atom.value {
            identifier
        } else {
            return Err(Backtrace::new(Log::error(
                format!("Expecting an identifier."),
                $atom.mark.clone(),
            )));
        }
    };
}
