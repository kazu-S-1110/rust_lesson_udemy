use string::string_fn;
mod array_module;
mod basic_struct_sec;
mod calc_module;
mod enum_sec;
mod fizzbuzz;
mod function;
mod if_section;
mod loop_section;
mod match_section;
mod optional_sec;
mod ref_section;
mod smart_pointer_section;
mod string;
mod sum;
mod tuple_module;
mod vector_module;

fn main() {
    // 絶対パスで指定
    // crate::tuple_module::tuple_fn();
    // 相対パスで指定（self::は省略できる）
    // array_module::array_fn();
    // vector_module::vector_fn();
    // string_fn();
    // if_section::if_section()
    // loop_section::loop_section()
    // smart_pointer_section::smart_pointer_section();
    // basic_struct_sec::basic_struct_sec()
    // enum_sec::enum_sec()
    // optional_sec::optional_sec()

    println!("{}", sum::sum(5));
    println!("{}", sum::sum(0));
    println!("{}", sum::sum(10));
}
