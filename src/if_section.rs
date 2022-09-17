pub fn if_section() {
    let x = 4;

    if x > 0 {
        println!("OK");
    }

    // ifの中身はbooleanで渡すこと
    if x > 0 && x < 10 {
        println!("hoge")
    }
    if x > 0 || x < 10 {
        println!("fuga")
    }

    // ifは式であるため値を返すことが可能、その場合必ずelseが必要(型も揃える)
    let y = if x > 10 { x } else { 0 };
}
