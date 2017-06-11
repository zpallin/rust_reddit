
use argparse::{ArgumentParser, Store};

pub struct Args {
    pub key: String,
    pub user_agent: String,
}

impl Default for Args {
    fn default() -> Args {
        Args {
            key: "".to_string(),
            user_agent: "rust-reddit-api".to_string(),
        }
    }
}

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
