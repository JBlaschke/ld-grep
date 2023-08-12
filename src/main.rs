mod cc_parser;
mod search_libs;

use crate::search_libs::{get_libdirs, filter_libs};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let paths: Vec<String> = get_libdirs(true)?;

    match filter_libs(paths, ".*so.*") {
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

    Ok(())
}