#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pseudo_lang);

mod ast;
mod generator;

use std::path::PathBuf;
use std::io::{Write, Read, stdout};
use std::fs::File;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Arguments {
    in_file: PathBuf,
    out_file: Option<PathBuf>,
}

fn output_latex_code(latex: String, file: Option<PathBuf>) -> std::io::Result<()> {
    let bytes = latex.as_bytes();
    if let Some(file) = file {
        let mut output = File::create(file)?;
        output.write(&bytes)?;
    } else {
        let mut output = stdout();
        output.write(&bytes)?;
    }
    Ok(())
}

fn load_file(file: &PathBuf) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() {
    println!("Hello, world!");
}
