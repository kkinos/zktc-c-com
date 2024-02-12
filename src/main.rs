use std::{
    collections::HashMap,
    io::{BufReader, Read, Write},
};

use anyhow::{anyhow, Context, Result};
use codegen::codegen;
use nom::error::convert_error;
use parse::{parse_define, parse_program, Program};

mod codegen;
mod parse;
mod ty;

use clap::Parser;

#[derive(Parser)]
#[clap(version = "0.1", author = "kinpoko", about = "ZKTC C compiler")]
struct Args {
    /// .zktc.c file path
    file_path: Vec<std::path::PathBuf>,

    /// output file name
    #[arg(short = 'o', default_value = "a.asm")]
    output_file_name: std::path::PathBuf,
}

fn main() -> Result<()> {
    let mut buf = String::new();
    let mut defines = HashMap::new();
    let mut program = Program {
        funcs: Vec::new(),
        globals: Vec::new(),
    };

    let args = Args::parse();
    for file_path in args.file_path {
        let file = std::fs::File::open(&file_path)
            .with_context(|| format!("could not read file '{}'", &file_path.display()))?;
        let mut reader = BufReader::new(file);
        let mut text = String::new();
        reader.read_to_string(&mut text)?;

        let res = parse_define(&text, defines.clone());
        match res {
            Ok((_, (res, d))) => {
                let ast = parse_program(&res, program.globals.clone());
                match ast {
                    Ok((_, (mut funcs, globals))) => {
                        program.funcs.append(&mut funcs);
                        program.globals = globals;
                        defines = d;
                        // println!("{:?}", program);
                    }
                    Err(nom::Err::Error(e)) => {
                        return Err(anyhow!(
                            "{} \nCompile error\n{}",
                            &file_path.display(),
                            convert_error(res.as_str(), e)
                        ));
                    }
                    _ => {}
                }
            }
            Err(nom::Err::Error(e)) => {
                return Err(anyhow!(
                    "{} \nMacro error\n{}",
                    &file_path.display(),
                    convert_error(text.as_str(), e)
                ));
            }
            _ => {}
        }
    }
    codegen(&mut buf, &program)?;

    let mut output_file = std::fs::File::create(&args.output_file_name)
        .with_context(|| "could not create file".to_string())?;
    write!(output_file, "{}", buf)?;
    output_file.flush()?;

    Ok(())
}
