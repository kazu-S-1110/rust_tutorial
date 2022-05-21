mod foo;

use crate::foo::fuga::fuga_func;
use foo::bar as hoge;

fn main() {
    println!("Hello, world!");
    foo::foo_fn();
    hoge::bar_fn();
    fuga_func();
}
