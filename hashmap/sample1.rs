/*
 * ハッシュマップは一般的に「連想配列」と呼ばれるものです。
 */
use std::collections::HashMap;

fn main() {
	let mut map = HashMap::new();
	map.insert("x", 10);
	map.insert("y", 20);
	map.insert("z", 30);
	println!("{} {} {}", map["x"], map["y"], map["z"]);

	for (k, v) in &map {
		println!("{} {}", k, v);
	}
}
