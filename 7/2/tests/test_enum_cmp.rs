use rust::Hands;

#[test]
fn test_enum_cmp() {
    assert_eq!(true, Hands::FiveOfAKind > Hands::FullHouse)
}
