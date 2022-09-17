pub fn loop_section() {
    // loop breakするまで続ける
    let mut count = 0;
    loop {
        println!("Hello!");
        if count == 10 {
            break;
        };
        count += 1;
    }

    // while
    let mut count = 0;
    while count <= 10 {
        println!("Hello!");
        count += 1;
    }

    // for
    for i in [1, 2, 3] {
        println!("Hello, {}", i);
    }
}
