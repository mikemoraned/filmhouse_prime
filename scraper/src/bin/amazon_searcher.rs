extern crate clap;
extern crate time;

use url::{ParseError, Url};

use clap::{App, Arg};

use scraper::{Html, Selector};

use serde::{Deserialize, Serialize};

use std::fs::File;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
struct Film {
    title: String,
    director: String,
    amazon_link: Option<String>,
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

    let link_selector = Selector::parse("a.a-link-normal").unwrap();
    let text_selector = Selector::parse("span.a-text-normal").unwrap();

    let client = reqwest::Client::new();

    let mut amazon_films = vec![];

    for film in films {
        let search_terms = format!(
            "{}+{}",
            film.title.replace(" ", "+"),
            film.director.replace(" ", "+")
        );
        let url_string = format!("https://www.amazon.co.uk/s?k={}&i=instant-video&bbn=3010086031&rh=n%3A3010085031%2Cn%3A%213010086031%2Cn%3A3046737031&dc&qid=1570310294&rnid=3010086031",
        search_terms);
        let base_url = Url::parse(&url_string).unwrap();
        println!("url = {}", url_string);
        let body = client.get(&url_string).send()?.text()?;

        let document = Html::parse_document(&body);

        for link_element in document.select(&link_selector) {
            let href = link_element.value().attr("href").unwrap();
            for text_element in link_element.select(&text_selector) {
                let text = text_element.text().collect::<Vec<_>>().join(" ");
                println!("href = {}, text = {:?}", href, text);
                let absolute_url = base_url.join(&href).unwrap();
                println!("abs = {}", absolute_url);

                amazon_films.push(Film {
                    title: film.title.clone(),
                    director: film.director.clone(),
                    amazon_link: Some(absolute_url.to_string()),
                });
            }
        }
    }

    let films_file = File::create("amazon_films.json")?;
    serde_json::to_writer_pretty(films_file, &amazon_films)?;

    Ok(())
}
