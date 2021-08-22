/*
 * 三項演算子を使ったifの記述
 */
fn example() -> i32 {
    let x = 42;
    let v = if x < 42 { -1 } else { 1 };
    println!("v: {}", v);
    return v
}

fn main() {
    println!("return: {}", example());
}

