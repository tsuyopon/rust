/*
 * traitの簡単なサンプルです。
 * traitは他の言語でのインターフェイスに似た仕組みです。
 */
struct Rect { width: u32, height: u32 }

// traitは構造体が実装すべきメソッドを定義します
trait Printable { 
	fn print(&self);
}

impl Printable for Rect {
    fn print(&self) {
        println!("width:{}, height:{}", self.width, self.height)
    }
}

fn main() {
    let r = Rect { width: 200, height: 300 };
    r.print();
}
