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
//!     let data = reddit!(
//!         "rust",
//!         "top/.json?count=20",
//!         "headers" => "User-Agent: rust-reddit-test");
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

#[macro_use]
pub mod api;
pub mod cli;

