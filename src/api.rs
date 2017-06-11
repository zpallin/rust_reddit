
// external imports
use std::io::{stdout, Write};
use std::sync::RwLock;
use curl::easy::{Easy, List};
use std::str::from_utf8 as str_from_utf8;
use serde_json::from_str as json_from_str;
use serde_json::Value as JsonValue;
use serde_json::Error as JsonError;

// internal imports
use cli::Args;

/// Queries the reddit api with a string, returns a JsonValue
///
/// # Examples
///
/// ```
/// extern crate serde_json;
/// extern crate rust_reddit;
/// use rust_reddit::api;
/// use serde_json::{Value,Error};
///
/// let res = api::query("/r/rust/top/.json?count=20", args);
/// ```
///
pub fn query(search_string: &str, args: Args) -> JsonValue {

    let mut easy = Easy::new();
    let mut list = List::new();
    let output_locker : RwLock<Vec<String>>= RwLock::new(Vec::new());
    let full_request = format!("https://www.reddit.com{}", search_string);
    let user_agent = format!("User-Agent: {}", args.user_agent); 

    list.append(&user_agent);
    easy.url(&full_request).unwrap();
    easy.http_headers(list).unwrap();

    let mut transfer = easy.transfer();

    transfer.write_function(|data| {
        let mut write_rwlock = output_locker.write().unwrap();
        write_rwlock.push(
            str_from_utf8(data).unwrap().to_string());
        Ok(data.len())
    }).unwrap();
    
    transfer.perform().unwrap();
    
    let output = output_locker.read().unwrap().clone().join("");

    json_from_str(&output).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let args = Args::default();
	    let data = reddit_api_search("/r/rust/top/.json?count=20", args);
    }
}
