use tokeniser::tokenise;

mod constants;
mod errors;
mod parser;
mod tokeniser;

#[cfg(test)]
mod tests;

fn main() {
    let string = std::fs::read_to_string("./test_files/functions.japl").unwrap();
    let token = tokenise(string.trim()).unwrap();
    let x = parser::function_names(token);
    println!("{:#?}", x);
}
