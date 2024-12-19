mod constants;
mod errors;
mod tokeniser;

fn main() {}

#[cfg(test)]
mod test {
    use tokeniser::Token;

    use super::*;

    #[test]
    fn hello_world() {
        std::env::set_var("RUST_BACKTRACE", "full");
        let string = std::fs::read_to_string("./test_files/hello_world.japl").unwrap();
        let expected: Vec<Token> = vec![
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

        println!("{:#?}", tokens);

        assert!(tokens.into_iter().eq(expected.into_iter()))
    }
}
