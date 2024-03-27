use rust::Question;

fn main() {
    let question = Question::build("../question");
    let answer = question.solve();

    println!("Answer: {answer}");
}
