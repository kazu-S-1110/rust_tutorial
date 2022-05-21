mod foo;

fn main() {
    println!("Hello, world!");
    foo::foo_fn();
    foo::bar::bar_fn();
}
