use clap::{Arg, Command, ArgMatches};


pub fn init() -> ArgMatches {
    let args = Command::new("LD Grep")
        .version("1.0")
        .author("Johannes Blaschke")
        .about("Perform regex searches on LD_LIBRARY_PATH and other common library locations.")
        .arg(
            Arg::new("regex")
            .value_name("REGEX")
            .help("Regex to match against files in search paths")
            .required_unless_present("info")
            .index(1)
        )
        .arg(
            Arg::new("cc_cmd")
            .short('c')
            .long("cc-cmd")
            .help("Interrogate Cray Compiler Wrappers for additional paths. CC = command to check for libraries")
            .value_name("CC")
            .default_value("cc")
            .required(false)
        )
        .arg(
            Arg::new("symbol")
            .short('s')
            .long("symbol")
            .help("List libraries that define or import the symbol")
            .num_args(0)
        )
        .arg(
            Arg::new("info")
            .short('i')
            .long("info")
            .help("List all locations that are searched")
            .num_args(0)
        )
        .get_matches();

    return args;
}


pub struct CLI<'a> {
    pub regex: Option<&'a String>,
    pub use_cray: bool,
    pub cmd: &'a str,
    pub symbol: bool,
    pub info: bool
}


pub fn parse<'a>(args: &'a ArgMatches) -> CLI<'a> {
    let regex = args.get_one::<String>("regex");
    let cmd = args.get_one::<String>("cc_cmd").unwrap();
    let symbol = args.get_one::<bool>("symbol").unwrap();
    let info = args.get_one::<bool>("info").unwrap();

    let cmd_str = cmd.as_str();
    CLI {
        regex: regex,
        use_cray: cmd_str != "",
        cmd: cmd_str,
        symbol: * symbol,
        info: * info
    }
}