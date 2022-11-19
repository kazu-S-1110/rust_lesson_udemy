use rust_lesson_udemy::sample_trait::{Circle, Rectangle, Shape};

fn main() {
    let rect = Rectangle {
        height: 4.0,
        width: 2.0,
    };

    let circle = Circle { raidus: 5.0 };

    println!("rect default is {}", rect.default_something());
    println!("circle default is {}", circle.default_something());
}
