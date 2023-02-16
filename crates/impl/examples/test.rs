fn main() {}

pub fn decide(t: &example::TestData) -> bool {
    if t.a < 1 {
        if t.b < 1 {
            false
        } else {
            true
        }
    } else {
        true
    }
}
