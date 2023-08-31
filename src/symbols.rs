use std::fs;
use std::error::Error;
use regex::Regex;
use goblin::elf::Elf;
use goblin::archive::Archive;
use goblin::Object;


#[derive(Debug)]
struct ELFError {
    message: String
}

impl std::fmt::Display for ELFError {
    fn fmt(& self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CmdError {}", self.message)
    }
}

impl Error for ELFError {}


#[derive(Debug)]
pub struct DynSymbol {
    pub value: u64,
    pub bind: u8,
    pub typ: u8,
    pub vis: u8,
    pub name: String,
    pub debug: bool
}

#[derive(Debug)]
pub struct ArchiveSymbol {
    pub member_name: String,
    pub symbol_name: String,
}

#[derive(Debug)]
pub enum Symbol {
    Dynamic(DynSymbol),
    Static(ArchiveSymbol)
}


fn list_elf_symbols(
        elf: Elf
    ) -> Result<Vec<Symbol>, Box<dyn std::error::Error>> {
    let mut v: Vec<Symbol> = Vec::new();

    for sym in & elf.dynsyms {
        v.push(Symbol::Dynamic(DynSymbol {
            value: sym.st_value,
            bind: sym.st_bind(),
            typ: sym.st_type(),
            vis: sym.st_visibility(),
            name: elf.dynstrtab.get_at(sym.st_name).unwrap().to_string(),
            debug: false
        }));
    };

    for sym in & elf.syms {
        v.push(Symbol::Dynamic(DynSymbol {
            value: sym.st_value,
            bind: sym.st_bind(),
            typ: sym.st_type(),
            vis: sym.st_visibility(),
            name: elf.dynstrtab.get_at(sym.st_name).unwrap_or("FAILED").to_string(),
            debug: true
        }));
    };

    Ok(v)
}

fn list_archive_symbols(
        arch: Archive
    ) -> Result<Vec<Symbol>, Box<dyn std::error::Error>> {
    let mut v: Vec<Symbol> = Vec::new();

    for (member_name, _, symbols) in arch.summarize() {
        for x in symbols {
            v.push(Symbol::Static(ArchiveSymbol {
                member_name: member_name.to_string(),
                symbol_name: x.to_string()
            }));
        }
    }
    Ok(v)
}


pub fn list_symbols(
        path: & str
    ) -> Result<Vec<Symbol>, Box<dyn Error>> {
    let buffer = fs::read(path)?;

    match Object::parse(& buffer)? {
        Object::Elf(elf) => {
            return list_elf_symbols(elf);
        },
        Object::Archive(archive) => {
            return list_archive_symbols(archive);
        },
        Object::Unknown(magic) => {
            let error = ELFError{
                message: format!("Unknown magic: {:#x}", magic)
            };
            return Err(Box::new(error))
        }
        _ => {}
    }

    let v: Vec<Symbol> = Vec::new();
    Ok(v)
}

pub fn filter_symbols<'a>(
        symbols: &'a Vec<Symbol>, pattern: & str
    ) -> Result<Vec<&'a Symbol>, Box<dyn std::error::Error>> {

    let regex = Regex::new(pattern).unwrap();
    let mut v: Vec<& Symbol> = Vec::new();

    for s in  symbols {
        match s {
            Symbol::Dynamic(sym) => {
                if regex.is_match(& sym.name.to_string()) {
                    v.push(s);
                }
            }
            Symbol::Static(sym) => {
                if regex.is_match(& sym.symbol_name.to_string()) {
                    v.push(s);
                }
            }
        }
    }

    Ok(v)
}
