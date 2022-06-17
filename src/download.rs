use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use image::GenericImageView;

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

fn minimize_image(filename: &str) -> Result<(), io::Error> {
    let img = image::open(&filename).unwrap();
    let dim = img.dimensions();

    let mut rgba: Vec<imagequant::RGBA> = Vec::new();
    for i in img.pixels() {
        let p = i.2;
        rgba.push(imagequant::RGBA {
            r: p.0[0],
            g: p.0[1],
            b: p.0[2],
            a: p.0[3],
        });
    }

    let mut lib = imagequant::new();
    lib.set_quality(70, 90).unwrap();

    let mut img2 = lib.new_image(rgba, dim.0.try_into().unwrap(), dim.1.try_into().unwrap(), 0.0)
        .unwrap();
    let mut res = lib.quantize(&mut img2).unwrap();
    res.set_dithering_level(1.0).unwrap();

    let (_pal, pixels) = res.remapped(&mut img2).unwrap();

    //image::save_buffer("test.png", &pixels, dim.0.try_into().unwrap(), dim.1.try_into().unwrap(), image::ColorType::Rgb8).unwrap();
    let mut file: File = File::create("test.png")?;
    file.write_all(&pixels)?;
    Ok(())
}