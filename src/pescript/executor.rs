// Physics Engine 11 (https://github.com/Camerxxn/PEngine11)
// License: GNU GPL-V3
// File Purpose: Parse a script and create an instruction set for the simulation 

use std::time::Instant;
use crate::engine::simulation::*;
use crate::engine::formulas::*;

#[derive(Debug)]
pub enum Token {
    PES, // Simulation entry
    CREATE, // Create command
    MODIFY, // Modify command
    TARGET, // Object, Let, or Const
    COMPONENT, // Component of target
    VARIABLE, // Variable of component
    VALUE, // New value for variable
    CONSTANT, // Constant variable
    LET, // Mutable variable
    EOC, // End of command
}

fn create_command(line: &str, simulation: &mut Simulation) {
    let mut line = line.split_whitespace();
    let _command = line.next().unwrap();
    let creation_type = line.next().unwrap();
    let identifier = line.next().unwrap().split(";").next().unwrap().to_string();

    match creation_type {
        // physics object
        "obj" => {
            simulation.objects.push(Object {
                name: identifier,
                position: Vec2 { x: 0f64, y: 0f64 },
                acceleration: Vec2 { x: 0f64, y: 0f64 },
                velocity: Vec2 { x: 0f64, y: 0f64 },
                force: Vec2 { x: 0f64, y: 0f64 },
                mass: 1f64,
                potential_energy: calc_gravitation_potential(1f64, simulation.gravity, 0f64),
                kinetic_energy: calc_kinetic_energy(1f64, 0f64),
                angle: 0f64 
            });
        },
        // var 
        "var" => {

        },
        // const
        _ => {

        }
    }
}

fn modify_command(line: &str, simulation: &mut Simulation) {
    let mut line = line.split_whitespace();
    let _command = line.next().unwrap();
    let target = line.next().unwrap();
    let component = line.next().unwrap();
    let variable = line.next().unwrap();
    let new_value = line.next().unwrap().split(";").next().unwrap().split_whitespace().next().unwrap().parse::<f64>().unwrap();

    for obj in simulation.objects.iter_mut() {
        if obj.name.eq_ignore_ascii_case(target) {
            match component {
                "velocity" => {
                    if variable.eq_ignore_ascii_case("x") {
                        obj.velocity.x = new_value;
                    } else {
                        obj.velocity.y = new_value;
                    }
                },
                "acceleration" => {
                    if variable.eq_ignore_ascii_case("x") {
                        obj.acceleration.x = new_value;
                    } else {
                        obj.acceleration.y = new_value;
                    }
                }
                _ => break
            }
        }
    } 
}

fn do_command(line: &str, simulation: &mut Simulation) {
    let mut line = line.split_whitespace();
    let _command = line.next().unwrap();
    let do_duration = line.next().unwrap().split(";").next().unwrap().parse::<f64>().unwrap();

    simulation_execute(simulation, do_duration);
}

fn print_command(line: &str, simulation: &mut Simulation) {
    let mut line = line.split_whitespace();
    let _command = line.next().unwrap();
    let target = line.next().unwrap();
    let component = line.next().unwrap();
    let variable = line.next().unwrap().split(";").next().unwrap();

    for obj in simulation.objects.iter_mut() {
        if obj.name.eq_ignore_ascii_case(target) {
            match component {
                "velocity" => {
                    if variable.eq_ignore_ascii_case("x") {
                        println!("{}.{}.x = {}", obj.name, component, obj.velocity.x);
                    } else {
                        println!("{}.{}.y = {}", obj.name, component, obj.velocity.y);
                    }
                },
                "position" => {
                    if variable.eq_ignore_ascii_case("x") {
                        println!("{}.{}.x = {}", obj.name, component, obj.position.x);
                    } else {
                        println!("{}.{}.y = {}", obj.name, component, obj.position.y);
                    }
                }
                _ => break
            }
        }
    }
}

pub fn run_script(input: String) {
    print!("\x1B[2J\x1B[1;1H");
    println!("Parsing Script...");
    let mut instructions: Vec<Token> = Vec::new();
    let mut simulation = Simulation::new();

    for line in input.lines() {
        for token in line.split(" ") {
            match token.to_lowercase().as_str() {
                "create" => {
                    instructions.push(Token::CREATE);
                    create_command(line, &mut simulation);
                },
                "modify" => {
                    instructions.push(Token::MODIFY);
                    modify_command(line, &mut simulation);
                },
                "do" => {
                    do_command(line, &mut simulation);
                },
                "print" => {
                    print_command(line, &mut simulation);
                }
                _ => {
                    
                } 
            }
        }
    }
}
