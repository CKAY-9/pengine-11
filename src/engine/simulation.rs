use super::formulas::{calc_final_velocity_one, calc_displacement_one};

pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

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

pub struct Variable {
    pub name: String,
    pub value: f64,
    pub constant: bool
}

pub struct Simulation {
    pub objects: Vec<Object>,
    pub variables: Vec<Variable>,
    pub gravity: f64,
    pub runtime: f64,
    pub should_simulate: bool,
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            objects: Vec::new(),
            variables: Vec::new(),
            gravity: 9.81,
            runtime: 10f64,
            should_simulate: false,
        }
    }
}

pub fn simulation_execute(simulation: &mut Simulation, time: f64) {
    for obj in simulation.objects.iter_mut() {
        obj.position.x += calc_displacement_one(obj.velocity.x, time, obj.acceleration.x); 
    }
}
