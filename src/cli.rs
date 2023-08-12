use clap::{Arg, Command, ArgAction, ArgMatches};


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
            .default_value("true")
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
        .get_matches();

    return args;
}