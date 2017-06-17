# rust_reddit

`rust_reddit` is a library that supports api calls to reddit via rust code.

This repository is in development.

---
## Examples

```rust
// simple use of the macro "rquery"
#[macro_use]
extern crate rust_reddit;

fn main(){
    let data = rquery!(
        "/r/rust/top/.json?count=20",
        "headers" => "User-Agent: rust-reddit-test");

    println!("{}", data);
}

```

