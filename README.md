# TCBScans Manga Viewer
Retrieves manga chapters for an input chapter and series data. Series and chapter ids can be retrieved from this tool too.

DISCLAIMER: I am not licensed or affiliated with TCBScans or Viz Media and this repository is meant for informational purposes only. Please delete the retrieved pages after reading.

# Installation
```
// Cargo.toml
[dependencies]
tcbscans = "0.1.0"
```

# Usage
Available methods:
- `series::get_series()`: Retrieves the list of available series from TCBScans.
- `chapters::get_chapters(...)`: Retrieves all available chapters for a series.
- `manga::get_pages(...)`: Retrieves all page links for a chapter from a series.
- `download::save_chapter_pages(...)`: Save the mange page images for a chapter.

## Series Module
The `series` module is responsible for looking up all the supported series from TCBScans. The returned `id` and `slug` parameters can be used to retrieve chapter and manga info.
### Example
```
use tcbscans::series::{self, Series};

let all_series: Vec<Series> = series::get_series();
for s in all_series {
    println!("{:?}", &s);
}
```
### Results
```
Series { id: "1", name: "Ace Novel - Manga Adaptation", slug: "ace-novel-manga-adaptation" }
Series { id: "8", name: "Attack on Titan", slug: "attack-on-titan" }
Series { id: "3", name: "Black Clover", slug: "black-clover" }
```

## Chapters Module
The `chapters` module is responsible for looking up all the available chapters from a series. Options for `chapters::get_chapters(..)`:
- `series_obj: &series::Series` = The `Series` object to look up, containing primarily the `slug`. The other fields are optionally used for reporting.
### Example
```
use tcbscans::series:Series;
use tcbscans::chapters::{self, Chapter};

let aot: Series = Series {
    id: "8", name: "Attack on Titan", slug: "attack-on-titan"
};
let all_chapters: Vec<Chapter> = chapters::get_chapters(&aot);
for chapter in all_chapters {
    println!("{:?}", &chapter);
}
```
### Results
```
Chapter { id: "30", name: "Attack on Titan Chapter 139: Moving Towards that Tree on the Hill", slug: "attack-on-titan-chapter-139" }
```

## Manga Module
The `manga` module is responsible for looking up all the page links in a chapter from a series. Options for `manga::get_pages(...)` are:
- `chapter: &chapters::Chapter` = The `Chapter` object used to look up the pages of the chapter, containing primarily the slug of the chapter. The other fields are optionally used for reporting.
### Example
```
use tcbscans::chapters::Chapter;
use tcbscans::manga;

let aot: Chapter = Chapter {
    id: "", name: "", slug: "attack-on-titan-chapter-139"
};
let all_pages: Vec<String> = manga::get_pages(&aot);
for link in all_pages {
    println!("{}", &link);
}
```
### Results
```
https://cdn.onepiecechapters.com/file/CDN-M-A-N/aotv2_139_vol_01.png
https://cdn.onepiecechapters.com/file/CDN-M-A-N/aotv2_139_vol_02.png
https://cdn.onepiecechapters.com/file/CDN-M-A-N/aotv2_139_vol_03.png
```

## Download Module
The `download` module is responsible for downloading and minimizing the chapter pages. The chapter pages will be downloaded into a folder named after the chapter `slug`. Image minimization is performed using the `imagequant` crate. Options for `save_chapter_pages(...)` are:
- `chapter: &chapters::Chapter` = The `Chapter` object used to look the pahes of the chapter, containing primarily the slug of the chapter. The other fields are optionally used for reporting.
- `directory: &str` = The parent directory to save all the chapter pages. A folder with the chapter slug will be created in this directory. If a chapter directory is already created, the chapter will not be redownloaded.
### Example
```
use tcbscans::chapters::Chapter;
use tcbscans::download;

let aot: Chapter = Chapter {
    id: "", name: "", slug: "attack-on-titan-chapter-139"
};
let dir = "images/";
download::save_chapter_pages(&aot, dir);
```
### Results
```
images/attack-on-titan-chapter-139/attack-on-titan-chapter-139-page28.png
images/attack-on-titan-chapter-139/attack-on-titan-chapter-139-page42.png
images/attack-on-titan-chapter-139/attack-on-titan-chapter-139-page49.png
```

# CLI Usage
This crate is also packaged with a CLI tool for exploring the TCBScans data.
```
Download manga chapters from TCBScans

USAGE:
    tcbscans <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    chapters    List all chapters for a supported series
    help        Print this message or the help of the given subcommand(s)
    manga       Save an entire chapter to disk
    series      List all supported series
```

## Get Series
```
>>> tcbscans series

{"id":"1","name":"Ace Novel - Manga Adaptation","slug":"ace-novel-manga-adaptation"}
{"id":"8","name":"Attack on Titan","slug":"attack-on-titan"}
{"id":"3","name":"Black Clover","slug":"black-clover"}
```

## Get Chapters
```
>>> tcbscans chapters --help
List all chapters for a supported series

USAGE:
    tcbscans chapters [OPTIONS]

OPTIONS:
        --download <DOWNLOAD>    Download all chapters of the series to a directory
    -h, --help                   Print help information
    -i, --id <ID>                Id of the series
    -s, --slug <SLUG>            Slug name of the series

>>> tcbscans chapters --id 8

{"id":"30","name":"Attack on Titan Chapter 139: Moving Towards that Tree on the Hill","slug":"attack-on-titan-chapter-139"}
```

## Download Manga
```
>>> tcbscans manga --help
Save an entire chapter to disk

USAGE:
    tcbscans manga [OPTIONS] <SERIES> <DIRECTORY>

ARGS:
    <SERIES>       Slug name of the series
    <DIRECTORY>    Directory to save the chapter

OPTIONS:
    -h, --help           Print help information
    -i, --id <ID>        Id of the chapter
    -s, --slug <SLUG>    Slug name of the chapter

>>> tcbscans manga attack-on-titan --id 30 images/

images/attack-on-titan-chapter-139/attack-on-titan-chapter-139-page28.png
images/attack-on-titan-chapter-139/attack-on-titan-chapter-139-page42.png
images/attack-on-titan-chapter-139/attack-on-titan-chapter-139-page49.png
```

# Docker
The CLI can be built and run via a Docker container. Pre-built containers can be found at:
https://hub.docker.com/r/minormending/tcbscans

```
>>> docker build -t tcbscans .
>>> docker run -v /home/user/images/:/images/:rw tcbscans manga  attack-on-titan --id 30 /images/

/images/attack-on-titan-chapter-139/attack-on-titan-chapter-139-page28.png
/images/attack-on-titan-chapter-139/attack-on-titan-chapter-139-page42.png
/images/attack-on-titan-chapter-139/attack-on-titan-chapter-139-page49.png
```
