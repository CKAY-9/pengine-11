// Physics Engine 11 (https://github.com/Camerxxn/PEngine11)
// License: GNU GPL-V3
// File Purpose: Handle physics simulation 

use std::time::{Instant, Duration};

use super::formulas::{calc_displacement_one, calc_kinetic_energy, calc_hyp, calc_gravitation_potential, calc_force_with_angle_cos, calc_force_with_angle_sin};

#[derive(Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone)]
pub struct Object {
    pub name: String,
    pub velocity: Vec2, 
    pub acceleration: Vec2,
    pub force: f64,
    pub position: Vec2,
    pub angle: f64,
    pub kinetic_energy: f64,
    pub potential_energy: f64,
    pub mass: f64,
}

impl Object {
    pub fn from_acceleration(&mut self) {
        self.force = calc_hyp(self.mass * self.acceleration.x, self.mass * self.acceleration.y);
    }

    pub fn from_force(&mut self) {
        self.acceleration.x = calc_force_with_angle_sin(self.force, self.angle) / self.mass;
        self.acceleration.y = calc_force_with_angle_cos(self.force, self.angle) / self.mass;
    }

    pub fn from_mass(&mut self, gravity: f64) {
        self.kinetic_energy = calc_kinetic_energy(self.mass, calc_hyp(self.velocity.x, self.velocity.y));
        self.potential_energy = calc_gravitation_potential(self.mass, gravity, self.position.y);
    }
}

#[derive(Clone)]
pub struct Variable {
    pub name: String,
    pub value: f64,
    pub constant: bool
}

#[derive(Clone)]
pub struct Simulation {
    pub objects: Vec<Object>,
    pub variables: Vec<Variable>,
    pub gravity: f64,
    pub runtime: f64,
    pub should_simulate: bool,
    pub overtime: bool,
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            objects: Vec::new(),
            variables: Vec::new(),
            gravity: 9.81,
            runtime: 10f64,
            should_simulate: false,
            overtime: false
        }
    }
}

pub fn simulation_execute(simulation: &mut Simulation, time: f64) {
    if simulation.overtime {
        let instant_now = Instant::now();
        while instant_now.elapsed().as_secs_f64() < time { 
            for obj in simulation.objects.iter_mut() {
                obj.position.x = calc_displacement_one(obj.velocity.x, instant_now.elapsed().as_secs_f64(), obj.acceleration.x); 
                obj.position.y = calc_displacement_one(obj.velocity.y, instant_now.elapsed().as_secs_f64(), obj.acceleration.y);
            }
        }

    } else {
        for obj in simulation.objects.iter_mut() {
            obj.position.x += calc_displacement_one(obj.velocity.x, time, obj.acceleration.x); 
            obj.position.y += calc_displacement_one(obj.velocity.y, time, obj.acceleration.y);
        }
    }
}
