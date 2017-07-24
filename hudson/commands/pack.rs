use clap::ArgMatches;
use core::error::*;
use image::{self, DynamicImage, RgbImage, RgbaImage};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::result;
use std::str::FromStr;

enum PackingMode {
    Static,
    DynamicPosition,
    // DynamicScale,
    // DynamicRotation,
    // DynamicScalePosition,
    // DynamicScaleRotation,
    // DynamicPositionRotation,
}

impl FromStr for PackingMode {
    type Err = &'static str;

    fn from_str(s: &str) -> result::Result<PackingMode, Self::Err> {
        match s {
            "static" => Ok(PackingMode::Static),
            "dynamic-pos" => Ok(PackingMode::DynamicPosition),
            _ => Err("unable to parse packing-mode"),
        }
    }
}

enum Pixel {
    Padding(u32),
    Color(u8, u8, u8),
}

pub fn pack(matches: &ArgMatches) -> Result<()> {
    let input_file_name = if let Some(file_name) = matches.value_of("input") {
        file_name
    } else {
        bail!("no file name given");
    };

    let file_name = if let Some(file_name) = Path::new(input_file_name).file_name() {
        file_name
    } else {
        bail!("unable to extract file name");
    };

    let pack_type: PackingMode = if let Some(pack_type) = matches.value_of("type") {
        pack_type.parse()?
    } else {
        "static".parse()? // The default packing mode
    };

    let image_data = image::open(input_file_name).chain_err(|| "unable to open image file")?;

    let (packed_image, dimensions) = if let DynamicImage::ImageRgb8(rgb_image) = image_data {
        (pack_rgb(rgb_image.clone())?, rgb_image.dimensions())
    } else if let DynamicImage::ImageRgba8(rgba_image) = image_data {
        (pack_rgba(rgba_image.clone())?, rgba_image.dimensions())
    } else {
        bail!("cannot load grayscale image");
    };

    println!("Image dimensions {:?}", dimensions);

    let (image_width, image_height) = dimensions;

    let module_name = format!(".assets.images.draw_{}", file_name.to_str().unwrap());
    let guard_name = module_name.replace('.', "_");

    let mut file_contents = String::new();

    file_contents += format!("\njmp {}", guard_name.clone()).as_str();

    file_contents += format!("\n{}", module_name).as_str();

    match pack_type {
        PackingMode::Static => {
            let mut idx = 0;
            file_contents += "\npush $st, @1";
            file_contents += "\nadd $bp, $st";

            file_contents += "\npush $vi(0), @0";

            file_contents += "\ndup $vi(1)";
            file_contents += "\nmov $vi(20), $st";
            file_contents += format!("\npush $st, @{}", image_width).as_str();
            file_contents += "\nsub $vi(20), $st";

            for pixel in packed_image {
                match pixel {
                    Pixel::Color(r, g, b) => {
                        file_contents += format!("\npush $fb, #{:02x}{:02x}{:02x}", r, g, b)
                            .as_str();
                        file_contents += "\npush $st, @1";
                        file_contents += "\nadd $vi(0), $st";
                        idx += 1;
                    }
                    Pixel::Padding(padding) => {
                        idx += padding;
                        file_contents += format!("\npush $st, @{}", padding).as_str();
                        file_contents += "\nadd $vi(0), $st";
                    }
                }

                if idx >= image_width {
                    idx = 0;
                    file_contents += "\ndup $vi(20)";
                    file_contents += "\nadd $vi(0), $st";
                }
            }


            file_contents += "\npush $st, @1";
            file_contents += "\nsub $bp, $st";
        }
        PackingMode::DynamicPosition => {
            let mut idx = 0;
            file_contents += "\npush $st, @1";
            file_contents += "\nadd $bp, $st";

            file_contents += "\ndup $vi(1)";
            file_contents += "\nmul $st, $st";
            file_contents += "\nadd $st, $st";

            file_contents += "\nmov $vi(0), $st";

            file_contents += "\ndup $vi(1)";
            file_contents += "\nmov $vi(20), $st";
            file_contents += format!("\npush $st, @{}", image_width).as_str();
            file_contents += "\nsub $vi(20), $st";

            for pixel in packed_image {
                match pixel {
                    Pixel::Color(r, g, b) => {
                        file_contents += format!("\npush $fb, #{:02x}{:02x}{:02x}", r, g, b)
                            .as_str();
                        file_contents += "\npush $st, @1";
                        file_contents += "\nadd $vi(0), $st";
                        idx += 1;
                    }
                    Pixel::Padding(padding) => {
                        idx += padding;
                        file_contents += format!("\npush $st, @{}", padding).as_str();
                        file_contents += "\nadd $vi(0), $st";
                    }
                }

                if idx >= image_width {
                    idx = 0;
                    file_contents += "\ndup $vi(20)";
                    file_contents += "\nadd $vi(0), $st";
                }
            }


            file_contents += "\npush $st, @1";
            file_contents += "\nsub $bp, $st";
        }
    }

    file_contents += "\nret";

    file_contents += format!("\n.{}", guard_name.clone()).as_str();
    file_contents += "\n";

    let mut file = File::create(format!("{}.basm", file_name.to_str().unwrap())).chain_err(
        || "failed to create file",
    )?;

    file.write_all(file_contents.as_bytes()).chain_err(|| "unable to write to file")?;

    Ok(())
}

fn pack_rgb(image: RgbImage) -> Result<Vec<Pixel>> {

    let mut pixels = Vec::new();

    for (x, y, color) in image.enumerate_pixels() {
        pixels.push(Pixel::Color(color[0], color[1], color[2]));
    }

    Ok(pixels)
}

fn pack_rgba(image: RgbaImage) -> Result<Vec<Pixel>> {
    let mut pixels = Vec::new();

    let mut padding = 0;

    for (x, y, color) in image.enumerate_pixels() {
        if color[3] > 128 {
            if padding > 0 {
                pixels.push(Pixel::Padding(padding));
                padding = 0;
            }

            pixels.push(Pixel::Color(color[0], color[1], color[2]));
        } else {
            padding += 1;
        }
    }

    Ok(pixels)
}
