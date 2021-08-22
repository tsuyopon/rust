/*
 * 変数の変更
 */
fn main() {

    // 可変値は mut キーワードで表します。mutを指定しないと不変とみなします。
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);
}

