/*
 * 基本的な型
 */
fn main() {
    let x = 12;      // デフォルトでは i32
    let a = 12u8;    // 数値型は、数値の最後に型を付加することで明示的に指定できます
    let b = 4.3;     // デフォルトでは f64
    let c = 4.3f32;  // 数値型は、数値の最後に型を付加することで明示的に指定できます
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );
}

