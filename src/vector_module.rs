pub fn vector_fn() {
    // ベクタ型（要素の長さは可変、ただし要素の型は統一）
    let v1 = vec![1, 2, 3];
    let v1 = vec![0; 2];

    let mut v3 = Vec::new(); //空のベクタを作ること
    v3.push(100);
    v3.push(200);
    v3.push(300);
    println!("{:?}", v3);

    // ベクタから取り出す方法
    let vop = v3.pop(); //取り出すので既存からなくなる。型がOptionであることに注意。

    println!("{:?}", vop); //Some(300)
    println!("{:?}", v3); //[100, 200]
    let vopanother = v3.get(400); //400番目のindexの要素を取得してくる
    println!("{:?}", vopanother);
    println!("{:?}", v3); //[100, 200]
}
