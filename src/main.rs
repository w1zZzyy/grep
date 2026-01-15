#[allow(dead_code)]

mod input;
mod output;

fn main() {
    let args = input::Arguments::new();
    match args {
        Ok(val) => println!("Arguments passed : {:?}", val),
        Err(err) => println!("Error occured {}", err)
    }
}
