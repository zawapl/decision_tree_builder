use std::cmp::Ordering;

pub struct DecisionEval {
    pub(crate) gain_ratio: f64,
    pub(crate) max_branch_width: usize,
}

impl Eq for DecisionEval {}

impl PartialEq<Self> for DecisionEval {
    fn eq(&self, other: &Self) -> bool {
        return ((self.gain_ratio == other.gain_ratio) || (self.gain_ratio.is_nan() && other.gain_ratio.is_nan()))
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
