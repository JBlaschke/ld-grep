use crate::cc_parser::cray_cc_libdirs;

use std::env;
use std::fs;
use regex::Regex;
use std::error::Error;


pub fn get_libdirs(
        add_cray: bool,
        cmd: &str
    ) -> Result<Vec<String>, Box<dyn Error>> {

    let ld_library_path = env::var("LD_LIBRARY_PATH").unwrap_or_default();
    let mut paths: Vec<String> = ld_library_path.split(":").map(
        |s| s.to_string()
    ).collect();

    if add_cray {
        let cray_libdirs = cray_cc_libdirs(cmd)?;
        paths.extend(cray_libdirs);
    }

    Ok(paths)
}

pub fn filter_libs(
        paths: Vec<String>, pattern: & str
    ) -> Result<Vec<String>, Box<dyn Error>> {

    let regex = Regex::new(pattern).unwrap();
    let mut libs = Vec::new();

    for path in paths {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if ! entry_path.is_file() { continue; }
                    if regex.is_match(entry_path.to_str().unwrap_or("")) {
                        libs.push(entry_path.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }

    Ok(libs)
}