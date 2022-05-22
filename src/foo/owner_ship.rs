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

    // 借用を行う（所有権を移動しない）
    pub fn dont_move_ownership(s: &str) -> usize {
        s.len()
    }

    //可変にして参照を受け取る
    pub fn change_string(some_string: &mut String) {
        some_string.push_str(" is Best!");
    }
}
