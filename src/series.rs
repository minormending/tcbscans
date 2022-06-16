use regex::Regex;
use serde::Serialize;

use crate::util;

#[derive(Serialize, Debug)]
pub struct Series {
    pub id: String,
    pub name: String,
    pub slug: String,
}

pub fn get_series() -> Vec<Series> {
    let html: String = util::get_page("https://onepiecechapters.com/projects")
        .expect("Unable to get mangas from site.");
    get_all_series_from_page(&html)
}

fn get_all_series_from_page(page: &str) -> Vec<Series> {
    let re: Regex = Regex::new(r#"href="/mangas/(\d*)/(.*?)">([^<]*)<"#).unwrap();

    let mut mangas: Vec<Series> = Vec::new();
    for cap in re.captures_iter(page) {
        let name: &str = cap[3].trim();
        if name.is_empty() {
            continue;
        }
        mangas.push(Series {
            id: cap[1].to_string(),
            name: name.to_string(),
            slug: cap[2].to_string(),
        });
    }
    mangas
}
