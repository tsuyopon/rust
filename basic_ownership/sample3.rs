/*
 * 参照は、& 演算子を使ってリソースへのアクセスを借用できるようにしてくれます。
 */
struct Foo {
    x: i32,
}

fn main() {
    let foo = Foo { x: 42 };
    let f = &foo;
    println!("{}", f.x);

    // f はここでドロップ
    // foo はここでドロップ
}

