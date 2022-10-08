pub fn fizzbuzz(num: i32) {
    if num % 3 == 0 && num % 5 == 0 {
        println!("FizzBuzz")
    } else if num % 3 == 0 {
        println!("Fixx")
    } else if num % 5 == 0 {
        println!("Buzz")
    } else {
        println!("{}", num)
    }
}

pub fn fizzbuzz_v2(num: i32) {
    let r = 1..=num;

    for x in r {
        match (x % 3, x % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", x),
        }
    }
}
