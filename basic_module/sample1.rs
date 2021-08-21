/*
 * モジュールのスコープ
 */
mod greet {
    pub fn hello() {
        println!("Hello world");
    }

    // pubで定義されていないので、外部からは呼び出せない
    fn private_hello() {
        println!("Private Hello world ");
    }
}

fn main() {
    greet::hello();
    //greet::private_hello(); // Compile Error
}
