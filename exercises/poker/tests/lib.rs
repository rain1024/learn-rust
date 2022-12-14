use poker::winning_hands;
use std::collections::HashSet;

fn hs_from<'a>(input: &[&'a str]) -> HashSet<&'a str> {
    let mut hs = HashSet::new();
    for item in input.iter() {
        hs.insert(*item);
    }
    hs
}

fn test<'a, 'b>(input: &[&'a str], expected: &[&'b str]) {
    assert_eq!(hs_from(&winning_hands(input)), hs_from(expected))
}

#[test]
fn test_single_hand_always_wins() {
    test(&["4S 5S 7H 8D JC"], &["4S 5S 7H 8D JC"])
}