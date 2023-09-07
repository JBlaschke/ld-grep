mod cc_parser;
mod search_libs;
mod cli;
mod symbols;

use crate::search_libs::{get_libdirs, filter_libs};
use crate::cli::{init, parse};
use crate::symbols::{Symbol, list_symbols, filter_symbols};
use goblin::elf::sym::{bind_to_str, type_to_str, visibility_to_str};

use libc::EXIT_FAILURE;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = init();
    let cli = parse(& args);

    let paths: Vec<String> = get_libdirs(cli.use_cray, cli.cmd)?;
    let look_for_symbols = cli.needs || cli.provies;

    if !look_for_symbols {
        match filter_libs(& paths, cli.regex) {

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
    } else {
        let so_files = "^.*\\.so[0-9\\.]*$";
        let  a_files = "^.*\\.a[0-9\\.]*$"; 

        match filter_libs(& paths, so_files) {

            Ok(output) => {
                for lib in output {
                    let symbols = match list_symbols(& lib) {
                        Ok(x) => x,
                        Err(x) => {
                            println!("Error when listing symbols. {}" , x);
                            Vec::<Symbol>::new()
                        }
                    };
                    for s in filter_symbols(&symbols, cli.regex)? {
                        match s {
                            Symbol::Dynamic(sym) => {
                                let mut prop_descr: Vec<String> = Vec::new();
                                if sym.is_debug {
                                    prop_descr.push("debug".to_string());
                                }
                                if sym.is_import {
                                    prop_descr.push("import".to_string());
                                }
                                println!(
                                    "{:<6} {:<6} {:<6} {:<12} {:}: {:}",
                                    bind_to_str(sym.bind),
                                    type_to_str(sym.typ),
                                    visibility_to_str(sym.vis),
                                    prop_descr.join(","), lib, sym.name,
                                )
                            }
                            Symbol::Static(sym) => {
                                println!(
                                    "{}: {} {}",
                                    lib, sym.member_name, sym.symbol_name
                                )
                            }
                        }
                    }
                }
            }

            Err(err) => {
                println!("{}", err);
                std::process::exit(EXIT_FAILURE);
            }
        }

        match filter_libs(& paths, a_files) {

            Ok(output) => {
                for lib in output {
                    let symbols = list_symbols(& lib)?;
                }
            }

            Err(err) => {
                println!("{}", err);
                std::process::exit(EXIT_FAILURE);
            }
        }
    }

    Ok(())
}