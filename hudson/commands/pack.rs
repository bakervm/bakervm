use core::error::*;
use image::{self, DynamicImage, RgbImage, RgbaImage};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

const BASM_EXTENSION: &str = "basm";

#[derive(Debug)]
pub enum PackingType {
    Static,
    Dynamic,
}

impl FromStr for PackingType {
    type Err = &'static str;
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            "static" => Ok(PackingType::Static),
            "dynamic" => Ok(PackingType::Dynamic),
            _ => bail!("unknown packing type. Packing type must be one of [static, dynamic]"),
        }
    }
}

pub fn pack(
    packing_type: Option<PackingType>, input: PathBuf, output: Option<PathBuf>
) -> Result<()> {
    let temp_input = input.clone();

    let file_name = if let Some(file_name) = temp_input.file_name() {
        file_name.to_str().ok_or("unable to convert file name")?
    } else {
        bail!("no file name given");
    };

    let pack_type = packing_type.unwrap_or(PackingType::Static);

    let mut fallback_output = input.clone();

    ensure!(
        fallback_output.set_extension(BASM_EXTENSION),
        "unable to set file extension"
    );

    let output = output.unwrap_or(fallback_output);

    let image_data = image::open(input).chain_err(|| "unable to open image file")?;

    let packed_image = if let DynamicImage::ImageRgb8(rgb_image) = image_data {
        pack_rgb(rgb_image)?
    } else if let DynamicImage::ImageRgba8(rgba_image) = image_data {
        pack_rgba(rgba_image)?
    } else {
        bail!("unrecognized image format");
    };

    let module_name = format!(".assets.images.draw_{}", file_name);
    let guard_name = module_name.replace('.', "_");

    let mut file_contents = String::new();

    file_contents += format!("\njmp {}", guard_name.clone()).as_str();

    file_contents += format!("\n{}", module_name).as_str();

    match pack_type {
        PackingType::Static => for (x, y, color) in packed_image {
            file_contents += "\npush $st, #";
            file_contents += format!("{:02x}{:02x}{:02x}", color.0, color.1, color.2).as_str();

            file_contents += "\npush $st, @";
            file_contents += format!("{}", x).as_str();

            file_contents += "\npush $st, @";
            file_contents += format!("{}", y).as_str();

            file_contents += "\ncall std.graphics.draw_point";
        },
        PackingType::Dynamic => {
            file_contents += "\npush $st, @2";
            file_contents += "\nadd $bp, $st";

            file_contents += "\nmov $vi(21), $st"; // y
            file_contents += "\nmov $vi(20), $st"; // x

            for (x, y, color) in packed_image {
                file_contents += "\npush $st, #";
                file_contents += format!("{:02x}{:02x}{:02x}", color.0, color.1, color.2).as_str();

                file_contents += format!("\npush $st, @{}", x).as_str();
                file_contents += "\ndup $vi(20)";
                file_contents += "\nadd $st, $st";

                file_contents += format!("\npush $st, @{}", y).as_str();
                file_contents += "\ndup $vi(21)";
                file_contents += "\nadd $st, $st";

                file_contents += "\ncall std.graphics.draw_point";
            }

            file_contents += "\npush $st, @2";
            file_contents += "\nsub $bp, $st";
        }
    }

    file_contents += "\nret";

    file_contents += format!("\n.{}", guard_name.clone()).as_str();
    file_contents += "\n";

    let mut file = File::create(output).chain_err(|| "failed to create file")?;

    file.write_all(file_contents.as_bytes())
        .chain_err(|| "unable to write to file")?;

    Ok(())
}

fn pack_rgb(image: RgbImage) -> Result<Vec<(usize, usize, (u8, u8, u8))>> {
    println!("RGB image dimensions {:?}", image.dimensions());

    let mut pixels: Vec<(usize, usize, (u8, u8, u8))> = Vec::new();

    for (x, y, color) in image.enumerate_pixels() {
        let color = (color[0], color[1], color[2]);

        pixels.push((x as usize, y as usize, color));
    }

    Ok(pixels)
}

fn pack_rgba(image: RgbaImage) -> Result<Vec<(usize, usize, (u8, u8, u8))>> {
    println!("RGBA image dimensions {:?}", image.dimensions());

    let mut pixels: Vec<(usize, usize, (u8, u8, u8))> = Vec::new();

    for (x, y, color) in image.enumerate_pixels() {
        if color[3] == 255 {
            let color = (color[0], color[1], color[2]);
            pixels.push((x as usize, y as usize, color));
        }
    }

    Ok(pixels)
}
