/*
 * std::any::type_nameを使って変数の型を出力する
 */
fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let a = 42;
    print_typename(a);
    let b = |x: i32| { x * 2 };
    print_typename(b);
    let c = (1..10).skip(2);
    print_typename(c);
}
