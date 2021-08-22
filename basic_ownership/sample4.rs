/*
 * &mut 演算子を使えば、リソースへの変更可能なアクセスを借用することもできます。
 *   cf. https://tourofrust.com/49_ja.html
 */
struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f はここでドロップ
}

fn main() {

    let mut foo = Foo { x: 42 };
    let f = &mut foo;  // &mut 演算子を使えば、リソースへの変更可能なアクセスを借用できる

    // 失敗: do_something(foo) はここでエラー
    // foo は可変に借用されており移動できないため

    // 失敗: foo.x = 13; はここでエラー
    // foo は可変に借用されている間は変更できないため

    f.x = 13;
    // f はここから先では使用されないため、ここでドロップ
    
    // foo.xをf.xにするとエラーになる
    println!("{}", foo.x);
    
    // 可変な借用はドロップされているため変更可能
    foo.x = 7;
    
    // foo の所有権を関数に移動
    do_something(foo);
}

