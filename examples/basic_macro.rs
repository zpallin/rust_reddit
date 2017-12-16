
#[macro_use]
extern crate rust_reddit;

fn main(){
    let data = reddit!(
        "rust",
        "top.json?count=20",
        "headers" => "User-Agent: rust-reddit-test");

    println!("{}", data);
}
