mod cc_parser;
mod search_libs;
mod cli;

use crate::search_libs::{get_libdirs, filter_libs};
use crate::cli::init;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = init();
    let regex = args.get_one::<String>("regex").unwrap();
    let use_cray = args.get_one::<bool>("use_cray").unwrap();
    let cmd = args.get_one::<String>("cc_cmd").unwrap();

    let paths: Vec<String> = get_libdirs(* use_cray, cmd)?;

    match filter_libs(paths, regex) {
        Ok(output) => {
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