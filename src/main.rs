pub mod lib;
use lib::{ api::StackExchangeAPI, models };
use std::io::Result;

fn main() -> Result<()> {
    let stack_overflow = StackExchangeAPI::new(
        "https://api.stackexchange.com",
        "stackoverflow"
    );

    let questions = stack_overflow
        .search("rounded rect", "swift")?;
    let question = questions
        .get(0)
        .unwrap();

    let answers = stack_overflow
        .get_answers(question.question_id)?;
    let answer = answers
        .get(0)
        .unwrap();

    println!("{}", answer.title);
    println!("\n\n");
    println!("{}", answer.body_markdown);

    return Ok(());
}
