use basm;
use beast;
use core::error::*;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

const BAKERVM_IMAGE_EXTENSION: &str = "img";
pub const DEFAULT_LANG: Lang = Lang::Beast;

#[derive(Debug)]
pub enum Lang {
    Beast,
    Basm,
}

impl FromStr for Lang {
    type Err = &'static str;
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            "basm" => Ok(Lang::Basm),
            "beast" | "bst" => Ok(Lang::Beast),
            _ => bail!("unknown language. Language must be one of [beast, bst, basm]"),
        }
    }
}

pub fn compile(lang: Option<Lang>, input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    let input = input
        .canonicalize()
        .chain_err(|| "unable to canonicalize input path")?;

    // We don't want to fail at recognizing the language, so we just default to
    // Beast
    let lang = lang.unwrap_or_else(|| {
        if let Some(extension) = input.extension() {
            if let Some(extension) = extension.to_str() {
                extension.parse().unwrap_or(DEFAULT_LANG)
            } else {
                DEFAULT_LANG
            }
        } else {
            DEFAULT_LANG
        }
    });

    let mut fallback_output = input.clone();

    ensure!(
        fallback_output.set_extension(BAKERVM_IMAGE_EXTENSION),
        "unable to set file extension"
    );

    let output = output.unwrap_or(fallback_output);

    let start_dir = env::current_dir().chain_err(|| "unable to get current directory")?;

    let program = match lang {
        Lang::Basm => basm::compile(input).chain_err(|| "unable to compile basm file")?,
        Lang::Beast => beast::compile(input).chain_err(|| "unable to compile Beast file")?,
    };

    env::set_current_dir(start_dir).chain_err(|| "unable to switch directories")?;

    let mut file = File::create(output).chain_err(|| "unable to create file")?;

    file.write_all(&program[..])
        .chain_err(|| "unable to write program data")?;

    file.sync_all()
        .chain_err(|| "unable to sync output file to file system")?;

    Ok(())
}
