/*
 * タプルライクな構造体
 */
struct Location(i32, i32);

fn main() {
    // これもスタックに入れられる構造体です。
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);
}

