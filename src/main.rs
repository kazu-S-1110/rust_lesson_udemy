use string::string_fn;

mod array_module;
mod string;
mod tuple_module;
mod vector_module;

mod calc_module;
mod function;

fn main() {
    // 絶対パスで指定
    // crate::tuple_module::tuple_fn();
    // 相対パスで指定（self::は省略できる）
    // array_module::array_fn();
    // vector_module::vector_fn();
    string_fn();
}
