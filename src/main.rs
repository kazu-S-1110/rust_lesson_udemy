use rust_lesson_udemy::sample_trait::{Circle, Rectangle, Shape};

fn main() {
    let rect = Rectangle {
        height: 4.0,
        width: 2.0,
    };

    let circle = Circle { raidus: 5.0 };

    println!("rect area is {}", rect.calc_area());
    println!("circle area is {}", circle.calc_area());
}
