/*
 * 関数の単純なサンプル
 */

// 関数名はsnake_caseを利用します。

// returnを記述する場合
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

// returnを記述しない場合は、最後の式が戻り値となります
fn add2(x: i32, y: i32) -> i32 {
    x + y	// セミコロン(;)無し
}

fn main() {
	println!("{}", add(1, 2));
	println!("{}", add2(10, 20));
}
