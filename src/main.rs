use std::io;
use rand::{thread_rng, Rng};

fn main() {
    println!("Devine mon nombre !");

    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).unwrap();
    //* .expect("Err read_line") *//
    
    let number: u32 = thread_rng().gen_range(0..101);
    let input_u32: u32 = input_str.trim().parse().unwrap();
    match input_u32 {
        number => { println!("Felicitation !"); }
        input_u32 if number < input_u32 => { println!("Trop grand !"); }
        input_u32 if number > input_u32 => { println!("Trop petit !"); }
        _ => { println!("Error !"); }
    }
}