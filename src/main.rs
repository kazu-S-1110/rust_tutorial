// #![feature(test)]
// extern crate test;
mod stack_heap;

fn main() {
    stack_heap::array_vector::run()
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
