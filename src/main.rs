use waterdrop_simulation::rk4::RK4; 
use waterdrop_simulation::bubble::Bubble;

use std::fs::File; 
use std::io::{BufWriter, Write}; 


fn main() {
    let bubble = Bubble {
        rho: 1000.0, 
        sigma: 0.072, 
        mu: 1.0e-3, 
        p_inf: 101_325.0, 
        r0: 1.0e-3, 
        p0: 101_325.0, 
        gamma: 1.4, 

    }; 

    let mut state = vec![
        1.01e-3, //init radial
        0.0, 
    ]; 

    let mut rk4 = RK4::new(2); 
    //time
    let dt = 1.0e-5; 
    let mut t = 0.0; 
    //file
    let file = File::create("bubble.txt").expect("cannot create file"); 
    let mut writer = BufWriter::new(file); 
    //integral
    for _ in 0..500_000 {
        rk4.step(&bubble, t, dt, &mut state); 
        t += dt; 

        writeln!(writer, "{:6e}", state[0]).unwrap()
    }
}
