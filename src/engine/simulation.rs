use std::time::Instant;

use super::formulas::{calc_displacement_one, calc_kinetic_energy, calc_hyp, calc_gravitation_potential, calc_force_with_angle_cos};

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
    pub force: Vec2,
    pub position: Vec2,
    pub angle: f64,
    pub kinetic_energy: f64,
    pub potential_energy: f64,
    pub mass: f64,
}

impl Object {
    pub fn from_acceleration(&mut self) {
        self.force.x = calc_force_with_angle_cos(self.mass * self.acceleration.x, self.angle);
        self.force.y = calc_force_with_angle_cos(self.mass * self.acceleration.y, self.angle);
    }

    pub fn from_force(&mut self) {
        self.acceleration.x = calc_force_with_angle_cos(self.force.x, self.angle) / self.mass;
        self.acceleration.y = calc_force_with_angle_cos(self.force.y, self.angle) / self.mass;
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
        let mut start_loop = Instant::now().elapsed().as_secs_f64();
        let mut total_time: f64 = 0f64;
        while instant_now.elapsed().as_secs_f64() < time {
            let end_time = Instant::now().elapsed().as_secs_f64() - start_loop;
            total_time += end_time;
            for obj in simulation.objects.iter_mut() {
                obj.position.x += calc_displacement_one(obj.velocity.x, end_time, obj.acceleration.x); 
                obj.position.y += calc_displacement_one(obj.velocity.y, end_time, obj.acceleration.y);
            }
            start_loop = end_time;
        }
        println!("{}", total_time);
    } else {
        for obj in simulation.objects.iter_mut() {
            obj.position.x += calc_displacement_one(obj.velocity.x, time, obj.acceleration.x); 
            obj.position.y += calc_displacement_one(obj.velocity.y, time, obj.acceleration.y);
        }
    }
}
