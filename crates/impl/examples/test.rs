fn main() {
    println!("{}", decide(&1));
}

pub fn decide(val: &example::TestStructData) -> bool {
    return if val.b.a { false } else { true };
}
