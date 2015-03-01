//Author: J. Pace <github.com/jpace121>
//Copyright: 2015
//License: http://opensource.org/licenses/BSD-2-Clause

// Make units we can enforce math with.
// Use enums? Or structs? Or phantom something or the other?

fn main() {
    // Set some current parameters.
    let v:f32 = 10f32; // m/s
    let m:f32 = 10f32; // kg
    let h:f32 = 0f32; // m

    println!("Velocity is {} m/s.", v);
    println!("Mass is {} kg", m);
    println!("Height is {} m", h);
    println!("");
    println!("Kinetic Energy: {}", get_kinetic(m,v));
    println!("Potential Energy: {}", get_potential(m,h));
}

fn get_kinetic(mass:f32, velocity:f32) -> f32{
    // Calculate the energy given the mass and the velocity
    // mass in kg, velocity in m/s, energy in J
    0.5*mass*velocity
}

fn get_potential(mass:f32, height:f32) -> f32{
    // Calculate the potential energy, gven the mass and current height
    // from the zeroth point.
    let g = 9.81; // accelearation of gravity in m/s^2
    mass*g*height
}
