use crate::*;

#[test]
fn hello_world() {
    let string = std::fs::read_to_string("./test_files/hello_world.japl").unwrap();
    let expected: Vec<tokeniser::Token> = [
        "fn",
        "main",
        "(",
        ")",
        "->",
        "i32",
        "{",
        "print",
        "(",
        "\"Hello world\"",
        ")",
        ";",
        "return",
        "0",
        ";",
        "}",
    ]
    .into_iter()
    .map(TryInto::try_into)
    .map(Result::unwrap)
    .collect();

    let tokens = tokeniser::tokenise(string.trim()).unwrap();

    assert!(tokens.into_iter().eq(expected.into_iter()))
}
