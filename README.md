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
## Series Module
The `series` module is responsible for looking up all the supported series from TCBScans.
```
use tcbscans::series::{self, Series};

let all_series: Vec<Series> = series::get_series();
for s in all_series {
    println!("{:?}", &s);
}
```
```
Series { id: "1", name: "Ace Novel - Manga Adaptation", slug: "ace-novel-manga-adaptation" }
Series { id: "8", name: "Attack on Titan", slug: "attack-on-titan" }
Series { id: "3", name: "Black Clover", slug: "black-clover" }
```

## Chapters Module
The `chapters` module is responsible for looking up all the available chapters from a series.
```
use tcbscans::series:Series;
use tcbscans::chapters::{self, Chapter};

let aot: Series = Series {
    id: "8", name: "Attack on Titan", slug: "attack-on-titan"
};
let all_chapters: Vec<Chapter> = chapters::get_chapters(aot);
for chapter in all_chapters {
    println!("{:?}", &chapter);
}
```
```
Chapter { id: "30", name: "Attack on Titan Chapter 139: Moving Towards that Tree on the Hill", slug: "attack-on-titan-chapter-139" }
```
