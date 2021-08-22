/*
 * 変数のサンプル
 */
fn main() {
    // x の型を推論
    let x = 13;
    println!("{}", x);

    // x の型を指定
    let x: f64 = 3.14159;
    println!("{}", x);

    // 宣言の後で初期化（あまり使われません）
    let x;
    x = 0;
    println!("{}", x);
}

