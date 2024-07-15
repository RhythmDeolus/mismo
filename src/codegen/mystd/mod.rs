use std::{fs::OpenOptions, time::SystemTime};
use std::io::prelude::*;

pub extern fn print_time() {
    let t =  SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    match t {
        Ok(x) => {
            println!("{:?}", x);
        }
        Err(x) => {
            println!("{}", x);
        }
    }
}

pub extern fn get_time() -> f64 {
    let t =  SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    match t {
        Ok(x) => {
            x.as_secs_f64()
        }
        Err(_) => {
            panic!("oh oh")
        }
    }
}

pub extern fn print(a: f64) {
    let mut file = OpenOptions::new()
        .append(true)
        .open("./tests/temp.out.txt")
        .unwrap();
    let x = writeln!(file, "{}", a);
    println!("{:?}", x);
    println!("{}", a);
}
