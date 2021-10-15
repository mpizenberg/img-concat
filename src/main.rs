// SPDX-License-Identifier: MPL-2.0

use anyhow::Context;
use image::io::Reader as ImageReader;
use image::{GenericImage, RgbImage};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let app_name = env!("CARGO_PKG_NAME");
    let app = seahorse::App::new(app_name)
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} PATH...", app_name))
        .action(|c| run(c).unwrap());
    app.run(args);
}

fn run(c: &seahorse::Context) -> anyhow::Result<()> {
    let img_paths: Vec<&Path> = c.args.iter().map(Path::new).collect();
    assert!(!img_paths.is_empty(), "At least one image is needed");

    // Read the image sizes.
    let mut img_sizes = Vec::with_capacity(img_paths.len());
    for path in &img_paths {
        let size = imagesize::size(path)?;
        img_sizes.push((size.width, size.height));
    }

    // Compute the total size of the concatenated image.
    let max_height = img_sizes.iter().map(|(_, h)| *h).max().unwrap() as u32;
    let width_sum = img_sizes.iter().map(|(w, _)| w).sum::<usize>() as u32;

    // Build the concatenated image.
    let mut concat_img = RgbImage::new(width_sum, max_height);
    let mut width_acc = 0;
    for path in &img_paths {
        eprintln!("Loading {}", path.display());
        let img = ImageReader::open(path)?.decode()?.into_rgb8();
        concat_img.copy_from(&img, width_acc, 0)?;
        width_acc += img.width();
    }

    // Save the concatenated image to disk.
    concat_img
        .save("out.jpg")
        .context("Failed to save concatenated image")
}
