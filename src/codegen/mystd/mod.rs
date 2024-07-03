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

pub extern fn print(a: f64) {
    println!("{}", a);
}
