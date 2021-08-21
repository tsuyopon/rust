mod greet; // greet.rs を module として読み込み

fn main() {
    greet::hello();
    greet::mod_a::hello();
    greet::mod_b::hello();
}
