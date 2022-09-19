pub fn fizzbuzz(num: i32) -> String {
    let mut return_string;
    if num % 3 == 0 {
        return_string = "fizz";
    } else if num % 5 == 0 {
        return_string = "buzz"
    }

    return_string.to_string();
}
