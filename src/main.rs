mod cc_parser;
mod search_libs;

use crate::search_libs::{get_libdirs, find_all_libs};

fn main() {
    match find_all_libs(".*so.*") {
        Ok(output) => {
            println!("Command OK:");
            for lib in output {
                println!("{}", lib);
            }
        }
        Err(err) => {
            println!("Command FAILED: {}", err);
        }
    }
}