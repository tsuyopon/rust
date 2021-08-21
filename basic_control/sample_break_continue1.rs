/*
 * breakとcontinueを使った非常に単純なサンプルです
 */
fn main() {
	let mut n = 0;
	loop {
		n += 1;
		if n == 2 {
			continue;
		}
		if n == 8 {
			break;
		}
		println!("{}", n);
	}
}
