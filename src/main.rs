mod foo;

use crate::foo::fuga::fuga_func;
use crate::foo::owner_ship::ownership::{
    change_string, dont_move_ownership, gives_ownership, makes_copy, takes_ownership,
};
use foo::bar as hoge;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    foo::foo_fn();
    hoge::bar_fn();
    fuga_func();

    enum Color {
        Red,
        Green,
        Blue,
    }
    let color = Color::Red;

    //tuple
    let tup = (10, 20, "30");
    println!("this is tuple result :{} {} {}", tup.0, tup.1, tup.2);

    //array
    let arr = [10, 20, 30];
    println!("this is array result :{} {} {}", arr[0], arr[1], arr[2]);

    //vector 要素数は可変
    let mut vect = vec![10, 20, 30];
    vect.push(40);
    println!("this is vector result : {} ", vect[0]);

    for v in &vect {
        println!("{}", v)
    }
    //hashmap オベジェクト的なやつ
    let mut map = HashMap::new();
    map.insert("x", "Hello");
    map.insert("y", "Rust");
    map.insert("z", "World");
    println!("this is hashmap result : {}", map["x"]);
    for (k, v) in &map {
        println!("key: {} , value: {}", k, v);
    }

    //ownership
    // stringは借用権が移動してしまう
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); //value borrowed here after move

    //integerは大丈夫
    let x = 5;
    makes_copy(x);
    // println!("{}", x); //ok

    //所有権は受け取ることも可能
    {
        let s1 = gives_ownership();
        println!("{}", s1)
    }

    let mut s1 = String::from("Rust");
    let len = dont_move_ownership(&s1); //参照渡しすることで所有権を渡さない(借用という)
    println!("the length of {} is {}", s1, len);

    change_string(&mut s1);
    println!("{}", s1);

    // 可変な参照を作成する際の注意点
    {
        let mut a = 10; // mutable object
        let a_mut_ref = &mut a; // mutable reference
        let a_mut_ref_move = a_mut_ref; // move mutable reference
                                        // print!("{}", a_mut_ref);        // borrow check!! - Error!
    }
    // 不変束縛変数から可変束縛変数に変える
    {
        let a = String::from("guhe");
        let mut b = a;
        b.push_str(" huge");
        println!("{}", b)
    }
}
