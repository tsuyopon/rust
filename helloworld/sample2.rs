// 同一ファイル内にモジュールを定義した簡単なサンプル
mod greet {
    pub fn hello() {
        println!("Hello world");
    }
}

fn main() {
    greet::hello();
}
