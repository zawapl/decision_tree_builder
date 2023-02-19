use std::collections::HashMap;
use std::hash::Hash;

pub(crate) fn split_data<T, F>(data: &mut [T], predicate: F) -> usize
where F: Fn(&T) -> bool {
    let mut a = 0;
    let mut b = data.len() - 1;
    while a != b {
        while predicate(&data[a]) && a < b {
            a += 1;
        }
        while !predicate(&data[b]) && a < b {
            b -= 1;
        }
        data.swap(a, b);
    }

    return a;
}

pub(crate) fn h(count: usize, total: usize) -> f64 {
    if count == 0 {
        return 0.0;
    }
    let p = count as f64 / total as f64;
    let result = -p * p.log2();
    debug_assert!(!result.is_nan());
    return result;
}

pub(crate) fn entropy<T>(map: &HashMap<T, usize>) -> f64 {
    let counts: Vec<usize> = map.values().into_iter().cloned().collect();
    let total = counts.iter().sum();
    let mut result = 0.0;
    for i in counts {
        result += h(i, total);
    }
    return result;
}

pub(crate) fn to_counts<D, R: Eq + Hash + Copy>(data: &[(D, R)]) -> HashMap<R, usize> {
    let mut results = HashMap::new();

    for (_, res) in data.iter() {
        *results.entry(*res).or_insert(0) += 1;
    }

    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle() {
        let mut data = [9, 1, 4, 8, 3, 7, 3, 1];
        let split = split_data(&mut data, |v| v < &5);
        assert_eq!(split, 5);
        assert_eq!(data, [1, 1, 4, 3, 3, 7, 8, 9]);
    }

    #[test]
    fn test_start() {
        let mut data = [9, 1, 4, 8, 3, 7, 3, 1];
        let split = split_data(&mut data, |v| v < &2);
        assert_eq!(split, 2);
        assert_eq!(data, [1, 1, 4, 8, 3, 7, 3, 9]);
    }

    #[test]
    fn test_end() {
        let mut data = [9, 1, 4, 8, 3, 7, 3, 1];
        let split = split_data(&mut data, |v| v < &9);
        assert_eq!(split, 7);
        assert_eq!(data, [1, 1, 4, 8, 3, 7, 3, 9]);
    }

    #[test]
    fn test_entropy() {
        let map = HashMap::from([("A", 9), ("B", 5)]);
        assert_eq!(entropy(&map), 0.9402859586706311);
    }
}
