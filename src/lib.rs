use std::{fs::{OpenOptions, File}, io::Write};

use minreq::{Error, Request, Response};
use regex::Regex;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Manga {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Debug)]
pub struct Chapter {
    pub id: String,
    pub name: String,
    pub slug: String,
}

pub fn get_mangas() -> Vec<Manga> {
    let html: String = get_mangas_page().expect("Unable to get mangas from site.");
    get_mangas_from_page(&html)
}

pub fn get_chapters(manga: &Manga) -> Vec<Chapter> {
    let html = get_chapters_page(&manga).expect("Unable to get chapters from site.");
    get_chapters_from_page(&html)
}

pub fn get_chapter_pages(chapter: &Chapter) -> Vec<String> {
    let html = get_chapter_page(&chapter).expect("Unable to get chapter pages");
    get_images_from_page(&html)
}

pub fn save_chapter_pages(chapter: &Chapter) {
    let images = get_chapter_pages(chapter);
    for (idx, url) in images.iter().enumerate() {
        let filename = format!("{}-page{}.png", chapter.slug, idx);
        println!("{}", &filename);
        save_image(url, &filename).unwrap();
    }
}

fn get_page(url: &str) -> Result<String, Error> {
    let req: Request = minreq::get(url);
    let res: Response = req.send()?;
    let body: &str = res.as_str()?;
    Ok(String::from(body))
}

fn save_image(url: &str, filename: &str) -> Result<(), Error> {
    let req: Request = minreq::get(url);
    let res: Response = req.send()?;
    let body = res.as_bytes();
    let mut file = File::create(filename)?;
    file.write_all(body).unwrap();
    Ok(())
}

fn get_mangas_page() -> Result<String, Error> {
    get_page("https://onepiecechapters.com/projects")
}

fn get_chapters_page(manga: &Manga) -> Result<String, Error> {
    let url: String = format!(
        "https://onepiecechapters.com/mangas/{}/{}",
        manga.id, manga.slug
    );
    get_page(&url)
}

fn get_chapter_page(chapter: &Chapter) -> Result<String, Error> {
    let url: String = format!(
        "https://onepiecechapters.com/chapters/{}/{}",
        chapter.id, chapter.slug
    );
    get_page(&url)
}

fn get_mangas_from_page(page: &str) -> Vec<Manga> {
    let re: Regex = Regex::new(r#"href="/mangas/(\d*)/(.*?)">([^<]*)<"#).unwrap();

    let mut mangas: Vec<Manga> = Vec::new();
    for cap in re.captures_iter(page) {
        let name: &str = cap[3].trim();
        if name.is_empty() {
            continue;
        }
        mangas.push(Manga {
            id: cap[1].to_string(),
            name: name.to_string(),
            slug: cap[2].to_string(),
        });
    }
    mangas
}

fn get_chapters_from_page(page: &str) -> Vec<Chapter> {
    let re_chapter: Regex = Regex::new(r#"(?s)href="/chapters/(\d*)/([^"]*)"[^>]*>(.*?)</a>"#).unwrap();
    let re_name: Regex = Regex::new(r"<div[^>]*>\s*([^<]+)\s*</div>").unwrap();

    let mut chapters: Vec<Chapter> = Vec::new();
    for cap in re_chapter.captures_iter(page) {
        let name_html: &str = cap[3].trim();
        if name_html.is_empty() {
            continue;
        }

        // sometimes the name is `Series Chapter X: Title`
        let names = re_name
            .captures_iter(name_html)
            .map(|c| c[1].to_string())
            .collect::<Vec<String>>();
        let name: String = names.join(": ");

        chapters.push(Chapter {
            id: cap[1].to_string(),
            name: name,
            slug: cap[2].to_string(),
        });
    }
    chapters
}

fn get_images_from_page(page: &str) -> Vec<String> {
    let re: Regex = Regex::new(r#"(?s)<picture[^>]*>.*?<img[^>]*?src="([^"]+)"[^>]*>.*?</picture>"#).unwrap();
    
    let mut images: Vec<String> = Vec::new();
    for cap in re.captures_iter(page) {
        images.push(cap[1].to_string());
    }
    images
}