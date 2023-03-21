use std::{io::{self, Write}, process::exit};

use gcd::Gcd;

struct Ressolution {
    width: u32,
    height: u32,
    gcd: u32
}

impl Ressolution {

    fn new(width: u32, height: u32) -> Ressolution {
        let gcd_calc: u32 = width.gcd_euclid(height);
        Ressolution {
            width,
            height,
            gcd: gcd_calc
        }
    }

    fn get_dar(&self) -> f32 {
        (self.width / self.height) as f32
    }

    fn get_ar(&self) -> (u32, u32) {
        let width = self.width / self.gcd;
        let height = self.height / self.gcd;
        (width, height)
    }

    fn get_p_count(&self) -> u32 {
        self.width * self.height
    }

}

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
        .expect("");

    let test: Vec<&str> = input.trim().split("x").collect();

    if let Some(num) = test.get(0) {
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

    if let Some(num) = test.get(1) {
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
