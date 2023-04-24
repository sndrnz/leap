use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::exit;
use std::{fs::File, io::Read};

const SHELL_HELPER: &str = r#"
l() {
    if result=$(%LEAP_PATH% $1) ; then
        cd $result
    else
        echo -n $result
        return 1
    fi
}
"#;

fn main() {
    // get file path
    let home_dir = dirs::home_dir().unwrap();
    let leaprc_path = home_dir.join(".leaprc");

    // if file not exists, create it
    if !leaprc_path.exists() {
        File::create(leaprc_path.clone()).unwrap_or_else(|_| {
            eprintln!("Cannot create config file: {}", leaprc_path.display());
            exit(1);
        });
    }

    // read file at ~/.leaprc
    let mut leaprc_file = File::open(leaprc_path.clone()).unwrap_or_else(|_| {
        eprintln!("Cannot open config file: {}", leaprc_path.display());
        exit(1);
    });
    let mut leaprc_content = String::new();

    leaprc_file.read_to_string(&mut leaprc_content).unwrap();

    // parse file: "<name> <path>\n"
    let leaprc_lines = leaprc_content.lines();

    let mut leaprc_map = HashMap::new();

    for line in leaprc_lines {
        if line.starts_with('#') {
            continue;
        }
        let line_split: Vec<&str> = line.splitn(2, ' ').collect();

        if line_split.len() != 2 {
            continue;
        }

        let name = line_split[0].to_string();
        let path = line_split[1].to_string();

        leaprc_map.insert(name, path);
    }

    // get command line arguments
    let args: Vec<String> = std::env::args().collect();

    // get command name
    let path_name = args[1].clone();

    if path_name == "shell" {
        println!(
            "{}",
            SHELL_HELPER.replace("%LEAP_PATH%", env::current_exe().unwrap().to_str().unwrap())
        );
        return;
    }

    // get command path
    let path = leaprc_map.get(&path_name).unwrap_or_else(|| {
        eprintln!("Name not found: {}", path_name);
        exit(1);
    });

    // check if path exists
    if !Path::new(path).exists() {
        eprintln!("Path doesn't exist: {}", path);
        exit(1);
    }

    // print command path
    println!("{}", path);
}
