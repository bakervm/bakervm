use clap::ArgMatches;
use definitions::{DisplayResolution, InternalInterrupt, Target, Value};
use definitions::ImageBuilder;
use error::*;
use std::fs::File;
use std::io::Write;

pub fn stock(_matches: &ArgMatches) -> CompilationResult<()> {
    let mut builder = ImageBuilder::new();

    let res_def = DisplayResolution::default();

    let max = res_def.width * res_def.height;

    for x in 0..max {
        builder.push(Target::Framebuffer(x), Value::Color(0, 0, 0));
    }

    // Triangle
    builder.push(
        Target::Framebuffer(res_def.width + 1),
        Value::Color(0xFF, 0xFF, 0xFF),
    );
    builder.push(
        Target::Framebuffer((res_def.width * 2) + 2),
        Value::Color(0xFF, 0xFF, 0xFF),
    );
    builder.push(
        Target::Framebuffer((res_def.width * 3) + 3),
        Value::Color(0xFF, 0xFF, 0xFF),
    );
    builder.push(
        Target::Framebuffer((res_def.width * 4) + 4),
        Value::Color(0xFF, 0xFF, 0xFF),
    );
    builder.push(
        Target::Framebuffer((res_def.width * 5) + 3),
        Value::Color(0xFF, 0xFF, 0xFF),
    );
    builder.push(
        Target::Framebuffer((res_def.width * 6) + 2),
        Value::Color(0xFF, 0xFF, 0xFF),
    );
    builder.push(
        Target::Framebuffer((res_def.width * 7) + 1),
        Value::Color(0xFF, 0xFF, 0xFF),
    );

    // Underscore
    builder.push(
        Target::Framebuffer((res_def.width * 7) + 6),
        Value::Color(0xFF, 0xFF, 0xFF),
    );
    builder.push(
        Target::Framebuffer((res_def.width * 7) + 7),
        Value::Color(0xFF, 0xFF, 0xFF),
    );
    builder.push(
        Target::Framebuffer((res_def.width * 7) + 8),
        Value::Color(0xFF, 0xFF, 0xFF),
    );
    builder.push(
        Target::Framebuffer((res_def.width * 7) + 9),
        Value::Color(0xFF, 0xFF, 0xFF),
    );
    builder.push(
        Target::Framebuffer((res_def.width * 7) + 10),
        Value::Color(0xFF, 0xFF, 0xFF),
    );
    builder.push(
        Target::Framebuffer((res_def.width * 7) + 11),
        Value::Color(0xFF, 0xFF, 0xFF),
    );

    builder.int(InternalInterrupt::FlushFramebuffer);

    builder.pause();
    builder.jmp(max + 1);

    let program_data = builder.gen();

    let mut output_file = File::create("stock.img").chain_err(|| "unable to open output file")?;

    output_file.write_all(&program_data[..]).chain_err(|| "unable to write output file")?;

    output_file.sync_all().chain_err(|| "unable to sync output file to file system")?;

    Ok(())
}

pub fn build(_matches: &ArgMatches) -> CompilationResult<()> {
    unimplemented!()
}
