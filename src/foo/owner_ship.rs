pub mod ownership {
    pub fn takes_ownership(some_string: String) {
        println!("takes ownership,{} (String)", some_string)
    }

    pub fn makes_copy(some_integer: i32) {
        println!("makes copy, {} (integer)", some_integer)
    }

    pub fn gives_ownership() -> String {
        let some_string = String::from("hello");
        some_string
    }
}
