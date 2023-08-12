use std::process::Command;
use std::error::Error;


#[derive(Debug)]
struct CmdError {
    message: String
}

impl std::fmt::Display for CmdError {
    fn fmt(& self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CmdError {}", self.message)
    }
}

impl Error for CmdError {}


fn run_cc(cmd: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new(cmd)
        .arg("--cray-print-opts=all")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(& output.stdout);
                return Ok(stdout.to_string());
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let error = CmdError{
                    message: stderr.to_string()
                };
                return Err(Box::new(error));
            }
        }
        Err(e) => {
            return Err(Box::new(e))
        }
    }
}

pub fn cray_cc_libdirs(cmd: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let cc_output = run_cc(cmd)?;

    let parts: Vec<& str> =  cc_output.split_whitespace().collect();
    let mut libs: Vec<String> = Vec::new();
    for part in parts {
        if part.starts_with("-L") {
            let _lib: String = part[2..].to_string();
            libs.push(_lib);
        }
    }

    return Ok(libs);
}