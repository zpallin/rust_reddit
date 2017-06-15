#[macro_use]
extern crate rust_reddit;

fn main(){
    let data = rquery!(
        "/r/rust/top/.json?count=20",
        "user_agent" => "rust-reddit-test");

    println!("{}", data);
}
