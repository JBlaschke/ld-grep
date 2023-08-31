use clap::{Arg, Command, ArgMatches};


pub fn init() -> ArgMatches {
    let args = Command::new("LD Grep")
        .version("1.0")
        .author("Johannes Blaschke")
        .about("Perform regex searches on LD_LIBRARY_PATH and other common library locations")
        .arg(
            Arg::new("regex")
            .value_name("REGEX")
            .help("Regex to match against files in search paths")
            .required(true)
            .index(1)
        )
        .arg(
            Arg::new("use_cray")
            .short('c')
            .long("use-cray")
            .help("Interrogate Cray Compiler Wrappers for additional paths")
            .required(false)
            .default_value("false")
            .value_parser(clap::builder::BoolishValueParser::new())
        )
        .arg(
            Arg::new("cc_cmd")
            .long("cc-cmd")
            .help("cray compiler wrapper command to check for libraries")
            .value_name("CC")
            .default_value("cc")
            .required(false)
        )
        .arg(
            Arg::new("needs")
            .long("needs")
            .help("List libraries that need the input")
            .required(false)
            .default_value("false")
            .value_parser(clap::builder::BoolishValueParser::new())
        )
        .arg(
            Arg::new("provides")
            .long("provides")
            .help("List libraries that provide the input")
            .required(false)
            .default_value("false")
            .value_parser(clap::builder::BoolishValueParser::new())
        )
        .get_matches();

    return args;
}


pub struct CLI<'a> {
    pub regex: &'a str,
    pub use_cray: bool,
    pub cmd: &'a str,
    pub needs: bool,
    pub provies: bool
}


pub fn parse<'a>(args: &'a ArgMatches) -> CLI<'a> {
    let regex = args.get_one::<String>("regex").unwrap();
    let use_cray = args.get_one::<bool>("use_cray").unwrap();
    let cmd = args.get_one::<String>("cc_cmd").unwrap();
    let needs = args.get_one::<bool>("needs").unwrap();
    let provides = args.get_one::<bool>("provides").unwrap();

    CLI {
        regex: regex.as_str(),
        use_cray: * use_cray,
        cmd: cmd.as_str(),
        needs: * needs,
        provies: * provides
    }
}