use std::cmp::Ordering;

use rust::Card;

#[test]
fn test_card_greater() {
    let test_case = Card('A').cmp(&Card('K'));
    assert_eq!(test_case, Ordering::Greater);
}

#[test]
fn test_card_less() {
    let test_case = Card('Q').cmp(&Card('K'));
    assert_eq!(test_case, Ordering::Less);
}

#[test]
fn test_card_equal() {
    let test_case = Card('K').cmp(&Card('K'));
    assert_eq!(test_case, Ordering::Equal);
}
