use clap::{Parser, Subcommand};
use tcbscans::{
    chapters::{self, Chapter},
    download,
    series::{self, Series},
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Actions,
}

#[derive(Subcommand, Debug)]
enum Actions {
    /// List all supported series
    Series,
    /// List all chapters for a supported series
    Chapters {
        /// Slug name of the series
        #[clap(short, long)]
        slug: Option<String>,
        /// Id of the chapter
        #[clap(short, long)]
        id: Option<u8>,
    },
    /// Save a chapter to disk
    Chapter {
        /// Slug name of the series
        #[clap()]
        series: String,
        /// Slug name of the chapter
        #[clap(short = 's', long)]
        slug: Option<String>,
        /// Id of the chapter
        #[clap(short, long)]
        id: Option<u32>,
        /// Directory to save the chapter
        #[clap()]
        directory: String,
    },
}

fn main() {
    let cli = Args::parse();
    match &cli.action {
        Actions::Series => {
            let mangas: Vec<Series> = series::get_series();
            for manga in mangas {
                let json = serde_json::to_string(&manga)
                    .expect(&format!("Unable to serialize manga: {:?}", &manga));
                println!("{}", &json);
            }
        }
        Actions::Chapters { slug, id } => {
            let mangas: Vec<Series> = series::get_series();
            let mut manga: Option<&Series> = None;
            if let Some(slug) = slug {
                manga = mangas.iter().find(|&m| &m.slug == slug);
            }
            if manga.is_none() {
                if let Some(id) = id {
                    let id: String = format!("{}", id);
                    manga = mangas.iter().find(|&m| &m.id == &id);
                }
            }

            let manga: &Series = manga.expect("Unable to find manga using slug or id.");
            let chapters: Vec<Chapter> = chapters::get_chapters(manga);
            for chapter in chapters {
                let json = serde_json::to_string(&chapter).unwrap();
                println!("{}", &json)
            }
        }
        Actions::Chapter {
            series,
            slug,
            id,
            directory,
        } => {
            let mangas: Vec<Series> = series::get_series();
            let manga = mangas
                .iter()
                .find(|&m| &m.slug == series)
                .expect("Could not find series!");

            let chapters: Vec<Chapter> = chapters::get_chapters(manga);
            let mut chapter: Option<&Chapter> = None;
            if let Some(slug) = slug {
                chapter = chapters.iter().find(|&c| &c.slug == slug);
            }
            if chapter.is_none() {
                if let Some(id) = id {
                    let id: String = format!("{}", id);
                    chapter = chapters.iter().find(|&c| &c.id == &id);
                }
            }
            let chapter = chapter.expect("Could not find chapter!");
            download::save_chapter_pages(chapter, directory).expect("Unable to save chapter!");
        }
    }
}
