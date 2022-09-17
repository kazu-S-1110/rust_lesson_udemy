pub fn match_section() {
    let x = 0;
    match x {
        0 => println!("hoge"),
        1 => {
            println!("fuga");
            println!("piyo");
        }
        // match内にwildCardは必須
        _ => println!("other"),
    }

    // matchも式である
    let y = match x {
        0 => 0,
        1 => 10,
        _ => 100,
    };
}
