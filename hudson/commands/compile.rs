use basm;
use clap::ArgMatches;
use core::error::*;
use std::env;
use std::fs::File;
use std::io::Write;

pub fn compile(matches: &ArgMatches) -> Result<()> {
    let input_file_name = if let Some(file_name) = matches.value_of("input") {
        file_name
    } else {
        bail!("no file name given");
    };

    let start_dir = env::current_dir().chain_err(|| "unable to get current directory")?;

    if matches.is_present("basm") {
        let program = basm::compile(input_file_name.to_owned()).chain_err(|| "unable to compile file")?;

        let output_file_name = if let Some(file_name) = matches.value_of("output") {
            file_name.to_owned()
        } else {
            format!("{}.img", input_file_name)
        };

        env::set_current_dir(start_dir).chain_err(|| "unable to switch directories")?;

        let mut file = File::create(output_file_name).chain_err(|| "unable to create file")?;

        file.write_all(&program[..]).chain_err(|| "unable to write program data")?;

        file.sync_all().chain_err(|| "unable to sync output file to file system")?;
    }

    Ok(())
}
