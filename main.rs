use std::env;

struct Force {
    i: f64,
    j: f64,
}

fn mode(mut angle: f64) -> u32{
    let mut counter = 0;
    while angle > 90.0
    {
        angle += - 90.0;
        counter += 1;
    }
    return counter;
}

fn calc(force: f64, theta: f64, is: f64, js: f64) -> Force {
    println!("{}", theta);
    let f = Force{
        i: is*force*theta.sin(),
        j: js*force*theta.cos(), 
    };
    return f;
}

fn split(force: f64, mut theta: f64) -> Force {
    let pi = std::f64::consts::PI;
    let mode = mode(theta);
    let angle = theta%90.0*pi/180.0;
    let mut f = Force{
        i: 0.0,
        j: 0.0, 
    };
    match mode {
        0 => (f = calc(force, angle, 1.0, 1.0)),
        1 => (f = calc(force, angle, -1.0, 1.0)),
        2 => (f = calc(force, angle, -1.0, -1.0)),
        3 => (f = calc(force, angle, 1.0, -1.0)),
        _ => (println!("how did u get here..."))
    }
    return f;
}

fn main() {
    let args: Vec<String> = env::args.collect()
    let f = split(args[1] as float, args[2] as float);
    println!("i: {}, j: {}", f.i, f.j);
}
