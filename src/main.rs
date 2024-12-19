mod constants;
mod errors;
mod tokeniser;

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hello_world() {
        // not a proper test yet
        let string = std::fs::read_to_string("./test_files/hello_world.alim").unwrap();
        println!("{:#?}", tokeniser::tokenise(string.trim()));
    }
}
