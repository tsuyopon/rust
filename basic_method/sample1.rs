/*
 * スタティックメソッドとインスタンスメソッドの例
 */
fn main() {

    // スタティックメソッドの場合は演算子::で呼び出せる。
    let s = String::from("Hello world!");

    // インスタンスメソッドは演算子.で呼び出せる
    println!("{} is {} characters long.", s, s.len());
}

