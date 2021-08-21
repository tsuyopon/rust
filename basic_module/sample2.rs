/*
 * モジュールのネスト
 */
mod greet {
    pub fn hello() {
        println!("Hello world");
    }

    pub mod mod_a {
        pub fn hello() {
            println!("Hello world from A");
        }
    }

    pub mod mod_b {
        pub fn hello() {
            println!("Hello world from B");
        }
    }
}

fn main() {
    greet::hello();
    greet::mod_a::hello();
    greet::mod_b::hello();
}
