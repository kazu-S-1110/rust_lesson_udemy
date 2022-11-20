use rust_lesson_udemy::sample_trait::{double_area, Circle, Rectangle, Shape};

fn main() {
    let rect = Rectangle {
        height: 4.0,
        width: 2.0,
    };

    let circle = Circle { raidus: 5.0 };

    println!("rect double area is {}", double_area(&rect));
    println!("circle double area  is {}", double_area(&circle));
}
