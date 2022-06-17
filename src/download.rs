use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use crate::chapters::Chapter;
use crate::manga;
use crate::util;

pub fn save_chapter_pages(chapter: &Chapter, directory: &str) -> Result<(), io::Error> {
    let folder = Path::new(&directory).join(&chapter.slug);
    if folder.exists() {
        println!("We have already downloaded the chapter, skipping...");
        //return Ok(());
    }
    fs::create_dir_all(&folder)?;

    let images: Vec<String> = manga::get_manga_pages(chapter);
    for (idx, url) in images.iter().enumerate() {
        let page = format!("{}-page{}.png", &chapter.slug, idx);
        let filename = folder
            .join(&page)
            .to_str()
            .expect("Unable to create path to manga.")
            .to_owned();
        println!("{}", &filename);
        if !Path::new(&filename).exists() {
            save_image(url, &filename)?;
        }
        minimize_image(&filename).unwrap();
    }
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
    let bitmap = lodepng::decode32_file(&filename)
        .expect("Unable to decode page for compression.");

    let mut attr = imagequant::new();
    attr.set_quality(60, 80)?;

    let mut image = attr
        .new_image(bitmap.buffer.to_vec(), bitmap.width, bitmap.height, 0.0)?;

    let mut quant = attr.quantize(&mut image)?;
    quant.set_dithering_level(1.0)?;

    let (palette, pixels) = quant.remapped(&mut image)?;

    let mut encoder = lodepng::Encoder::new();
    encoder.set_palette(&palette).unwrap();
    let png_pixels = encoder
        .encode(&pixels, bitmap.width, bitmap.height)
        .unwrap();

    let mut file: File = File::create(&filename).unwrap();
    file.write_all(&png_pixels).unwrap();

    Ok(())
}
