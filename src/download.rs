use imagequant::Attributes;
use lodepng::Encoder;
use rayon::prelude::*;
use std::{
    fs::{self, File, Metadata},
    io::{self, Write},
    path::{Path, PathBuf},
};

use crate::chapters::Chapter;
use crate::manga;
use crate::util;

pub fn save_chapter_pages(chapter: &Chapter, directory: &str) -> Result<(), io::Error> {
    let folder: PathBuf = Path::new(&directory).join(&chapter.slug);
    if folder.exists() {
        println!(
            "We have already downloaded chapter {}, skipping...",
            &chapter.slug
        );
        return Ok(());
    }
    fs::create_dir_all(&folder)?;

    let images: Vec<String> = manga::get_manga_pages(chapter);
    images.par_iter().enumerate().for_each(|(idx, url)| {
        let page: String = format!("{}-page{}.png", &chapter.slug, idx);
        let filename: PathBuf = folder.join(&page);
        let filename: String = filename
            .to_str()
            .expect(&format!(
                "Unable to join path to manga: {}/{}",
                folder.to_str().unwrap(),
                &page
            ))
            .to_owned();

        if !Path::new(&filename).exists() {
            println!("{}", &filename);
            save_image(url, &filename)
                .expect(&format!("Unable to download and save file: {}", &filename));
        }
        if minimize_image(&filename).is_err() {
            println!("Error minimizing {}, skipping.", &filename);
        }
    });
    Ok(())
}

fn save_image(url: &str, filename: &str) -> Result<(), io::Error> {
    if let Ok(body) = util::get_image(url) {
        let mut file: File = File::create(filename)?;
        file.write_all(&body)?;
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Could not download image!",
        ))
    }
}

fn minimize_image(filename: &str) -> Result<(), imagequant::Error> {
    let file: File = fs::File::open(&filename)
        .expect(&format!("Could not open image for reading: {}", &filename));
    let meta: Metadata = file
        .metadata()
        .expect(&format!("Could not load metadata for image: {}", &filename));
    let original_size: u64 = meta.len();

    let bitmap = lodepng::decode32_file(&filename).expect(&format!(
        "Unable to decode image for compression: {}",
        &filename
    ));

    let mut attr: Attributes = imagequant::new();
    attr.set_quality(50, 80)?;

    let mut image: imagequant::Image =
        attr.new_image(bitmap.buffer.to_vec(), bitmap.width, bitmap.height, 0.0)?;

    let mut quant: imagequant::QuantizationResult = match attr.quantize(&mut image) {
        Ok(quant) => quant,
        Err(_err) => return Ok(()),
    };
    quant.set_dithering_level(1.0)?;

    let (palette, pixels) = quant.remapped(&mut image)?;

    let mut encoder: Encoder = lodepng::Encoder::new();
    encoder.set_palette(&palette).expect(&format!(
        "Unable to set palette for minimized image: {}",
        &filename
    ));
    let png_pixels: Vec<u8> =
        encoder
            .encode(&pixels, bitmap.width, bitmap.height)
            .expect(&format!(
                "Unable to encode pixels for minimize image: {}",
                &filename
            ));

    let mut file: File =
        File::create(&filename).expect(&format!("Could not open image for writing: {}", &filename));
    file.write_all(&png_pixels)
        .expect(&format!("Unable to save minimized image: {}", &filename));

    let meta: Metadata = file.metadata().expect(&format!(
        "Could not load metadata for minimized image: {}",
        &filename
    ));
    let diff: u64 = (100.0 - (meta.len() as f64 / original_size as f64) * 100.0) as u64;
    println!("{} {}%", &filename, diff);

    Ok(())
}
