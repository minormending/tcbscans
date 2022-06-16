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
        return Ok(());
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
