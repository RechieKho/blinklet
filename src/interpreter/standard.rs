pub mod var;

#[macro_export]
macro_rules! assert_atoms_count {
    ($atoms:expr, $range:expr) => {
        let atom = $atoms.first().unwrap();
        if $atoms.len() > $range.end {
            raise_backtrace_error!(
                atom.mark.clone(),
                "Argument count exceeds maximum, which is {}.",
                $range.end
            );
        } else if $atoms.len() < $range.start {
            raise_backtrace_error!(
                atom.mark.clone(),
                "Argument count recede minimum, which is {}.",
                $range.start
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
