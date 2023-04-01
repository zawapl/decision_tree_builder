use std::cmp::Ordering;

pub struct DecisionEval {
    pub(crate) gain_ratio: f64,
    pub(crate) max_branch_width: usize,
}

impl Eq for DecisionEval {}

impl PartialEq<Self> for DecisionEval {
    fn eq(&self, other: &Self) -> bool {
        return ((self.gain_ratio == other.gain_ratio)
            || (self.gain_ratio.is_nan() && other.gain_ratio.is_nan()))
            && (self.max_branch_width == self.max_branch_width);
    }
}

impl PartialOrd<Self> for DecisionEval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for DecisionEval {
    fn cmp(&self, other: &Self) -> Ordering {
        return if self.gain_ratio.is_nan() && other.gain_ratio.is_nan() {
            other.max_branch_width.cmp(&self.max_branch_width)
        } else if self.gain_ratio.is_nan() {
            Ordering::Less
        } else if other.gain_ratio.is_nan() {
            Ordering::Greater
        } else {
            if self.gain_ratio > other.gain_ratio {
                Ordering::Greater
            } else if self.gain_ratio < other.gain_ratio {
                Ordering::Less
            } else {
                other.max_branch_width.cmp(&self.max_branch_width)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::DecisionEval;

    #[test]
    fn test_enum() {
        let a = DecisionEval { gain_ratio: 3.0, max_branch_width: 1 };
        let b = DecisionEval { gain_ratio: 2.0, max_branch_width: 1 };
        let c = DecisionEval { gain_ratio: 2.0, max_branch_width: 2 };
        let d = DecisionEval { gain_ratio: 3.0, max_branch_width: 2 };
        let e = DecisionEval { gain_ratio: f64::NAN, max_branch_width: 1 };
        let f = DecisionEval { gain_ratio: f64::NAN, max_branch_width: 2 };

        assert_eq!(a.cmp(&a), Ordering::Equal);
        assert_eq!(a.cmp(&b), Ordering::Greater);
        assert_eq!(a.cmp(&c), Ordering::Greater);
        assert_eq!(a.cmp(&d), Ordering::Greater);
        assert_eq!(a.cmp(&e), Ordering::Greater);
        assert_eq!(a.cmp(&f), Ordering::Greater);

        assert_eq!(b.cmp(&a), Ordering::Less);
        assert_eq!(b.cmp(&b), Ordering::Equal);
        assert_eq!(b.cmp(&c), Ordering::Greater);
        assert_eq!(b.cmp(&d), Ordering::Less);
        assert_eq!(b.cmp(&e), Ordering::Greater);
        assert_eq!(b.cmp(&f), Ordering::Greater);

        assert_eq!(c.cmp(&a), Ordering::Less);
        assert_eq!(c.cmp(&b), Ordering::Less);
        assert_eq!(c.cmp(&c), Ordering::Equal);
        assert_eq!(c.cmp(&d), Ordering::Less);
        assert_eq!(c.cmp(&e), Ordering::Greater);
        assert_eq!(c.cmp(&f), Ordering::Greater);

        assert_eq!(d.cmp(&a), Ordering::Less);
        assert_eq!(d.cmp(&b), Ordering::Greater);
        assert_eq!(d.cmp(&c), Ordering::Greater);
        assert_eq!(d.cmp(&d), Ordering::Equal);
        assert_eq!(d.cmp(&e), Ordering::Greater);
        assert_eq!(d.cmp(&f), Ordering::Greater);

        assert_eq!(e.cmp(&a), Ordering::Less);
        assert_eq!(e.cmp(&b), Ordering::Less);
        assert_eq!(e.cmp(&c), Ordering::Less);
        assert_eq!(e.cmp(&d), Ordering::Less);
        assert_eq!(e.cmp(&e), Ordering::Equal);
        assert_eq!(e.cmp(&f), Ordering::Greater);

        assert_eq!(f.cmp(&a), Ordering::Less);
        assert_eq!(f.cmp(&b), Ordering::Less);
        assert_eq!(f.cmp(&c), Ordering::Less);
        assert_eq!(f.cmp(&d), Ordering::Less);
        assert_eq!(f.cmp(&e), Ordering::Less);
        assert_eq!(f.cmp(&f), Ordering::Equal);
    }
}
