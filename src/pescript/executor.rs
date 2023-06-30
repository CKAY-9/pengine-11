// Physics Engine 11 (https://github.com/Camerxxn/pengine-11)
// License: GNU GPL-V3
// File Purpose: Parse a script and create an instruction set for the simulation 

use crate::engine::simulation::*;
use crate::engine::formulas::*;

fn create_command(line: &str, simulation: &mut Simulation) {
    let mut line = line.split_whitespace();
    let _command = line.next().unwrap();
    let creation_type = line.next().unwrap();
    let identifier = line.next().unwrap().split(";").next().unwrap().to_string();

    println!("Creating {} with the identifier of {}", creation_type, identifier);

    match creation_type {
        // physics object
        "obj" => {
            simulation.objects.push(Object {
                name: identifier,
                position: Vec2 { x: 0f64, y: 0f64 },
                acceleration: Vec2 { x: 0f64, y: 0f64 },
                velocity: Vec2 { x: 0f64, y: 0f64 },
                force: 0f64,
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

    println!("Applying modification on {}.{}.{}, new value: {}", target, component, variable, new_value);

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

                    obj.from_acceleration();
                },
                "mass" => {
                    obj.mass = new_value;

                    obj.from_mass(simulation.gravity);
                },
                "force" => {
                    obj.force = new_value;
                    obj.from_force();
                },
                "angle" => {
                    obj.angle = new_value;
                },
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

    println!("\nDoing {}s of simulation\n", do_duration);
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
                    println!("{}.{}.{} = {}m/s", obj.name, component, variable, match variable {
                        "x" => obj.velocity.x,
                        _ => obj.velocity.y,
                    });
                },
                "position" => {
                    println!("{}.{}.{} = {}m", obj.name, component, variable, match variable {
                        "x" => obj.position.x,
                        _ => obj.position.y,
                    });
                },
                "acceleration" => {
                    println!("{}.{}.{} = {}m/s^2", obj.name, component, variable, match variable {
                        "x" => obj.acceleration.x,
                        _ => obj.acceleration.y
                    });
                },
                "force" => {
                    println!("{}.{}.{} = {}J", obj.name, component, variable, obj.force);
                },
                _ => break
            }
        }
    }

    println!("");
}

pub fn run_script(input: String) {
    print!("\x1B[2J\x1B[1;1H");
    let mut simulation = Simulation::new();

    println!("Running script/simulation...\n");

    for line in input.lines() {
        for token in line.split(" ") {
            match token.to_lowercase().as_str() {
                "create" => {
                    create_command(line, &mut simulation);
                },
                "overtime" => {
                    simulation.overtime = true;
                    println!("Enabling overtime simulation");
                },
                "modify" => {
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

    println!("Finished simulation!");
}
