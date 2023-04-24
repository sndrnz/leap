use std::env;
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

    let mut leaprc_map = std::collections::HashMap::new();

    for line in leaprc_lines {
        let mut line_split = line.split_whitespace();
        let name = line_split.next().unwrap();
        let path = line_split.next().unwrap();

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
    let path = leaprc_map.get(path_name.as_str()).unwrap_or_else(|| {
        eprintln!("Path not found: {}", path_name);
        exit(1);
    });

    // print command path
    println!("{}", path);
}
