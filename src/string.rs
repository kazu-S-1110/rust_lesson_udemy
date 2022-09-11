pub fn string_fn() {
    // 文字型 シングルクォーテーションで囲むこと
    let c1: char = 'a';
    let c2 = '@';
    let c3 = '🌟';

    // 文字列型
    let s1 = "Rust"; //こう宣言したものは変更できないもの

    // 変更の可能性があるものは以下のように宣言する
    let s2 = String::from("Python");
    let s3 = "Java".to_string();

    // 実際に変更してみる
    let mut s4 = String::from("Hello");
    s4.push_str(", Rust");

    let s5 = format!("{}{}", s1, s2);
    println!("{}", s5); //RustPython
}
