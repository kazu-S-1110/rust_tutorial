pub fn run() {
    //     配列でstack overflowを発生させてみる
    // let a1: [u8; 7000000] = [1; 7000000]; //7MBなのでこれは大丈夫

    // let a1: [u8; 9000000] = [1; 9000000]; //out

    //     可変長の配列vector
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    println!("stack address of v1 is : {:p}", &v1);
    println!("stack address of v2 is : {:p}", &v2);
    println!("heap memory address of v1 : {:?}", v1.as_ptr());
    println!("len of v1 is : {}", v1.len());
    println!("capacity of v1 is : {}", v1.capacity());
    v1.insert(1, 10);
    println!("{:?}", v1); //vec自体を表示したい時は{:?}を使う -> [1, 10, 2, 3, 4]
    v1.remove(0);
    println!("{:?}", v1); //[10, 2, 3, 4]
    v1.append(&mut v3); //vectorにほかのVecを追加する、追加したVecは移動する
    println!("{:?}", v1); //[10, 2, 3, 4, 9, 10]
    println!("{:?}", v3); //[]
}
