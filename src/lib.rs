extern crate curl;
extern crate argparse;
extern crate serde_json;

pub use self::api::query;
pub use self::cli::get_args;

pub mod api;
pub mod cli;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
