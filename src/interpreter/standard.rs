pub mod print;
pub mod list;
pub mod set;
pub mod var;

#[macro_export]
macro_rules! assert_atoms_count {
    ($atoms:expr, $count:expr) => {
        let atom = $atoms.first().unwrap();
        if $atoms.len() != $count {
            raise_error!(
                Some(atom.mark.clone()),
                "Argument count exceeds maximum, which is {}.",
                $count
            );
        }
    };
}

#[macro_export]
macro_rules! assert_atoms_count_max {
    ($atoms:expr, $max:expr) => {
        let atom = $atoms.first().unwrap();
        if $atoms.len() > $max {
            raise_error!(
                Some(atom.mark.clone()),
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
            raise_error!(
                Some(atom.mark.clone()),
                "Argument count recede minimum, which is {}.",
                $min
            );
        }
    };
}
