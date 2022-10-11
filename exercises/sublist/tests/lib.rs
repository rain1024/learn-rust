use sublist::*;

#[test]
fn sublist_1() {
    let v1 = &[1, 2, 3];
    let v2 = &[1, 2, 3];
    assert_eq!(sublist(v1, v2), Comparison::Equal);
}

#[test]
fn sublist_2() {
    let v1 = &[1, 2, 3, 1];
    let v2 = &[1, 2, 3];
    assert_eq!(sublist(v1, v2), Comparison::Superlist);
}

#[test]
fn sublist_3() {
    let v1 = &[1, 2, 3];
    let v2 = &[1, 2, 3, 4];
    assert_eq!(sublist(v1, v2), Comparison::Sublist);
}

#[test]
fn sublist_4() {
    let v1 = &[1, 2, 3, 2];
    let v2 = &[1, 2, 3, 4];
    assert_eq!(sublist(v1, v2), Comparison::Unequal);
}

#[test]
fn sublist_5() {
    let v1 = &[1, 3];
    let v2 = &[1, 2, 3];
    assert_eq!(sublist(v1, v2), Comparison::Unequal);
}
