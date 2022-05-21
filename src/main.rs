mod foo;

use crate::foo::fuga::fuga_func;
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
}
