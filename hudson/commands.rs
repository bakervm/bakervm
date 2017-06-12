use basm;
use clap::ArgMatches;
use definitions::{DisplayResolution, ImageBuilder, Signal, Target, Value};
use definitions::error::*;
use std::fs::File;
use std::io::Write;

pub fn stock(_matches: &ArgMatches) -> Result<()> {
    let mut builder = ImageBuilder::new();

    let res_def = DisplayResolution::default();

    let max = res_def.width * res_def.height;

    for x in 0..max {
        builder.push(Target::ValueIndex(0), Value::Address(x));
        builder.push(Target::Framebuffer, Value::Color(0, 0, 0));
    }

    // Triangle
    builder.push(Target::ValueIndex(0), Value::Address(res_def.width + 1));
    builder.push(Target::Framebuffer, Value::Color(0xFF, 0xFF, 0xFF));

    builder.push(
        Target::ValueIndex(0),
        Value::Address((res_def.width * 2) + 2),
    );
    builder.push(Target::Framebuffer, Value::Color(0xFF, 0xFF, 0xFF));

    builder.push(
        Target::ValueIndex(0),
        Value::Address((res_def.width * 3) + 3),
    );
    builder.push(Target::Framebuffer, Value::Color(0xFF, 0xFF, 0xFF));

    builder.push(
        Target::ValueIndex(0),
        Value::Address((res_def.width * 4) + 4),
    );
    builder.push(Target::Framebuffer, Value::Color(0xFF, 0xFF, 0xFF));

    builder.push(
        Target::ValueIndex(0),
        Value::Address((res_def.width * 5) + 3),
    );
    builder.push(Target::Framebuffer, Value::Color(0xFF, 0xFF, 0xFF));

    builder.push(
        Target::ValueIndex(0),
        Value::Address((res_def.width * 6) + 2),
    );
    builder.push(Target::Framebuffer, Value::Color(0xFF, 0xFF, 0xFF));

    builder.push(
        Target::ValueIndex(0),
        Value::Address((res_def.width * 7) + 1),
    );
    builder.push(Target::Framebuffer, Value::Color(0xFF, 0xFF, 0xFF));

    // Underscore
    builder.push(
        Target::ValueIndex(0),
        Value::Address((res_def.width * 7) + 6),
    );
    builder.push(Target::Framebuffer, Value::Color(0xFF, 0xFF, 0xFF));
    builder.push(
        Target::ValueIndex(0),
        Value::Address((res_def.width * 7) + 7),
    );
    builder.push(Target::Framebuffer, Value::Color(0xFF, 0xFF, 0xFF));
    builder.push(
        Target::ValueIndex(0),
        Value::Address((res_def.width * 7) + 8),
    );
    builder.push(Target::Framebuffer, Value::Color(0xFF, 0xFF, 0xFF));
    builder.push(
        Target::ValueIndex(0),
        Value::Address((res_def.width * 7) + 9),
    );
    builder.push(Target::Framebuffer, Value::Color(0xFF, 0xFF, 0xFF));
    builder.push(
        Target::ValueIndex(0),
        Value::Address((res_def.width * 7) + 10),
    );
    builder.push(Target::Framebuffer, Value::Color(0xFF, 0xFF, 0xFF));
    builder.push(
        Target::ValueIndex(0),
        Value::Address((res_def.width * 7) + 11),
    );
    builder.push(Target::Framebuffer, Value::Color(0xFF, 0xFF, 0xFF));

    builder.int(Signal::FlushFrame);

    builder.pause();
    builder.jmp(max + 1);

    let program_data = builder.gen();

    let mut output_file = File::create("stock.img").chain_err(|| "unable to open output file")?;

    output_file.write_all(&program_data[..]).chain_err(|| "unable to write output file")?;

    output_file.sync_all().chain_err(|| "unable to sync output file to file system")?;

    Ok(())
}

pub fn compile(matches: &ArgMatches) -> Result<()> {
    let input_file_name = if let Some(file_name) = matches.value_of("input") {
        file_name
    } else {
        bail!("no file name given");
    };

    if matches.is_present("basm") {
        let program = basm::compile(input_file_name.to_owned())
            .chain_err(|| "unable to compile file")?;

        let output_file_name = if let Some(file_name) = matches.value_of("output") {
            file_name.to_owned()
        } else {
            format!("{}.img", input_file_name)
        };

        let mut file = File::create(output_file_name).chain_err(|| "unable to create file")?;

        file.write_all(&program[..]).chain_err(|| "unable to write program data")?;

        file.sync_all().chain_err(|| "unable to sync output file to file system")?;
    }

    Ok(())
}
