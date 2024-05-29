//! This crate contains the expansion logic for a Solidity proc_macro2::TokenStream.
//! Its used to expand and generate Rust bindings from Solidity.
//!
//! Note: This is not the procedural macro crate, it is intended to be used as library crate.
//!
//! This crate is used by [`sol!`][sol!] macro in `linera-alloy-sol-macro` crate.
//!
//! [sol!]: https://docs.rs/linera-alloy-sol-macro/latest/linera_alloy_sol_macro/index.html

pub mod expand;
mod utils;
#[cfg(feature = "json")]
mod verbatim;

extern crate syn_solidity as ast;
