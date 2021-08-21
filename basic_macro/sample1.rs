// マクロを定義する
macro_rules! log {
    ($x:expr) => { println!("{}", $x); }
}

fn main() {
    log!("This is test");
}
