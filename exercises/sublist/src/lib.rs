#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_v1: &[T], _v2: &[T]) -> Comparison {
    if _v1.len() == 0 {
        if _v2.len() == 0 {
            return Comparison::Equal;
        } else {
            return Comparison::Sublist;
        }
    }
    if _v2.len() == 0 {
        return Comparison::Superlist;
    }
    if _v1.len() > _v2.len() {
        if contains(_v1, _v2) {
            return Comparison::Superlist;
        } else {
            return Comparison::Unequal;
        }
    } else if _v1.len() == _v2.len() {
        if contains(_v1, _v2) {
            return Comparison::Equal;
        } else {
            return Comparison::Unequal;
        }
    } else {
        if contains(_v2, _v1) {
            return Comparison::Sublist;
        } else {
            return Comparison::Unequal;
        }
    }
}

pub fn contains<T: PartialEq>(_v1: &[T], _v2: &[T]) -> bool {
    _v1.windows(_v2.len()).any(|item| item == _v2)
}