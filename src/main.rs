use std::env;

use rust_lesson_udemy::sample_trait::{Circle, Rectangle, Shape};

fn main() {
    let args: Vec<String> = env::args().collect();
    let sum: i32 = args[1].parse::<i32>().unwrap() + args[2].parse::<i32>().unwrap();

    println!("sum is {}", sum);

    let rect = Rectangle {
        height: 4.0,
        width: 2.0,
    };

    let circle = Circle { raidus: 5.0 };

    println!("rect area is {}", rect.calc_area());
    println!("circle area is {}", circle.calc_area());
}
