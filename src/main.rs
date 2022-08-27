fn main() {
    let t1 = (1,true, 2.4);
    let t2 = (2.4, "hoge",true);

    println!("{:?}",t1);
    println!("{:?}",t2);

    // タプルのindexから要素を取り出す
    let i = t1.0;
    println!("{}",i);

    // タプルから要素を取り出す方法
    let (x,_,z ) = t1;


    // 配列は要素は固定にする
    let l1 = [1,2,3];
    let l2 = [0;1000]; //0の要素が1000個の配列が作られる
    // println!("{:?}",l2);

    // 配列から要素を取り出す
    let i = l1[2];
    let [x,_,z] = l1;

    let l3 = &l1[0..2];//これで新しい配列を作成できる、ドットの最後は含まれないので注意
    println!("{:?}",l3);
    let l4 = &l1[0..=2];//これで含まれる
    println!("{:?}",l4);
    let l5 = &l1[..2];//0から取得したい場合は省略可能
    println!("{:?}",l5);
    let l6 = &l1[..];//最後まで取得したい場合も省略可能
    println!("{:?}",l6);

    // ベクタ型（要素の長さは可変、ただし要素の型は統一）
    let v1 = vec![1,2,3];
    let v1 = vec![0;2];

    let mut v3 = Vec::new(); //空のベクタを作ること
    v3.push(100);
    v3.push(200);
    v3.push(300);
    println!("{:?}",v3);

    // ベクタから取り出す方法
    let vop = v3.pop(); //取り出すので既存からなくなる。型がOptionであることに注意。
    println!("{:?}",vop); //Some(300)
    println!("{:?}",v3); //[100, 200]
    let vopanother = v3.get(400); //400番目のindexの要素を取得してくる
    println!("{:?}",vopanother);
    println!("{:?}",v3); //[100, 200]

}
