use std::time::SystemTime;

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
    println!("{}", a);
}
