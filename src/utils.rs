use std::cmp::Ordering;

pub fn split_data<T, F>(data: &mut [T], predicate: F) -> usize
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

#[cfg(test)]
mod tests {
    use crate::split_data;

    #[test]
    fn middle() {
        let mut data = [9, 1, 4, 8, 3, 7, 3, 1];
        let split = split_data(&mut data, |v| v < &5);
        assert_eq!(split, 5);
        assert_eq!(data, [1, 1, 4, 3, 3, 7, 8, 9]);
    }

    #[test]
    fn start() {
        let mut data = [9, 1, 4, 8, 3, 7, 3, 1];
        let split = split_data(&mut data, |v| v < &2);
        assert_eq!(split, 2);
        assert_eq!(data, [1, 1, 4, 8, 3, 7, 3, 9]);
    }

    #[test]
    fn end() {
        let mut data = [9, 1, 4, 8, 3, 7, 3, 1];
        let split = split_data(&mut data, |v| v < &9);
        assert_eq!(split, 7);
        assert_eq!(data, [1, 1, 4, 8, 3, 7, 3, 9]);
    }
}