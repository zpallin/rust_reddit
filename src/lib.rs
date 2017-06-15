extern crate curl;
extern crate argparse;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub use self::api::path_query;
pub use self::cli::get_args;

#[macro_use]
pub mod api;
pub mod cli;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
