/*
 * 基本型の変換
 */
fn main() {
    let a = 13u8;
    let b = 7u32;

    // 数値型を扱う際にはu8とu32を混ぜるとエラーになります。
    // Rust は as キーワードで数値型を簡単に変換できます。
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
}

