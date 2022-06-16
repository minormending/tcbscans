use minreq::{Error, Request, Response};
use regex::Regex;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Manga {
    pub id: String,
    pub name: String,
    pub slug: String,
}

pub fn get_mangas() -> Vec<Manga> {
    let html: String = get_mangas_page().expect("Unable to get mangas from site.");
    get_mangas_from_page(&html)
}

pub fn get_chapters(manga: &Manga) -> Vec<Manga> {
    let html = get_chapters_page(&manga).expect("Unable to get chapters from site.");
    get_chapters_from_page(&html)
}

fn get_page(url: &str) -> Result<String, Error> {
    let req: Request = minreq::get(url);
    let res: Response = req.send()?;
    let body: &str = res.as_str()?;
    Ok(String::from(body))
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

fn get_chapters_from_page(page: &str) -> Vec<Manga> {
    let re: Regex = Regex::new(r#"(?s)href="/chapters/(\d*)/([^"]*)"[^>]*>(.*?)</a>"#).unwrap();

    let mut chapters: Vec<Chapter> = Vec::new();
    for cap in re.captures_iter(page) {
        if cap[3].trim().is_empty() {
            continue;
        }
        chapters.push(Manga {
        chapters.push(Chapter {
            id: cap[1].to_string(),
            name: name,
            slug: cap[2].to_string(),
        });
    }
    chapters
}
