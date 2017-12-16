
use serde_json::Error;
use argparse::{ArgumentParser, Store, StoreTrue, StoreFalse};

/// Struct for gathering cli arguments.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Args {
  pub key: String,
  pub headers: String,
  pub nocapture: bool,
}

/// Default args are generic and probably won't work on default.
///
impl Default for Args {
  fn default() -> Args {
    Args {
      key: "".to_string(),
      headers: "".to_string(),
      nocapture: false,
    }
  }
}

/// Gets the arguments from the command line, in case you are 
/// leveraging this as a command line tool.
///
/// ```
/// extern crate rust_reddit;
/// use rust_reddit::cli;
///
/// fn main() {
///     let args = cli::get_args();
/// }
/// ```
///
pub fn get_args() -> Args {
  ///
  /// Argument parsing goes here
  ///
  let mut args = Args::default();
  {
    let mut ap = ArgumentParser::new();
    ap.set_description("Rust Library for Reddit API");
    ap.refer(&mut args.nocapture)
      .add_option(
        &["--nocapture"],
        StoreTrue,
        "Pass nocapture to the cargo test toolchain",
        );
    ap.refer(&mut args.key)
      .add_option(
        &["-k", "--api-key"],
        Store,
        "Your Reddit API key (for authorized-only calls)"
        );
    ap.refer(&mut args.headers)
      .add_option(
        &["-H", "--headers"],
        Store,
        "Headers for the request, delimited by \",\" between full header lines"
        );
    ap.parse_args_or_exit();
  }
  args
}

#[cfg(test)]
mod tests {
  extern crate serde_json;
  use serde_json::to_string as json_to_string;
  use cli::Args;
  use cli::get_args;

#[test]
  fn test_get_args() {
    // Since I am not mocking ArgumentParser, it is not tested properly
    // instead, all I am doing is demonstrating that the args returned
    // will remain default as get_args returns args and by default
    // they are Args::default()
    // This is... okay for now

    use cli::get_args;
    let args_s = json_to_string(&get_args()).unwrap();
    let args_expected = json_to_string(&Args::default()).unwrap(); 

    assert!(args_s == args_expected);

  }
}
