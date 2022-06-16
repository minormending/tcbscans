use regex::Regex;
use serde::Serialize;

use crate::series::Series;
use crate::util;

#[derive(Serialize, Debug)]
pub struct Chapter {
    pub id: String,
    pub name: String,
    pub slug: String,
}

pub fn get_chapters(manga: &Series) -> Vec<Chapter> {
    let url: String = format!(
        "https://onepiecechapters.com/mangas/{}/{}",
        manga.id, manga.slug
    );
    let html = util::get_page(&url).expect("Unable to get chapters from site.");
    get_chapters_from_page(&html)
}

fn get_chapters_from_page(page: &str) -> Vec<Chapter> {
    let re_chapter: Regex =
        Regex::new(r#"(?s)href="/chapters/(\d*)/([^"]*)"[^>]*>(.*?)</a>"#).unwrap();
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
