pub mod add_fn;
pub mod break_fn;
pub mod closure_fn;
pub mod continue_fn;
pub mod div_fn;
pub mod list_fn;
pub mod list_pop_fn;
pub mod list_push_fn;
pub mod mul_fn;
pub mod print_fn;
pub mod println_fn;
pub mod return_fn;
pub mod set_fn;
pub mod sub_fn;
pub mod table_fn;
pub mod var_fn;
pub mod when_fn;
pub mod while_fn;

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
