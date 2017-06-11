
use argparse::{ArgumentParser, Store};

/// Struct for gathering cli arguments.
///
pub struct Args {
    pub key: String,
    pub user_agent: String,
}

/// Default args are generic and probably won't work on default.
///
impl Default for Args {
    fn default() -> Args {
        Args {
            key: "".to_string(),
            user_agent: "rust-reddit-api".to_string(),
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
                "Your Reddit API key (which you will need)"
            ).required();
        ap.refer(&mut args.user_agent)
            .add_option(
                &["-i", "--user-agent"],
                Store,
                "A user user-agent to pass to the HTTP request"
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
