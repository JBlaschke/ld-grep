mod cc_parser;
mod search_libs;
mod cli;
mod symbols;

use crate::search_libs::{get_libdirs, filter_libs};
use crate::cli::{init, parse};
use crate::symbols::{Symbol, list_symbols, filter_symbols};
use goblin::elf::sym::{bind_to_str, type_to_str, visibility_to_str};

use libc::{EXIT_FAILURE, EXIT_SUCCESS};


fn fetch_symbols(lib: & String) -> Vec<Symbol> {
    let symbols = match list_symbols(& lib) {
        Ok(x) => x,
        Err(x) => {
            println!("Error when listing symbols. {}" , x);
            Vec::<Symbol>::new()
        }
    };
    return symbols;
}


fn display_symbol(lib: & String, s: & Symbol) {
    match s {
        Symbol::Dynamic(sym) => {
            let mut prop_descr: Vec<String> = Vec::new();
            if sym.is_debug {
                prop_descr.push("debug".to_string());
            }
            if sym.is_import {
                prop_descr.push("import".to_string());
            } else {
                prop_descr.push("define".to_string());
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


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = init();
    let cli = parse(& args);

    let paths: Vec<String> = get_libdirs(cli.use_cray, cli.cmd)?;

    // Regexes to be used to find .so and .a files
    let so_files = "^.*\\.so[0-9\\.]*$";
    let  a_files = "^.*\\.a[0-9\\.]*$"; 

    if cli.info {
        println!("LD-GREP is searching for libraries (and symbols thereing) here: ");
        for (i, dir) in paths.iter().enumerate() {
            println!("    {}. {}", i, dir);
        }
        match filter_libs(& paths, so_files) {
            Ok(output) => {
                println!("Found {} dynamically-linked libaries.", output.len());
            }
            _ => {}
        }
        match filter_libs(& paths, a_files) {
            Ok(output) => {
                println!("Found {} statically-linked libaries.", output.len());
            }
            _ => {}
        }
        std::process::exit(EXIT_SUCCESS);
    }

    // Regex is required from this point onwards. CLAP guards against the
    // improper omission of the regex argument.
    let regex: & String = cli.regex.unwrap();

    if !cli.symbol {
        match filter_libs(& paths, regex) {

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
        match filter_libs(& paths, so_files) {

            Ok(output) => {
                for lib in output {
                    let symbols = fetch_symbols(& lib);
                    for s in filter_symbols(&symbols, regex)? {
                        display_symbol(& lib, s);
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
                    let symbols = fetch_symbols(& lib);
                    for s in filter_symbols(&symbols, regex)? {
                        display_symbol(& lib, s);
                    }
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