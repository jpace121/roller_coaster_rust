//Author: J. Pace <github.com/jpace121>
//Copyright: 2015
//License: http://opensource.org/licenses/BSD-2-Clause

// Make units we can enforce math with.
// Use enums? Or structs? Or phantom something or the other?

// Use a struct for the current status of the coaster.
struct CoasterStatus{
    m:f32, //mass in kg
    v:f32, //velocity in m/s
    h:f32, //height in m
    U:f32, //Potential energy in J
    K:f32 //Kinetic Energy in J
}

fn main() {
    // Set some current parameters.
    let status = CoasterStatus{
        m:10f32,
        v:10f32,
        h:0f32,
        U:0f32,
        K:0f32
    };

    println!("Velocity is {} m/s.",status.v );
    println!("Mass is {} kg", status.m);
    println!("Height is {} m", status.h);
    println!("");
    println!("Kinetic Energy: {}", get_kinetic(&status));
    println!("Potential Energy: {}", get_potential(&status));
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
