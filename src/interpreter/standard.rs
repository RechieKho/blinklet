pub mod add_fn;
pub mod break_fn;
pub mod closure_fn;
pub mod continue_fn;
pub mod div_fn;
pub mod eq_fn;
pub mod g_fn;
pub mod ge_fn;
pub mod import_fn;
pub mod l_fn;
pub mod le_fn;
pub mod list_fn;
pub mod list_length_fn;
pub mod list_pop_fn;
pub mod list_push_fn;
pub mod mul_fn;
pub mod parameter_fn;
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
macro_rules! context_get_current_scope {
    ($context:expr, $mark:expr) => {
        match $context.scopes.last_mut() {
            Some(table) => table,
            None => {
                crate::raise_bug!($mark, "Empty scopes should be unreachable.");
            }
        }
    };
}

#[macro_export]
macro_rules! assert_atoms_count {
    ($atoms:expr, $mark:expr, $count:expr) => {
        if $atoms.len() != $count {
            crate::raise_error!(
                $mark,
                "Argument count exceeds maximum, which is {}.",
                $count
            );
        }
    };
}

#[macro_export]
macro_rules! assert_atoms_count_max {
    ($atoms:expr, $mark:expr, $max:expr) => {
        if $atoms.len() > $max {
            crate::raise_error!($mark, "Argument count exceeds maximum, which is {}.", $max);
        }
    };
}

#[macro_export]
macro_rules! assert_atoms_count_min {
    ($atoms:expr, $mark:expr, $min:expr) => {
        if $atoms.len() < $min {
            crate::raise_error!($mark, "Argument count recede minimum, which is {}.", $min);
        }
    };
}
