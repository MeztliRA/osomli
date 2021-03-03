use std::{fs, path, ffi::OsStr};
use structopt::StructOpt;

pub mod tokens;
mod lexer;
mod interpreter;

#[derive(StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,
}

fn main() {
    let args = Args::from_args();

    let extension = OsStr::new("soml");
    
    assert_eq!(&extension, &args.path.extension().unwrap(), "not a soml file.");

    let file = fs::read_to_string(&args.path).unwrap();

    let tokens = lexer::lex(file);

    interpreter::interpret(tokens);
}