fn main() {
    println!("{}", decide(&3.0));
}

pub fn decide(val: &f32) -> &str {
    return if *val < 2.0 { "false" } else { "true" };
}
