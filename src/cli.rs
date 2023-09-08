use clap::{Arg, Command, ArgMatches};

use crate::search_libs::get_libdirs;

pub fn init() -> ArgMatches {
    let args = Command::new("LD Grep")
        .version("1.0")
        .author("Johannes Blaschke")
        .about("Perform regex searches on LD_LIBRARY_PATH and other common library locations.")
        .arg(
            Arg::new("regex")
            .value_name("REGEX")
            .help("Regex to match against files in search paths")
            .required(true)
            .index(1)
        )
        .arg(
            Arg::new("cc_cmd")
            .short('c')
            .long("cc-cmd")
            .help("Interrogate Cray Compiler Wrappers for additional paths. CC = command to check for libraries")
            .value_name("CC")
            .default_value("")
            .required(false)
        )
        .arg(
            Arg::new("symbol")
            .short('s')
            .long("symbol")
            .help("List libraries that define or import the symbol")
            .num_args(0)
        )
        .get_matches();

    return args;
}


pub struct CLI<'a> {
    pub regex: &'a str,
    pub use_cray: bool,
    pub cmd: &'a str,
    pub symbol: bool
}


pub fn parse<'a>(args: &'a ArgMatches) -> CLI<'a> {
    let regex = args.get_one::<String>("regex").unwrap();
    let cmd = args.get_one::<String>("cc_cmd").unwrap();
    let symbol = args.get_one::<bool>("symbol").unwrap();

    let cmd_str = cmd.as_str();
    CLI {
        regex: regex.as_str(),
        use_cray: cmd_str != "",
        cmd: cmd_str,
        symbol: * symbol
    }
}