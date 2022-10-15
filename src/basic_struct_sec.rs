// structはClassと似てる感じ
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 型関連関数（constructorと同じ感じ？）慣習的にnewの名前が多い
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

pub fn basic_struct_sec() {
    let rectangle = Rectangle::new(10, 5);
}
