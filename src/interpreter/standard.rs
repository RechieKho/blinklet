pub mod add;
pub mod div;
pub mod list;
pub mod mul;
pub mod print;
pub mod rep;
pub mod return_fn;
pub mod set;
pub mod sub;
pub mod var;

#[macro_export]
macro_rules! assert_atoms_count {
    ($atoms:expr, $count:expr) => {
        let atom = $atoms.first().unwrap();
        if $atoms.len() != $count {
            crate::raise_error!(
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
            crate::raise_error!(
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
            crate::raise_error!(
                Some(atom.mark.clone()),
                "Argument count recede minimum, which is {}.",
                $min
            );
        }
    };
}
