// #![feature(test)]
// extern crate test;

mod foo;
mod stack_heap;

fn main() {
    // println!("Hello, world!");
    // foo::foo_fn();
    // hoge::bar_fn();
    // fuga_func();
    //
    // enum Color {
    //     Red,
    //     Green,
    //     Blue,
    // }
    // let color = Color::Red;
    //
    // //tuple
    // let tup = (10, 20, "30");
    // println!("this is tuple result :{} {} {}", tup.0, tup.1, tup.2);
    //
    // //array
    // let arr = [10, 20, 30];
    // println!("this is array result :{} {} {}", arr[0], arr[1], arr[2]);
    //
    // //vector 要素数は可変
    // let mut vect = vec![10, 20, 30];
    // vect.push(40);
    // println!("this is vector result : {} ", vect[0]);
    //
    // for v in &vect {
    //     println!("{}", v)
    // }
    // //hashmap オベジェクト的なやつ
    // let mut map = HashMap::new();
    // map.insert("x", "Hello");
    // map.insert("y", "Rust");
    // map.insert("z", "World");
    // println!("this is hashmap result : {}", map["x"]);
    // for (k, v) in &map {
    //     println!("key: {} , value: {}", k, v);
    // }
    //
    // //ownership
    // // stringは借用権が移動してしまう
    // let s = String::from("hello");
    // takes_ownership(s);
    // // println!("{}", s); //value borrowed here after move
    //
    // //integerは大丈夫
    // let x = 5;
    // makes_copy(x);
    // // println!("{}", x); //ok
    //
    // //所有権は受け取ることも可能
    // {
    //     let s1 = gives_ownership();
    //     println!("{}", s1)
    // }
    //
    // let mut s1 = String::from("Rust");
    // let len = dont_move_ownership(&s1); //参照渡しすることで所有権を渡さない(借用という)
    // println!("the length of {} is {}", s1, len);
    //
    // change_string(&mut s1);
    // println!("{}", s1);
    //
    // // 可変な参照を作成する際の注意点
    // {
    //     let mut a = 10; // mutable object
    //     let a_mut_ref = &mut a; // mutable reference
    //     let a_mut_ref_move = a_mut_ref; // move mutable reference
    //                                     // print!("{}", a_mut_ref);        // borrow check!! - Error!
    // }
    // // 不変束縛変数から可変束縛変数に変える
    // {
    //     let a = String::from("guhe");
    //     let mut b = a;
    //     b.push_str(" huge");
    //     println!("{}", b)
    // }
    //
    // for k in 0..10 {
    //     if k % 2 == 0 {
    //         for i in 0..10 {
    //             if i % 2 == 0 {
    //                 print!("{}", 0)
    //             } else {
    //                 print!("{}", 8)
    //             }
    //         }
    //     } else {
    //         for i in 0..10 {
    //             if i % 2 != 0 && k % 2 != 0 {
    //                 print!("{}", 0)
    //             } else {
    //                 print!("{}", 8)
    //             }
    //         }
    //     }
    //     println!()
    // }
    //
    // let cs: &str = "apple";
    // fn double_check(some_string: &str) {
    //     for string in some_string.split("") {
    //         print!("{ }", string);
    //     }
    // }
    // double_check(cs);
    stack_heap::stack_heap::run()
}

// #[cfg(test)]
// mod tests {
//     use crate::foo::fibo;
//     use test::Bencher;
//
//     #[bench]
//     fn bench_fibo(b: &mut Bencher) {
//         b.iter(|| fibo());
//     }
// }
