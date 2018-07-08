use std::io;

/// Represents the capability of enumerating EFI variables
pub trait VarEnumerator {
    /// Enumerates all known variables on the system. Returns a list of found variable names.
    ///
    /// *Note that some implementations of `VarEnumerator` rely on a static list since the
    /// underlying OS is not capable of enumerating variables.*
    fn get_var_names(&self) -> io::Result<Vec<String>>;
}
