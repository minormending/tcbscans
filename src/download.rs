use std::{
    fs::{self, File},
    io::{self, Write},
};

use crate::chapters::Chapter;
use crate::manga;
use crate::util;

pub fn save_chapter_pages(chapter: &Chapter, directory: &str) -> Result<(), io::Error> {
    fs::create_dir_all(directory)?;

    let images: Vec<String> = manga::get_manga_pages(chapter);
    for (idx, url) in images.iter().enumerate() {
        let filename = format!("{}/{}-page{}.png", directory, chapter.slug, idx);
        println!("{}", &filename);
        save_image(url, &filename)?;
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
