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
}
