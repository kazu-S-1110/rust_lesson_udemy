pub fn ref_section() {
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    // let s = concat_v1(s1, s2);
    // println!("{}", s);
    //v1の関数だと所有権が移動してしまっているので不可
    // println!("{}", s1);
    // println!("{}", s2);

    // v2なら参照を渡すので問題なし
    let s = concat_v2(&s1, &s2);
    println!("{}", s);
    println!("{}", s1);
    println!("{}", s2);
}

fn concat_v1(a: String, b: String) -> String {
    let c = format!("{},{}", a, b);
    c
}
fn concat_v2(a: &String, b: &String) -> String {
    let c = format!("{},{}", a, b);
    c
}
