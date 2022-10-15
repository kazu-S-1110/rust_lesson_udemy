// structはClassと似てる感じ
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn basic_struct_sec() {
    // 事前に値を宣言してインスタンス作成の時に入れ込むことも可能
    let height = 6;

    // インスタンスの作成
    let rentangle = Rectangle { width: 10, height };

    println!("width: {}", rentangle.width);
    println!("height: {}", rentangle.height);

    // mutをつけて可変にすることも可能
    let mut rentangle2 = Rectangle { width: 10, height };
    rentangle2.height = 10;
    println!("height: {}", rentangle2.height);

    println!("area: {}", rentangle.area())
}
