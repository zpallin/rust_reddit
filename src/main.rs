extern crate curl;
extern crate argparse;
extern crate serde_json;
extern crate rust_reddit_api;

use argparse::{ArgumentParser, Store};
use std::io::{stdout, Write};
use std::sync::RwLock;
use curl::easy::{Easy, List};
use serde_json::Value as JsonValue;
use serde_json::Error as JsonError;

use rust_reddit_api::cli;
use rust_reddit_api::api;

fn main(){
    let args = cli::get_args();
    let data = api::query("/r/rust/top/.json?count=20", args);
    println!("{}", data);
}
