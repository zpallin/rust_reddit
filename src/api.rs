
// external imports
//use std::io::{stdout, Write};
use std::sync::RwLock;
use curl::easy::{Easy, List};
use std::str::from_utf8 as str_from_utf8;
use serde_json;

// internal imports
use cli::*;

////////////////////////////////////////////////////////////////////////////////
/// Those pesky list structs need to be easier to handle for things
/// like tests and print statements, so here we go
///
fn return_vec_from_list(list : List) -> Vec<String> {
    let iter = list.iter();

    iter.map(|res|{ 
        str_from_utf8(res).unwrap().to_string()
    }).collect()
}

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

impl Rreq {
  /// for ergonomics, generates a Rreq struct without a request string
  ///
  pub fn stub(sub: &str) -> Self {
    Rreq { 
      sub : sub.to_owned(),
      req : "".to_owned(),
      args : Args::default(),
      data : None,
    }
  }

  /// generates a Rreq struct with a request string
  ///
  pub fn new(sub: &str, req: &str) -> Self {
    Rreq {
      sub : sub.to_owned(),
      req : req.to_owned(),
      args  : Args::default(),
      data : None,
    }
  }

  /// generate with args
  ///
  pub fn args(sub: &str, args: Args) -> Self {
    Rreq {
      sub : sub.to_owned(),
      req : "".to_owned(),
      args : args,
      data : None,
    }
  }

  /// generate with args and request
  ///
  pub fn full(sub: &str, req: &str, args: Args) -> Self {
    Rreq {
      sub : sub.to_owned(),
      req : req.to_owned(),
      args : args,
      data : None,
    }
  }

  /// Generates request full uri
  ///
  pub fn uri(&self) -> String{
    format!("https://www.reddit.com/r/{}/{}", self.sub, self.req).to_owned()
  }

  /// Generates a curl::easy::List from HashMap, formats headers
  ///
  fn headers(&self) -> List {
    let mut list = List::new();
    for header in self.args.headers.split(",") {
      match list.append(header) {
        Ok(v) => (v),
        Err(e) => panic!(format!("{}", e)),
      }
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
  pub fn web_request(&self, easy : &mut Easy) -> String {
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
  /// 
  /// fn main() {
  ///     let args = cli::get_args();
  ///     let rreq = api::Rreq::full("rust", "top/.json?count=20", args);
  ///     let res = rreq.query();
  ///     println!("{}", res);
  /// }
  /// ```
  ///
  pub fn query(&self) -> String { //serde_json::Value {

    let mut easy = Easy::new();

    easy.url(&self.uri()).unwrap();
    easy.http_headers(self.headers()).unwrap();

    let output = self.web_request(&mut easy);

    output
    //serde_json::from_str(&output).unwrap()
  }
}

#[macro_export]
macro_rules! reddit {
  ( $sub:expr ) => {{
    extern crate rust_reddit;
    use rust_reddit::api::Rreq;
    let rreq = Rreq::stub($sub);
    rreq.query()
  }};
  ( $sub:expr, $($key:expr => $val:expr),* ) => {{
    extern crate rust_reddit;
    use rust_reddit::api::Rreq;
    use rust_reddit::cli::Args;
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
    rreq.query()
  }};
  ( $sub:expr, $query:expr ) => {{
    extern crate rust_reddit;
    use rust_reddit::api::Rreq;
    let rreq = Rreq::new($sub, $query);
    rreq.query()
  }};
  ( $sub:expr, $query:expr, $($key:expr => $val:expr),* ) => {{
    extern crate rust_reddit;
    use rust_reddit::api::Rreq;
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
    rreq.query()
  }};
}
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test_api {

  use api::{Rreq, Rdata};
  use api::return_vec_from_list;
  use curl::easy::List;

  #[test]
  fn test_gen_request_uri() {

    let expected = "https://www.reddit.com/r/rust/top/.json?count=20".to_owned();
    let rreq = Rreq::new("rust", "top/.json?count=20");
    let actual = rreq.uri();
    println!("{}", actual);
    assert!(expected == actual);
  }

  #[test]
  fn test_return_vec_from_list() {
    let mut list = List::new();
    list.append("User-Agent: test-user");
    list.append("Host: fake.com");

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
    expect_list.append("User-Agent: test-user");
    expect_list.append("Host: fake.com");

    let mut wrong_list = List::new();
    wrong_list.append("User-Agent: not-user");
    wrong_list.append("Host: wrong.org");

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
    use serde_json::Value;
    let rreq : Rreq = Rreq::stub("rust");

    // for the time being, tests will query the web and print for "nocapture" debugging
    println!("- test_rreq: {}", rreq.uri());
    println!("- test_rreq: {}", rreq.query());
  }
}
