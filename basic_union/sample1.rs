union MyUnion {
    f1: u32,
    f2: u32,
}

fn main() {
    let u = MyUnion { f1: 123 };
    unsafe {
        println!("{}", u.f1);
        println!("{}", u.f2);	// メモリを共用しているのでこちらも123と表示される
    }
}
