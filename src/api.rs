
#![feature(macro_rules)]

// external imports
use std::io::{stdout, Write};
use std::sync::RwLock;
use curl::easy::{Easy, List};
use std::str::from_utf8 as str_from_utf8;
use std::collections::HashMap;
use serde_json;

// internal imports
use cli::*;

/// Generates request full uri
///
fn gen_request_uri(search: &str) -> String{
    format!("https://www.reddit.com{}", search).to_owned()
}

/// Generates a curl::easy::List from HashMap, formats headers
///
fn gen_headers(header_string : String) -> List {
    let mut list = List::new();
    for header in header_string.split(",") {
        list.append(header);
    }
    list
}

/// Takes a formatted curl struct and generates output from a query
/// sending it back to the caller as a string of JSON
///
pub fn get_output_from_transfer(easy : &mut Easy) -> String {
    let output_locker : RwLock<Vec<String>>= RwLock::new(Vec::new());
    let mut transfer = easy.transfer();

    transfer.write_function(|data| {
        let mut write_rwlock = output_locker.write().unwrap();
        write_rwlock.push(
            str_from_utf8(data).unwrap().to_string());
        Ok(data.len())
    }).unwrap();
    
    transfer.perform().unwrap();
    
    let output = output_locker.read().unwrap().clone().join("");
    output
}

/// Queries the reddit api with a string, returns a serde_json::Value
///
/// # Examples
///
/// ```
/// extern crate serde_json;
/// extern crate rust_reddit;
/// use rust_reddit::api;
/// use rust_reddit::cli;
/// use serde_json::{Value,Error};
/// 
/// fn main() {
///     let args = cli::get_args();
///     let res = api::path_query("/r/rust/top/.json?count=20", args);
/// }
/// ```
///
pub fn path_query(search_string: &str, args: Args) -> serde_json::Value {

    let mut easy = Easy::new();
    let mut list = List::new();

    easy.url(&gen_request_uri(search_string)).unwrap();
    easy.http_headers(gen_headers(args.headers)).unwrap();

    let output = get_output_from_transfer(&mut easy);

    serde_json::from_str(&output).unwrap()
}

#[macro_export]
macro_rules! rquery {
    ( $q:expr ) => {{
        extern crate rust_reddit;
        use rust_reddit::api::path_query;
        use rust_reddit::cli::Args;
        path_query($q, Args::default())
    }};
    ( $q:expr, $($key:expr => $val:expr),* ) => {{
        extern crate rust_reddit;
        use rust_reddit::api::path_query;
        use rust_reddit::cli::Args;
        let mut args = Args::default();
        $(
            let val = $val.to_string();
            match $key {
                "key" => args.key = val,
                "headers" => args.headers = val,
                _ => (),
            }
        )*
        path_query($q, args)
    }}
}

#[cfg(test)]
mod test_api {
    #[test]
    fn test_gen_request_uri() {
        use api::gen_request_uri;

        let expected_output = "https://www.reddit.com/r/rust/top/.json?count=20".to_owned();
        let actual_output = gen_request_uri("/r/rust/top/.json?count=20");
        assert!(expected_output == actual_output);
    }
}
