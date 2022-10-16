enum Shape {
    Circle,
    Square(u32),
    Triangle { base: u32, height: u32 },
}

impl Shape {
    fn sample_metod(&self) {
        println!("call sample method");
    }
}

pub fn enum_sec() {
    let c = Shape::Circle;
    let s = Shape::Square(4);
    let t = Shape::Triangle {
        base: 34,
        height: 43,
    };

    c.sample_metod()
}
