mod alias;
mod error;
mod lexer;
mod parser;
mod runtime;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut args = std::env::args();
    args.next();

    let file = args.next().unwrap_or("program.japl".into());
    let program = std::fs::read_to_string(file).unwrap();

    let tokens = match lexer::tokenise(program.trim()) {
        Ok(i) => i,
        Err(i) => {
            eprintln!("Error: {:?}", i);
            return;
        }
    };

    let (instruction_set, labels) = match parser::parse(tokens) {
        Ok(i) => {
            // println!("{:#?}", i);
            i
        }
        Err(i) => {
            eprintln!("Error: {:?}", i);
            return;
        }
    };

    runtime::run(instruction_set, labels);
}
