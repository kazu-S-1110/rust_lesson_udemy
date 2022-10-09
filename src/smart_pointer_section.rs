use std::rc::Rc;

pub fn smart_pointer_section() {
    // 値を持った参照のことをスマートポインタという
    let x = Box::new(1);
    println!("x: {:p}", x);
    println!("x + 2 = {}", *x + 2);

    // 複数の参照に所有権を持たせることができるRc
    let a = Rc::new("hello".to_string());
    println!("count1: {}", Rc::strong_count(&a));
    {
        let b = Rc::clone(&a);
        println!("a: {:p}", a);
        println!("a is {}", *a);
        println!("b: {:p}", b);
        println!("count2: {}", Rc::strong_count(&a))
    }
}
