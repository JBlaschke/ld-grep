use crate::cc_parser::cray_cc_libdirs;

use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use regex::Regex;
use std::error::Error;
use std::collections::HashSet;
use glob::glob;


fn process_ld_conf_file(
        path: &Path,
        paths_to_process: &mut Vec<PathBuf>,
        ld_conf: &mut Vec<String>
    ) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() || line.starts_with('#') {
            continue; // Skip empty lines and comments
        }

        let trimmed_line = line.trim();
        if trimmed_line.starts_with("include ") {
            let include_path = Path::new(&trimmed_line[8..]);
            if include_path.is_absolute() {
                // Handle absolute paths
                paths_to_process.push(include_path.to_path_buf());
            } else {
                // Handle relative paths
                let dir = path.parent().unwrap_or(Path::new("/"));
                let full_path = dir.join(include_path);
                paths_to_process.push(full_path);
            }
        } else {
            // Add ld.so.conf locations to the list of search paths
            ld_conf.push(trimmed_line.to_string());
        }
    }

    Ok(())
}


pub fn get_libdirs(
        add_cray: bool,
        cmd: &str
    ) -> Result<Vec<String>, Box<dyn Error>> {

    // Defaults from ld.so
    let mut libdirs: Vec<String> = vec![
        "/lib64".to_string(), "/lib".to_string(),
        "/usr/lib64".to_string(), "/usr/lib".to_string()
    ];

    // Process ld.so.conf
    let mut paths_to_process: Vec<PathBuf> = vec![
        Path::new("/etc/ld.so.conf").to_path_buf()
    ];
    let mut processed_paths: HashSet<PathBuf> = HashSet::new();

    while let Some(path) = paths_to_process.pop() {
        if processed_paths.contains(& path) {
            continue; // Skip if already processed
        }

        if path.is_file() {
            // Process single .conf file
            process_ld_conf_file(
                & path, &mut paths_to_process, &mut libdirs
            )?;
        } else {
            // Process *.conf glob
            for entry in glob(path.to_str().unwrap())? {
                match entry {
                    Ok(g_path) => {
                        process_ld_conf_file(
                            & g_path, &mut paths_to_process, &mut libdirs
                        )?;
                    }
                    Err(_e) => {}
                }
            }
        }

        processed_paths.insert(path);
    }

    // Add paths from LD_LIBRARY_PATH
    let ld_library_path = env::var("LD_LIBRARY_PATH").unwrap_or_default();
    let paths: Vec<String> = ld_library_path.split(":").map(
        |s| s.to_string()
    ).collect();
    libdirs.extend(paths);

    // Add paths from Cray compiler wrappers
    if add_cray {
        let cray_libdirs = cray_cc_libdirs(cmd)?;
        libdirs.extend(cray_libdirs);
    }

    Ok(libdirs)
}

pub fn filter_libs(
        paths: & Vec<String>, pattern: & str
    ) -> Result<Vec<String>, Box<dyn Error>> {

    let regex = Regex::new(pattern).unwrap();
    let mut libs = Vec::new();

    for path in paths {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if ! entry_path.is_file() { continue; }
                    if regex.is_match(entry_path.to_str().unwrap()) {
                        libs.push(entry_path.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }

    Ok(libs)
}