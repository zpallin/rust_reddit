
// external imports
//use std::io::{stdout, Write};
use std::sync::RwLock;
use curl::easy::{Easy, List};
use std::str::from_utf8 as str_from_utf8;
use serde_json;
use serde_json::{Value, Error};

// internal imports
use cli::*;

pub mod prelude {
  pub use api::{Rreq, Rdata, Initializer, Request};
}

////////////////////////////////////////////////////////////////////////////////


////////////////////////////////////////////////////////////////////////////////
/// Rdata and Rreq struct definitions
#[derive(Serialize, Deserialize, Debug)]
pub enum Rdata {
  String,
  None,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rreq {
  pub sub : String,
  pub req : String,
  pub args : Args,
  pub data : Option<String>,
}

////////////////////////////////////////////////////////////////////////////////
/// Initializer
/// Handles all Rreq Initialization methods
pub trait Initializer {
  fn stub(&str) -> Self;
  fn new(&str, &str) -> Self;
  fn args(&str, Args) -> Self;
  fn full(&str, &str, Args) -> Self;
}

impl Initializer for Rreq {
  /// for ergonomics, generates a Rreq struct without a request string
  fn stub(sub: &str) -> Self {
    Rreq { 
      sub : sub.to_owned(),
      req : "".to_owned(),
      args : Args::default(),
      data : None,
    }
  }

  /// generates a Rreq struct with a request string
  fn new(sub: &str, req: &str) -> Self {
    Rreq {
      sub : sub.to_owned(),
      req : req.to_owned(),
      args  : Args::default(),
      data : None,
    }
  }

  /// generate with args
  fn args(sub: &str, args: Args) -> Self {
    Rreq {
      sub : sub.to_owned(),
      req : "".to_owned(),
      args : args,
      data : None,
    }
  }

  /// generate with args and request
  fn full(sub: &str, req: &str, args: Args) -> Self {
    Rreq {
      sub : sub.to_owned(),
      req : req.to_owned(),
      args : args,
      data : None,
    }
  }
}

////////////////////////////////////////////////////////////////////////////////
/// Request 
/// The baseline request interface used to make calls to reddit

pub trait Request {
  fn uri(&self) -> String;
  fn headers(&self) -> List;
  fn request(&self, &mut Easy) -> String;
  fn query(&self) -> Result<Value, Error>;
}

impl Request for Rreq {
  /// Generates request full uri
  fn uri(&self) -> String{
    format!("https://www.reddit.com/r/{}/{}", self.sub, self.req).to_owned()
  }

  /// Generates a curl::easy::List from HashMap, formats headers
  fn headers(&self) -> List {
    let mut list = List::new();
    for header in self.args.headers.split(",") {
      list.append(header).unwrap();
    }
    list
  }

  /// Takes a formatted curl struct and generates output from a query
  /// sending it back to the caller as a string of JSON
  ///
  /// Unfortunately, due to the complexity of the code here as well as
  /// the fact that this workload here is mostly dependent on code in another
  /// code base, rather than custom unit logic, this remains untested
  ///
  fn request(&self, easy : &mut Easy) -> String {
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
  /// use rust_reddit::api::prelude::*;
  /// use rust_reddit::cli;
  /// 
  /// fn main() {
  ///     let rreq = Rreq::full(
  ///       "rust", 
  ///       "top/.json?count=20", 
  ///       cli::get_args());
  ///
  ///     let res = rreq.query();
  ///     println!("{:?}", res);
  /// }
  /// ```
  ///
  fn query(&self) -> Result<Value, Error>  {

    let mut easy = Easy::new();

    easy.url(&self.uri()).unwrap();
    easy.http_headers(self.headers()).unwrap();

    let output = self.request(&mut easy);

    serde_json::from_str(&output)
  }
}

#[macro_export]
macro_rules! reddit {
  ( $sub:expr ) => {{
    extern crate rust_reddit;
    use rust_reddit::api::prelude::*;

    let rreq = Rreq::stub($sub);
    rreq.query().unwrap()
  }};
  ( $sub:expr, $($key:expr => $val:expr),* ) => {{
    extern crate rust_reddit;
    use rust_reddit::cli::Args;
    use rust_reddit::api::prelude::*;

    let mut args = Args::default();
    let mut rreq = Rreq::stub($sub);
    $(
        let val = $val.to_string();
        match $key {
        "key" => args.key = val,
        "headers" => args.headers = val,
        _ => (),
        }
    )*
    rreq.args = args;
    rreq.query().unwrap()
  }};
  ( $sub:expr, $query:expr ) => {{
    extern crate rust_reddit;
    use rust_reddit::api::prelude::*;

    let rreq = Rreq::new($sub, $query);
    rreq.query().unwrap()
  }};
  ( $sub:expr, $query:expr, $($key:expr => $val:expr),* ) => {{
    extern crate rust_reddit;
    use rust_reddit::api::prelude::*;
    use rust_reddit::cli::Args;

    let mut args = Args::default();
    let mut rreq = Rreq::new($sub, $query);
    $(
      let val = $val.to_string();
      match $key {
      "key" => args.key = val,
      "headers" => args.headers = val,
      _ => (),
      }
    )*
    rreq.args = args;
    rreq.query().unwrap()
  }};
}
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test_api {

  use api::prelude::*;
  use curl::easy::List;
  use std::str::from_utf8 as str_from_utf8;
  
  /// Those pesky list structs need to be easier to handle for things
  /// like tests and print statements, so here we go
  ///
  fn return_vec_from_list(list : List) -> Vec<String> {
      let iter = list.iter();

      iter.map(|res|{ 
          str_from_utf8(res).unwrap().to_string()
      }).collect()
  }


  #[test]
  fn test_gen_request_uri() {

    let expected = "https://www.reddit.com/r/rust/top.json?count=1".to_owned();
    let rreq = Rreq::new("rust", "top.json?count=1");
    let actual = rreq.uri();
    println!("{}", actual);
    assert!(expected == actual);
  }

  #[test]
  fn test_return_vec_from_list() {
    let mut list = List::new();
    list.append("User-Agent: test-user").unwrap();
    list.append("Host: fake.com").unwrap();

    let expect: Vec<String> = vec![
      "User-Agent: test-user".to_string(), 
      "Host: fake.com".to_string()];

    // will fail if this is not a Vec<String>
    let actual: Vec<String> = return_vec_from_list(list);
    let actual_s: String = actual.into_iter().collect();
    let expect_s: String = expect.into_iter().collect();

    assert!(expect_s == actual_s);
  }

  #[test]
  fn test_gen_headers() {
    use cli::Args;

    let mut args = Args::default();
    let mut expect_list = List::new();
    expect_list.append("User-Agent: test-user").unwrap();
    expect_list.append("Host: fake.com").unwrap();

    let mut wrong_list = List::new();
    wrong_list.append("User-Agent: not-user").unwrap();
    wrong_list.append("Host: wrong.org").unwrap();

    args.headers  = "User-Agent: test-user,Host: fake.com".to_owned();
    let mut rreq = Rreq::stub("rust");
    rreq.args = args;

    let actual_list = rreq.headers();

    let actual: String = return_vec_from_list(actual_list).into_iter().collect();
    let expect: String = return_vec_from_list(expect_list).into_iter().collect();
    let wrong: String = return_vec_from_list(wrong_list).into_iter().collect();

    assert!(actual == expect);
    assert!(actual != wrong);
  }

  #[test]
  fn test_rreq() {
    let rreq : Rreq = Rreq::stub("rust");

    // for the time being, tests will query the web and print for "nocapture" debugging
    println!("- test_rreq: {}", rreq.uri());
    println!("- test_rreq: {:?}", rreq.query());
  }
}
