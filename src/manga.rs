use regex::Regex;

use crate::chapters::Chapter;
use crate::util;

pub fn get_manga_pages(chapter: &Chapter) -> Vec<String> {
    let url: String = format!(
        "https://onepiecechapters.com/chapters/{}/{}",
        chapter.id, chapter.slug
    );
    let html: String = util::get_page(&url).expect("Unable to get chapter pages");
    get_images_from_page(&html)
}

fn get_images_from_page(page: &str) -> Vec<String> {
    let re: Regex =
        Regex::new(r#"(?s)<picture[^>]*>.*?<img[^>]*?src="([^"]+)"[^>]*>.*?</picture>"#).unwrap();

    let mut images: Vec<String> = Vec::new();
    for cap in re.captures_iter(page) {
        images.push(cap[1].to_string());
    }
    images
}
