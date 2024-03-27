use rust::Question;

#[test]
fn test_example() {
    let question = Question::build("../example");
    let answer = question.solve();

    assert_eq!(6440, answer)
}

#[test]
fn test_example2() {
    let question = Question::build("../example2");
    let answer = question.solve();

    assert_eq!(6592, answer)
}
