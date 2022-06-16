use clap::{Parser, Subcommand};
use tcbscans::{get_chapters, get_mangas, Manga, Chapter};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Actions,
}

#[derive(Subcommand, Debug)]
enum Actions {
    /// List all supported mangas
    Mangas,
    /// List all chapters for a supported manga
    Chapters {
        /// Slug name of the manga
        #[clap(short, long)]
        slug: Option<String>,
        /// Id of the chapter
        #[clap(short, long)]
        id: Option<u8>,
    },
}

fn main() {
    let cli = Args::parse();
    match &cli.action {
        Actions::Mangas => {
            let mangas: Vec<Manga> = get_mangas();
            for manga in mangas {
                let json = serde_json::to_string(&manga)
                    .expect(&format!("Unable to serialize manga: {:?}", &manga));
                println!("{}", &json);
            }
        }
        Actions::Chapters { slug, id } => {
            let mangas: Vec<Manga> = get_mangas();
            let mut manga: Option<&Manga> = None;
            if let Some(slug) = slug {
                manga = mangas
                    .iter()
                    .find(|&m| &m.slug == slug);
            } 
            if manga.is_none() {
                if let Some(id) = id {
                    let id: String = format!("{}", id);
                    manga = mangas
                        .iter()
                        .find(|&m| &m.id == &id);
                }
            }

            let manga: &Manga = manga.expect("Unable to find manga using slug or id.");
            let chapters: Vec<Chapter> = get_chapters(manga);
            for chapter in chapters {
                let json = serde_json::to_string(&chapter).unwrap();
                println!("{}", &json)
            }
        }
    }
}
