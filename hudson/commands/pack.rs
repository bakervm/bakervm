// use clap::ArgMatches;
use core::error::*;
use image::{self, DynamicImage, RgbImage, RgbaImage};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

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

// pub fn pack(matches: &ArgMatches) -> Result<()> {
// let input_file_name = if let Some(file_name) = matches.value_of("input")
// {         file_name
//     } else {
//         bail!("no file name given");
//     };
//
// let file_name = if let Some(file_name) =
// Path::new(input_file_name).file_name() {         file_name
//     } else {
//         bail!("unable to extract file name");
//     };
//
//     let pack_type = if let Some(pack_type) = matches.value_of("type") {
//         pack_type
//     } else {
//         "static"
//     };
//
// let image_data = image::open(input_file_name).chain_err(|| "unable to
// open image file")?;
//
// let packed_image = if let DynamicImage::ImageRgb8(rgb_image) =
// image_data {         pack_rgb(rgb_image)?
//     } else if let DynamicImage::ImageRgba8(rgba_image) = image_data {
//         pack_rgba(rgba_image)?
//     } else {
//         bail!("cannot load grayscale image");
//     };
//
// let module_name = format!(".assets.images.draw_{}",
// file_name.to_str().unwrap()); let guard_name = module_name.replace('.',
// "_");
//
//     let mut file_contents = String::new();
//
//     file_contents += format!("\njmp {}", guard_name.clone()).as_str();
//
//     file_contents += format!("\n{}", module_name).as_str();
//
//     if pack_type == "static" {
//         for (x, y, color) in packed_image {
//             file_contents += "\npush $st, #";
// file_contents += format!("{:02x}{:02x}{:02x}", color.0, color.1,
// color.2).as_str();
//
//             file_contents += "\npush $st, @";
//             file_contents += format!("{}", x).as_str();
//
//             file_contents += "\npush $st, @";
//             file_contents += format!("{}", y).as_str();
//
//             file_contents += "\ncall std.graphics.draw_point";
//         }
//     } else if pack_type == "dynamic-pos" {
//         file_contents += "\npush $st, @2";
//         file_contents += "\nadd $bp, $st";
//
//         file_contents += "\nmov $vi(21), $st"; // y
//         file_contents += "\nmov $vi(20), $st"; // x
//
//         for (x, y, color) in packed_image {
//             file_contents += "\npush $st, #";
// file_contents += format!("{:02x}{:02x}{:02x}", color.0, color.1,
// color.2).as_str();
//
//             file_contents += format!("\npush $st, @{}", x).as_str();
//             file_contents += "\ndup $vi(20)";
//             file_contents += "\nadd $st, $st";
//
//             file_contents += format!("\npush $st, @{}", y).as_str();
//             file_contents += "\ndup $vi(21)";
//             file_contents += "\nadd $st, $st";
//
//             file_contents += "\ncall std.graphics.draw_point";
//         }
//
//         file_contents += "\npush $st, @2";
//         file_contents += "\nsub $bp, $st";
//     }
//
//     file_contents += "\nret";
//
//     file_contents += format!("\n.{}", guard_name.clone()).as_str();
//     file_contents += "\n";
//
// let mut file = File::create(format!("{}.basm",
// file_name.to_str().unwrap())) .chain_err(|| "failed to create
// file")?;
//
// file.write_all(file_contents.as_bytes()).chain_err(|| "unable to write
// to file")?;
//
//     Ok(())
// }
//
// fn pack_rgb(image: RgbImage) -> Result<Vec<(usize, usize, (u8, u8, u8))>> {
//     println!("RGB image dimensions {:?}", image.dimensions());
//
//     let mut pixels: Vec<(usize, usize, (u8, u8, u8))> = Vec::new();
//
//     for (x, y, color) in image.enumerate_pixels() {
//         let color = (color[0], color[1], color[2]);
//
//         pixels.push((x as usize, y as usize, color));
//     }
//
//     Ok(pixels)
// }
//
// fn pack_rgba(image: RgbaImage) -> Result<Vec<(usize, usize, (u8, u8, u8))>> {
//     println!("RGBA image dimensions {:?}", image.dimensions());
//
//     let mut pixels: Vec<(usize, usize, (u8, u8, u8))> = Vec::new();
//
//     for (x, y, color) in image.enumerate_pixels() {
//         if color[3] == 255 {
//             let color = (color[0], color[1], color[2]);
//             pixels.push((x as usize, y as usize, color));
//         }
//     }
//
//     Ok(pixels)
// }
