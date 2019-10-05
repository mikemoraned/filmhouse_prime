extern crate clap;
extern crate time;

use clap::{App, Arg};

use scraper::{Html, Selector};

use serde::{Deserialize, Serialize};

use std::fs::File;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
struct Film {
    title: String,
    director: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("amazon_searcher")
        .about("find films on Amazon Prime")
        .author("Mike Moran")
        .arg(
            Arg::with_name("films")
                .long("films")
                .help("file containing films in JSON format")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let films_file = matches.value_of("films").unwrap();
    let films_file = File::open(films_file)?;
    let films: Vec<Film> = serde_json::from_reader(films_file)?;

    let client = reqwest::Client::new();
    let url = "https://www.amazon.co.uk/s?k=Happy+As+Lazzaro+Alice+Rohrwacher&i=instant-video&bbn=3010086031&rh=n%3A3010085031%2Cn%3A%213010086031%2Cn%3A3046737031&dc&qid=1570310294&rnid=3010086031";
    println!("url = {}", url);
    let body = client.get(url).send()?.text()?;

    let document = Html::parse_document(&body);

    let link_selector = Selector::parse("a.a-link-normal").unwrap();
    let text_selector = Selector::parse("span.a-text-normal").unwrap();

    for link_element in document.select(&link_selector) {
        let href = link_element.value().attr("href").unwrap();
        for text_element in link_element.select(&text_selector) {
            let text = text_element.text().collect::<Vec<_>>().join(" ");
            println!("href = {}, text = {:?}", href, text);
        }
    }
    Ok(())
}
