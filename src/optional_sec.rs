pub fn optional_sec() {
    // NullというものはRustにはないため、Optionを使う
    // 型推論が基本効くが初期値が無い場合は型を明示する
    let a = Some(1);
    let b = Some("str");
    let c: Option<i32> = None;

    let v = vec![1, 2, 3];
    // get関数はoptinalを返す（あれば数値、なければnone）
    let val = v.get(2);

    // 値を参照するにはtypeGuard的なものが必要
    match val {
        Some(x) => println!("value exists:{}", x),
        None => println!("value is None"),
    }

    // if letでもアクセス可能
    if let Some(x) = val {
        println!("val = {}", x)
    }
}
