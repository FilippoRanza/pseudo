#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pseudo_lang);

mod ast;
mod generator;
mod string_builder;

use std::fs::File;
use std::io::{stdout, Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Arguments {
    #[structopt(help = "Specify input pseudo code file")]
    in_file: PathBuf,
    #[structopt(help = "Specify output latex algorithm file, by default writes to STDOUT")]
    out_file: Option<PathBuf>,
    #[structopt(short = "-l", long = "--label", help ="Specify algorithm label, if label text is not specified the label is generated from input file name")]
    label: Option<Option<String>>,
    #[structopt(short = "-H", long = "--here", help ="Force algorithm positioning with '!htb'")]
    force_location: bool,
}

fn get_algorithm_label(label: Option<Option<String>>, file_name: &PathBuf) -> LabelResult {
    match label {
        Some(Some(label)) => LabelResult::Success(Some(label)),
        Some(None) => label_from_file_name(file_name),
        None => LabelResult::Success(None),
    }
}

#[derive(Debug)]
enum LabelResult {
    Success(Option<String>),
    Error,
}
impl LabelResult {
    fn parse_result(self) -> Result<Option<String>, &'static str> {
        match self {
            Self::Success(lbl) => Ok(lbl),
            Self::Error => Err(
                "Given file name cannot be converted in a UTF-8 string: cannot generate label from file name"
            )
        }
    }
}

fn label_from_file_name(file_name: &PathBuf) -> LabelResult {
    let label: Option<String> =
        file_name
            .iter()
            .map(|c| c.to_str())
            .try_fold(String::new(), |acc, curr| {
                let curr = curr?;
                if acc.len() == 0 {
                    Some(acc + curr)
                } else {
                    Some(acc + "-" + curr)
                }
            });

    if let Some(label) = label {
        let label = remove_extension(label, file_name);
        LabelResult::Success(Some(label))
    } else {
        LabelResult::Error
    }
}

fn remove_extension(mut label: String, file: &PathBuf) -> String {
    if let Some(ext) = file.extension() {
        let new_len = label.len() - ext.len() - 1;
        label.truncate(new_len);
        format!("algo:{}", label)
    } else {
        format!("algo:{}", label)
    }
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

fn run_translation<'a>(code: &'a str, label: Option<String>, force_position: bool) -> Result<String, String> {
    let parser = pseudo_lang::CodeParser::new();
    let res = parser.parse(&code);
    match res {
        Ok(tree) => Ok(generator::generate(tree, label, ' ', force_position)),
        Err(err) => Err(format!("{}", err)),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::from_args();
    let code = load_file(&args.in_file)?;
    let label = get_algorithm_label(args.label, &args.in_file).parse_result()?;
    let latex = run_translation(&code, label, args.force_location)?;
    output_latex_code(latex, args.out_file)?;
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_label_from_file_name() {
        let file_name = PathBuf::new()
            .join("file")
            .join("inside")
            .join("directory")
            .join("test.algo");
        let res = label_from_file_name(&file_name);
        match res {
            LabelResult::Success(Some(label)) => {
                assert_eq!(label, "algo:file-inside-directory-test")
            }
            _ => panic!("This test results in: {:?}", res),
        }
    }
}
