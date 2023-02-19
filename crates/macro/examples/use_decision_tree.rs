pub fn decide(val: &TestData) -> bool {
    return if val.b < 1 {
        if val.a < 1 {
            false
        } else {
            true
        }
    } else {
        if val.a < 1 {
            true
        } else {
            false
        }
    };
}

struct T {
    a: usize,
    b: B,
}

struct B();

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

fn foo(t: &T) {
    let a = t.a;
    let b = t.b;
}
