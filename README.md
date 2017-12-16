[![Build Status](https://travis-ci.org/zpallin/rust_reddit.svg?branch=master)](https://travis-ci.org/zpallin/rust_reddit)

# rust_reddit

`rust_reddit` is a library that supports api calls to reddit via rust code.

## Purpose

The intent of this library is two fold:

1. provide an easy, extensible way to integrate reddit api calls into your program
2. give me a fun project to practice rust development

Rust has proven itself as a competitor to other low-level languages in many ways, but one way in which rust developers would like to see more development is in open-source, fun projects.

Reddit is a popular social media site and provides a fairly simple REST api to access its data. People regularly create bots for reddit in languages like Ruby, Python, and Javascript. For developers who prefer to write in lower languages because of the additional challenges and higher performance, I find that there should be an extensible Reddit library written in Rust.

---
## Examples

```rust
// simple use of the macro "reddit!"
#[macro_use]
extern crate rust_reddit;

fn main(){
    let data = reddit!(
        "rust",
        "top/.json?count=20",
        "headers" => "User-Agent: rust-reddit-test");

    println!("{}", data);
}

```
