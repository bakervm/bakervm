use clap::ArgMatches;
use definitions::error::*;
use image::{self, DynamicImage, RgbImage, RgbaImage};

pub fn pack(matches: &ArgMatches) -> Result<()> {
    let input_file_name = if let Some(file_name) = matches.value_of("input") {
        file_name
    } else {
        bail!("no file name given");
    };

    let image_data = image::open(input_file_name).chain_err(|| "unable to open image file")?;

    let packed_image = if let DynamicImage::ImageRgb8(rgb_image) = image_data {
        pack_rgb(rgb_image)?
    } else if let DynamicImage::ImageRgba8(rgba_image) = image_data {
        pack_rgba(rgba_image)?
    } else {
        bail!("cannot load grayscale image");
    };

    let mut file_contents = String::new();
    file_contents += "\n.assets.images.draw_";
    file_contents += input_file_name;

    for (x, y, color) in packed_image {
        println!("{:?} -- {:?} -- {:?}", x, y, color);
    }

    file_contents += "\nret";

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
