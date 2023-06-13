//! # env_vars_explorer
//!
//! This crate provides a simple macro that can be used to explore what environment variables
//! are available at compile-time in your rust project.
//!
//! Simply run `env_vars_explorer::explore!` anywhere and all environment variables will be
//! printed from the proc macro. This is useful to discover what environment variables are
//! available at compile-time v.s. at runtime.
//!
//! ```rust
//! env_vars_explorer::explore!();
//! ```

use proc_macro::TokenStream;
use std::env;

#[proc_macro]
pub fn explore(tokens: TokenStream) -> TokenStream {
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
    tokens
}
