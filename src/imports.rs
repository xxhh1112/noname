use std::fmt;

use crate::{
    circuit_writer::{CircuitWriter, VarInfo},
    constants::Span,
    error::Result,
    parser::{FnSig, Function, UsePath},
    stdlib::{self, parse_fn_sigs, ImportedModule, BUILTIN_FNS},
    type_checker::FnInfo,
    var::Var,
};

/// An actual handle to the internal function to call to resolve a built-in function call.
///
/// Note that the signature of a `FnHandle` is designed to:
/// * `&mut CircuitWriter`: take a mutable reference to the circuit writer, this is because built-ins need to be able to register new variables and add gates to the circuit
/// * `&[Var]`: take an unbounded list of variables, this is because built-ins can take any number of arguments, and different built-ins might take different types of arguments
/// * `Span`: take a span to return user-friendly errors
/// * `-> Result<Option<Var>>`: return a `Result` with an `Option` of a `Var`. This is because built-ins can return a variable, or they can return nothing. If they return nothing, then the `Option` will be `None`. If they return a variable, then the `Option` will be `Some(Var)`.
pub type FnHandle = fn(&mut CircuitWriter, &[VarInfo], Span) -> Result<Option<Var>>;

/// The different types of a noname function.
#[derive(Clone)]
pub enum FnKind {
    /// A built-in is just a handle to a function written in Rust.
    BuiltIn(FnSig, FnHandle),

    /// A native function is represented as an AST.
    Native(Function),

    /// For the main function, we don't need the AST, just the signature
    Main(FnSig),
}

impl fmt::Debug for FnKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fnkind>")
    }
}

pub fn resolve_builtin_functions() -> Vec<FnInfo> {
    parse_fn_sigs(&BUILTIN_FNS)
}

pub fn resolve_imports(path: &UsePath) -> Result<ImportedModule> {
    if path.module.value == "std" {
        stdlib::parse_std_import(&path.submodule.value, path.span)
    } else {
        // we only support std root module for now
        unimplemented!()
    }
}
