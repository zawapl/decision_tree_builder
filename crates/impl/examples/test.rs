fn main() {
    println!("{}", decide(&1));
}

pub fn decide(val: &usize) -> bool {
    return if *val < 4 {
        return if *val < 2 {
            # [tree_builder_uncertain_node (false = 1)]
            # [tree_builder_uncertain_node (true = 1)]
            true
        } else {
            false
        };
    } else {
        true
    };
}
