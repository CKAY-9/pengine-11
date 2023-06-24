// Physics Engine 11 (https://github.com/Camerxxn/PEngine11)
// License: GNU GPL-V3
// File Purpose: Primary entry for PEngine 11 

use std::io;

pub mod engine;
pub mod pescript;

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    let scripts = pescript::script::load_scripts();
    println!("Loaded scripts!");

    for _ in 0..10 {
        println!("");
    }

    println!("Avaliable Scripts:");

    let mut input = String::new();
    for i in 0..scripts.len() {
        println!("{}: {:?}", i, scripts.get(i).unwrap());
    }

    io::stdin().read_line(&mut input).expect("Error: Couldn't read user input!");
    let selected_script: usize = input.split_whitespace().next().unwrap().parse().unwrap();

    pescript::script::begin_script(&scripts.get(selected_script).unwrap());
}
