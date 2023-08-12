mod cc_parser;
mod search_libs;
mod cli;

use crate::search_libs::{get_libdirs, filter_libs};
use crate::cli::{init, parse};

use libc::EXIT_FAILURE;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = init();
    let cli = parse(& args);

    let paths: Vec<String> = get_libdirs(cli.use_cray, cli.cmd)?;

    match filter_libs(paths, cli.regex) {

        Ok(output) => {
            for lib in output {
                println!("{}", lib);
            }
        }

        Err(err) => {
            println!("{}", err);
            std::process::exit(EXIT_FAILURE);
        }
    }

    Ok(())
}