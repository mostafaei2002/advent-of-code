use std::cmp::Ordering;

use rust::Hand;

#[test]
fn test_2345a_q2kjj() {
    let hand1 = Hand::from("2345A");
    let hand2 = Hand::from("Q2KJJ");

    assert_eq!(Ordering::Less, hand1.cmp(&hand2));
}

#[test]
fn test_q2kjj_q2q2q() {
    let hand1 = Hand::from("Q2KJJ");
    let hand2 = Hand::from("Q2Q2Q");

    assert_eq!(Ordering::Less, hand1.cmp(&hand2));
}

#[test]
fn test_q2q2q_t3t3j() {
    let hand1 = Hand::from("Q2Q2Q");
    let hand2 = Hand::from("T3T3J");

    assert_eq!(Ordering::Greater, hand1.cmp(&hand2));
}

#[test]
fn test_t3t3j_t3q33() {
    let hand1 = Hand::from("T3T3J");
    let hand2 = Hand::from("T3Q33");

    assert_eq!(Ordering::Less, hand1.cmp(&hand2));
}

#[test]
fn test_t3q33_2345j() {
    let hand1 = Hand::from("T3Q33");
    let hand2 = Hand::from("2345J");

    assert_eq!(Ordering::Greater, hand1.cmp(&hand2));
}

#[test]
fn test_2345j_j345a() {
    let hand1 = Hand::from("2345J");
    let hand2 = Hand::from("J345A");

    assert_eq!(Ordering::Less, hand1.cmp(&hand2));
}

#[test]
fn test_j345a_32t3k() {
    let hand1 = Hand::from("J345A");
    let hand2 = Hand::from("32T3K");

    assert_eq!(Ordering::Less, hand1.cmp(&hand2));
}

#[test]
fn test_32t3k_t55j5() {
    let hand1 = Hand::from("32T3K");
    let hand2 = Hand::from("T55J5");

    assert_eq!(Ordering::Less, hand1.cmp(&hand2));
}

#[test]
fn test_t55j5_kk677() {
    let hand1 = Hand::from("T55J5");
    let hand2 = Hand::from("KK677");

    assert_eq!(Ordering::Greater, hand1.cmp(&hand2));
}

#[test]
fn test_kk677_ktjjt() {
    let hand1 = Hand::from("KK677");
    let hand2 = Hand::from("KTJJT");

    assert_eq!(Ordering::Greater, hand1.cmp(&hand2));
}

#[test]
fn test_ktjjt_qqqja() {
    let hand1 = Hand::from("KTJJT");
    let hand2 = Hand::from("QQQJA");

    assert_eq!(Ordering::Less, hand1.cmp(&hand2));
}

#[test]
fn test_qqqja_jjjjj() {
    let hand1 = Hand::from("QQQJA");
    let hand2 = Hand::from("JJJJJ");

    assert_eq!(Ordering::Less, hand1.cmp(&hand2));
}

#[test]
fn test_jjjjj_jaaaa() {
    let hand1 = Hand::from("JJJJJ");
    let hand2 = Hand::from("QQQJA");

    assert_eq!(Ordering::Greater, hand1.cmp(&hand2));
}

#[test]
fn test_jaaaa_aaaaj() {
    let hand1 = Hand::from("JAAAA");
    let hand2 = Hand::from("AAAAJ");

    assert_eq!(Ordering::Less, hand1.cmp(&hand2));
}

#[test]
fn test_aaaaj_aaaaa() {
    let hand1 = Hand::from("AAAAJ");
    let hand2 = Hand::from("AAAAA");

    assert_eq!(Ordering::Less, hand1.cmp(&hand2));
}

#[test]
fn test_aaaaa_2aaaa() {
    let hand1 = Hand::from("AAAAA");
    let hand2 = Hand::from("2AAAA");

    assert_eq!(Ordering::Greater, hand1.cmp(&hand2));
}

#[test]
fn test_2aaaa_2jjjj() {
    let hand1 = Hand::from("2AAAA");
    let hand2 = Hand::from("2JJJJ");

    assert_eq!(Ordering::Greater, hand1.cmp(&hand2));
}

#[test]
fn test_2jjjj_jjjj2() {
    let hand1 = Hand::from("2JJJJ");
    let hand2 = Hand::from("JJJJ2");

    assert_eq!(Ordering::Less, hand1.cmp(&hand2));
}
