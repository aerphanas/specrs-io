mod libs;
use libs::Ressolution;

use std::{
    io::{
        self,
        Write
    },
    process::exit
};

fn main() {
    let mut input = String::new();
    let width: u32;
    let height: u32;

    println!("Aspect Rasio Calculator");
    print!("input width x height : ");

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Error on input");

    let number: Vec<&str> = input.trim().split("x").collect();

    if number.len() != 2 {
        eprintln!("Please input valid parameter");
        exit(7);
    }

    if let Some(num) = number.get(0) {
        width = match num.parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Error : {e}");
                exit(4);
            }
        };
    } else {
        eprintln!("please input correct number");
        exit(1)
    }

    if let Some(num) = number.get(1) {
        height = match num.parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Error : {e}");
                exit(6);
            }
        };
    } else {
        eprintln!("please input correct number");
        exit(1)
    }

    let ress = Ressolution::new(width, height);

    println!("Aspect Ratio         : {}:{}", ress.get_ar().0, ress.get_ar().1);
    println!("Display Aspect Ratio : {}", ress.get_dar());
    println!("Full Pixel Count     : {}", ress.get_p_count());

}
