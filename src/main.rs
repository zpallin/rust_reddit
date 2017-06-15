#[macro_use]
extern crate rust_reddit;

use rust_reddit::cli;
use rust_reddit::api;

fn main(){
    let args = cli::get_args();
    let data = rquery!(
        "/r/rust/top/.json?count=20", 
        "key" => "Stuff", 
        "user_agent" => "zpallin");
    
    println!("{}", data); 

}
