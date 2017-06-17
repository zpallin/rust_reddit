
use serde_json::Error;
use argparse::{ArgumentParser, Store};
use std::collections::HashMap;

/// Struct for gathering cli arguments.
///
#[derive(Serialize, Deserialize)]
pub struct Args {
    pub key: String,
    pub headers: String,
}

/// Default args are generic and probably won't work on default.
///
impl Default for Args {
    fn default() -> Args {
        Args {
            key: "".to_string(),
            headers: "".to_string(),
        }
    }
}

/// Gets the arguments from the command line, in case you are 
/// leveraging this as a command line tool.
///
/// # Example:
/// ```
/// extern crate rust_reddit;
/// use rust_reddit::cli;
///
/// let args = cli::get_args();
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
    #[test]
    fn it_works() {
        let args = get_args();
    }
}
