/*
 * for文のサンプル
 */
fn main() {

    // ..演算子は、0から4まで
    for x in 0..5 {
        println!("{}", x);
    }

    // ..=演算子は、0から5まで
    for x in 0..=5 {
        println!("{}", x);
    }
}
