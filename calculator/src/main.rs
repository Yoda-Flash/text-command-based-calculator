use std::env::args;
use std::ffi::CString;
use std::io::{stderr, stdout};

extern crate argparse;
use argparse::{ArgumentParser, Store, StoreTrue};
use num_traits::{pow, Pow};
// x: base number, y: number operated on it

fn plus(x: f64, y: f64) -> f64 {
    return (x + y);
}

fn minus(x: f64, y: f64) -> f64 {
    return (x - y);
}

fn multiply(x: f64, y: f64) -> f64 {
    return (x * y);
}

fn divide(x: f64, y: f64) -> f64 {
    return (x / y);
}

fn power_of(base: f64, exp: f64) -> f64 {
    return pow(base, exp as usize);
}

fn root_of(base: f64, power: f64) -> f64 {
    return base.pow(1.0/power);
}

fn log(x: f64, base: f64) -> f64 {
    return x.log(base);
}

fn sine(x: f64) -> f64 {
    return x.sin();
}

fn cosine(x: f64) -> f64 {
    return x.cos();
}

fn tangent(x: f64) -> f64 {
    return x.tan();
}

fn arcsine(x: f64) -> f64 {
    return x.asin();
}

fn arccosine(x: f64) -> f64 {
    return x.acos();
}

fn arctangent(x: f64) -> f64 {
    return x.atan();
}

fn degrees_to_radians(x: f64) -> f64 {
    return x.to_radians();
}

fn radians_to_degrees(x: f64) -> f64 {
    return x.to_degrees();
}


fn main() {
    let mut base_number = 0.0;
    let mut addend = 0.0;
    let mut subtrahend = 0.0;
    let mut multiplier = 1.0;
    let mut divisor = 1.0;
    let mut exponent = 1.0;
    let mut root = 1.0;
    let mut log_base = 0.0;
    let mut sin = false;
    let mut cos = false;
    let mut tan = false;
    let mut arcsin = false;
    let mut arccos = false;
    let mut arctan = false;
    let mut deg_to_rad = false;
    let mut rad_to_deg = false;

    let mut operations: Vec<String> = vec![];
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("This is a simple command line app that acts as a scientific calculator, operating through commands instead of buttons.");
        parser.refer(&mut base_number)
            .add_argument(&"base_number", Store, &"The base operand. Other operations will be performed on this number.");
        parser.refer(&mut addend)
            .add_option(&["-p", "--plus"], Store, &"Add to the base number.");
        parser.refer(&mut subtrahend)
            .add_option(&["-s", "--subtract-by"], Store, &"Subtract from the base number.");
        parser.refer(&mut multiplier)
            .add_option(&["-m", "--multiply-by"], Store, &"Multiply with the base number.");
        parser.refer(&mut divisor)
            .add_option(&["-d", "--divide-by"], Store, &"Divide from the base number.");
        parser.refer(&mut exponent)
            .add_option(&["-x", "--exponent"], Store, &"Raise the base number to an exponent.");
        parser.refer(&mut root)
            .add_option(&["-r", "--root"], Store, &"Take a root of the base number.");
        parser.refer(&mut log_base)
            .add_option(&["-l", "--log"], Store, &"Take the logarithm of a base on the base number.");
        parser.refer(&mut sin)
            .add_option(&["--sin"], StoreTrue, &"Apply sine to the base number.");
        parser.refer(&mut cos)
            .add_option(&["--cos"], StoreTrue, &"Apply cosine to the base number.");
        parser.refer(&mut tan)
            .add_option(&["--tan"], StoreTrue, &"Apply tangent to the base number.");
        parser.refer(&mut arcsin)
            .add_option(&["--arcsin"], StoreTrue, &"Apply arcsin to the base number.");
        parser.refer(&mut arccos)
            .add_option(&["--arccos"], StoreTrue, &"Apply arccos to the base number.");
        parser.refer(&mut arctan)
            .add_option(&["--arctan"], StoreTrue, &"Apply arctan to the base number.");
        parser.refer(&mut deg_to_rad)
            .add_option(&["--degrees-to-radians"], StoreTrue, &"Convert the base number from degrees to radians.");
        parser.refer(&mut rad_to_deg)
            .add_option(&["--radians-to-degrees"], StoreTrue, &"Convert the base number from radians to degrees.");
        parser.parse_args_or_exit();
    }
    if (addend!=0.0) {
        println!("{}", plus(base_number, addend));
    }
    else if (subtrahend!=0.0) {
        println!("{}", minus(base_number, subtrahend));
    }
    else if (multiplier!=1.0) {
        println!("{}", multiply(base_number, multiplier));
    }
    else if (divisor!=1.0){
        println!("{}", divide(base_number, divisor));
    }
    else if (exponent!=1.0){
        println!("{}", power_of(base_number, exponent));
    }
    else if (root!=1.0){
        println!("{}", root_of(base_number, root));
    }
    else if (log_base!=0.0) {
        println!("{}", log(base_number, log_base));
    }
    else if sin {
        println!("{}", sine(base_number));
    }
    else if cos {
        println!("{}", cosine(base_number));
    }
    else if tan {
        println!("{}", tangent(base_number));
    }
    else if arcsin {
        println!("{}", arcsine((base_number)));
    }
    else if arccos {
        println!("{}", arccosine(base_number));
    }
    else if arctan {
        println!("{}", arctangent(base_number));
    }
    else if deg_to_rad {
        println!("{}", degrees_to_radians(base_number));
    }
    else if rad_to_deg {
        println!("{}", radians_to_degrees(base_number));
    }

}
