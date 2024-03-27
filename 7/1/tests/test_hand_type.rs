use rust::{Hand, Hands};

#[test]
fn test_five_of_a_kind() {
    let hand = Hand::from("AAAAA");
    assert_eq!(Hands::FiveOfAKind, hand.get_type());
}

#[test]
fn test_four_of_a_kind() {
    let hand = Hand::from("AAAAK");
    assert_eq!(Hands::FourOfAKind, hand.get_type());
}

#[test]
fn test_full_house_correct() {
    let hand = Hand::from("AAAKK");
    assert_eq!(Hands::FullHouse, hand.get_type());
}

#[test]
fn test_three_of_a_kind() {
    let hand = Hand::from("AAAKJ");
    assert_eq!(Hands::ThreeOfAKind, hand.get_type());
}

#[test]
fn test_two_pair() {
    let hand = Hand::from("AAKKJ");
    assert_eq!(Hands::TwoPair, hand.get_type());
}

#[test]
fn test_one_pair() {
    let hand = Hand::from("AAKJ1");
    assert_eq!(Hands::OnePair, hand.get_type());
}

#[test]
fn test_high_card() {
    let hand = Hand::from("AKQJT");
    assert_eq!(Hands::HighCard, hand.get_type());
}

#[test]
fn test_q2kjj() {
    let hand = Hand::from("Q2KJJ");
    assert_eq!(Hands::OnePair, hand.get_type())
}

#[test]
fn test_q2q2q() {
    let hand = Hand::from("Q2Q2Q");
    assert_eq!(Hands::FullHouse, hand.get_type())
}

#[test]
fn test_t3t3j() {
    let hand = Hand::from("T3T3J");
    assert_eq!(Hands::TwoPair, hand.get_type())
}

#[test]
fn test_kk677() {
    let hand = Hand::from("KK677");
    assert_eq!(Hands::TwoPair, hand.get_type())
}

#[test]
fn test_ktjjt() {
    let hand = Hand::from("KTJJT");
    assert_eq!(Hands::TwoPair, hand.get_type())
}

#[test]
fn test_2jjjj() {
    let hand = Hand::from("2JJJJ");
    assert_eq!(Hands::FourOfAKind, hand.get_type())
}

#[test]
fn test_jjjj2() {
    let hand = Hand::from("JJJJ2");
    assert_eq!(Hands::FourOfAKind, hand.get_type())
}

#[test]
fn test_t55j5() {
    let hand = Hand::from("T55J5");
    assert_eq!(Hands::ThreeOfAKind, hand.get_type())
}
