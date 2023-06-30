// Physics Engine 11 (https://github.com/Camerxxn/PEngine11)
// License: GNU GPL-V3
// File Purpose: Formulas for calculating specific values such as velocity, force, energy

//
// GENERAL EQUATIONS
//

// a^2 + b^2 = c^2
pub fn calc_hyp(a: f64, b: f64) -> f64 { 
    return ((a * a) + (b * b)).sqrt();
}



//
// KINEMATIC EQUATIONS
//

// vf = vi + at
pub fn calc_final_velocity_one(vi: f64, acc: f64, time: f64) -> f64 {
    vi + (acc * time)
}

// vf^2 = vi^2 + 2ad
pub fn calc_final_velocity_two(vi: f64, acc: f64, displacement: f64) -> f64 {
    ((vi * vi) + 2f64 * (acc * displacement)).sqrt()
}

// d = vi * t + 1/2(a*t^2)
pub fn calc_displacement_one(vi: f64, t: f64, acc: f64) -> f64 {
    (vi * t) + (0.5 * (acc * (t * t)))
}



//
// ENERGY EQUATIONS
//

// W = Fa * d
pub fn calc_work(fa: f64, displacement: f64) -> f64 {
    fa * displacement
}

// Epg = mgh
pub fn calc_gravitation_potential(mass: f64, gravity: f64, height: f64) -> f64 {
    mass * gravity * height
}

// Ek = 1/2 * m * v^2
pub fn calc_kinetic_energy(mass: f64, velocity: f64) -> f64 {
    0.5 * mass * (velocity * velocity)
}

// Eps = 1/2 * k * x^2 
pub fn calc_spring_potential(k_spring_constant: f64, delta_x: f64) -> f64 {
    0.5 * k_spring_constant * (delta_x * delta_x)
}



//
// FORCE EQUATIONS
//

// Fa = ma
pub fn calc_force_applied(mass: f64, acc: f64) -> f64 {
    mass * acc
}

// Fnet = F1 + F2 + F...
pub fn calc_net_force(forces: Vec<f64>) -> f64 {
    let mut net_force: f64 = 0f64;
    for force in forces {
        // a force can be negative so + (-) will subtract
        net_force += force;
    }
    net_force
}

pub fn calc_force_with_angle_cos(force: f64, angle: f64) -> f64 {
    force * (1f64 - angle.to_radians().cos())
}

pub fn calc_force_with_angle_sin(force: f64, angle: f64) -> f64 {
    force * (1f64 - angle.to_radians().sin())
}
