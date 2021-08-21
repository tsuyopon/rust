struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 100, y: 200 };
    println!("{} {}", p.x, p.y);
}
