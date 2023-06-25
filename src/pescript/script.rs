// Physics Engine 11 (https://github.com/Camerxxn/PEngine11)
// License: GNU GPL-V3
// File Purpose: Read scripts

use std::fs;

use crate::pescript::parser;

// Return all scripts with "./scripts"
pub fn load_scripts() -> Vec<String> {
    let paths = fs::read_dir("./scripts/").unwrap();
    let mut scripts: Vec<String> = Vec::new();
    for path in paths {
        scripts.push(path.unwrap().file_name().into_string().unwrap());
        println!("Found script!");
    }

    return scripts;
}

// Begin script reading
pub fn begin_script(script_file: &str) {
    let contents = fs::read_to_string(format!("./scripts/{}", script_file))
        .expect("Error: Couldn't read file!");

    parser::parse_script(contents);
}
