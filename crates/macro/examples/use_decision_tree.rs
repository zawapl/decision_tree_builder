use decision_impl::TreeBuilder;

pub fn decide(t: &TestData) -> bool {
    if t.a < 1usize {
        if t.b < 1usize {
            false
        } else {
            true
        }
    } else {
        true
    }
}

pub struct TestData {
    a: usize,
    b: usize,
    c: bool,
    d: bool,
}

fn main() {
    let test_data = TestData { a: 1, b: 1, c: true, d: true };
    println!("Decision: {}", decide(&test_data));
}
