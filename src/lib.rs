//! `rust_reddit` is a library that supports api calls to reddit via rust code.
//!
//!	This repository is in development.
//!
//! ---
//! # Examples
//!
//! ```
//! // simple use of the macro "rquery"
//! #[macro_use]
//! extern crate rust_reddit;
//! 
//! fn main(){
//!     let data = rquery!(
//!         "/r/rust/top/.json?count=20",
//!         "user_agent" => "rust-reddit-test");
//! 
//!     println!("{}", data);
//! }
//!
//! ```
//!

extern crate curl;
extern crate argparse;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub use self::api::path_query;
pub use self::cli::get_args;

#[macro_use]
pub mod api;
pub mod cli;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
