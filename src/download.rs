use rayon::prelude::*;
use std::{
    fs::{self, File},
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
    let meta = fs::File::open(&filename).unwrap().metadata().unwrap();
    //println!("initial file length: {}", meta.len() / 1024);
    let original_size = meta.len();

    let bitmap = lodepng::decode32_file(&filename).expect("Unable to decode page for compression.");

    let mut attr = imagequant::new();
    attr.set_quality(50, 80)?;

    let mut image = attr.new_image(bitmap.buffer.to_vec(), bitmap.width, bitmap.height, 0.0)?;

    let mut quant = match attr.quantize(&mut image) {
        Ok(quant) => quant,
        Err(_err) => return Ok(()),
    };
    quant.set_dithering_level(1.0)?;

    let (palette, pixels) = quant.remapped(&mut image)?;

    let mut encoder = lodepng::Encoder::new();
    encoder.set_palette(&palette).unwrap();
    let png_pixels = encoder
        .encode(&pixels, bitmap.width, bitmap.height)
        .unwrap();

    let mut file: File = File::create(&filename).unwrap();
    file.write_all(&png_pixels).unwrap();

    let meta = fs::File::open(&filename).unwrap().metadata().unwrap();
    println!(
        "{} {}%",
        &&filename,
        (100.0 - (meta.len() as f64 / original_size as f64) * 100.0) as u64
    );

    Ok(())
}
