//Author: J. Pace <github.com/jpace121>
//Copyright: 2015
//License: http://opensource.org/licenses/BSD-2-Clause

// Make units we can enforce math with.
// Use enums? Or structs? Or phantom something or the other?

// Use a struct for the current status of the coaster.
// (Should this be more OOP-ish?)
struct CoasterStatus{
    m:f32, //mass in kg
    v:f32, //velocity in m/s
    h:f32, //height in m
    u:f32, //Potential energy in J
    k:f32 //Kinetic Energy in J
}

fn main() {
    // Set some current parameters.
    let mut status = CoasterStatus{
        m:10f32,
        v:10f32,
        h:0f32,
        u:0f32,
        k:0f32
    };

    status.k = get_kinetic(&status);
    status.u = get_potential(&status);
    println!("Velocity is {} m/s.",status.v );
    println!("Mass is {} kg", status.m);
    println!("Height is {} m", status.h);
    println!("");
    println!("Kinetic Energy: {}", status.k);
    println!("Potential Energy: {}", status.u);
}

fn get_kinetic(status:&CoasterStatus) -> f32{
    // Calculate the energy given the mass and the velocity
    // mass in kg, velocity in m/s, energy in J
    0.5*status.m*status.v
}

fn get_potential(status:&CoasterStatus) -> f32{
    // Calculate the potential energy, gven the mass and current height
    // from the zeroth point.
    let g = 9.81; // acceleration of gravity in m/s^2
    status.m*g*status.h
}
