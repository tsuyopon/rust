/*
 *  ベクタは構造体 Vec で表される可変サイズのリストです。
 *    cf. https://tourofrust.com/40_ja.html
 */
fn main() {

    // 型を明示的に指定
    let mut i32_vec = Vec::<i32>::new(); // turbofish <3
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);
    println!("{:?}", i32_vec);

    // もっと賢く、型を自動的に推論
    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);
    println!("{:?}", float_vec);

    // vec!はマクロです
    let string_vec = vec![String::from("Hello"), String::from("World")];

    // ベクタからイテレータを生成すれば、ベクタを簡単に for ループに入れることができます。
    for word in string_vec.iter() {
        println!("{}", word);
    }
}
