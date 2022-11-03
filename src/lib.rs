//! ライブラリクレートのドキュメント（このコメントはライブラリ全体にかかるコメント

/// これはクレートにかかるコメント
pub fn say_hello() {
    println!("Hello Rust!")
    // binは相互に干渉はできないがlibは全てに干渉が可能
}

/// **say_good_by**関数のドキュメント
/// ### Usage
/// ```
/// fn main(){
///     rust_lesson_udemy::say_good_bye()
/// }
/// ````
pub fn say_good_bye() {
    println!("Good bye")
}

pub mod sample_trait {
    pub trait Shape {
        fn calc_area(&self) -> f64;
        fn calc_perimeter(&self) -> f64;
        fn do_something();
    }

    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }

    impl Shape for Rectangle {
        fn calc_area(&self) -> f64 {
            self.height * self.width
        }

        fn calc_perimeter(&self) -> f64 {
            self.height * 2.0 + self.width * 2.0
        }

        fn do_something() {
            println!("This is Rectangle func")
        }
    }

    pub struct Circle {
        pub raidus: f64,
    }

    impl Shape for Circle {
        fn calc_area(&self) -> f64 {
            self.raidus * self.raidus * std::f64::consts::PI
        }
        fn calc_perimeter(&self) -> f64 {
            self.raidus * 2.0 * std::f64::consts::PI
        }
        fn do_something() {
            println!("This is Circle func")
        }
    }
}
