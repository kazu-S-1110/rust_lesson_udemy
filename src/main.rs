fn main() {
    let t1 = (1,true, 2.4);
    let t2 = (2.4, "hoge",true);

    println!("{:?}",t1);
    println!("{:?}",t2);

    // 配列のindexから要素を取り出す
    let i = t1.0;
    println!("{}",i);

    // 配列から要素を取り出す方法
    let (x,_,z ) = t1;

}
