extern crate clap;
extern crate time;

use clap::{App, Arg};

use chrono::prelude::*;
use scraper::{Html, Selector};
use time::Duration;

use serde::{Deserialize, Serialize};

use std::collections::HashSet;

use std::fs::File;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
struct Film {
    title: String,
    director: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("filmhouse_scraper")
        .about("get all films shown at Filmhouse")
        .author("Mike Moran")
        .arg(
            Arg::with_name("start_date")
                .long("start_date")
                .help("Start date to scrape from")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("days")
                .long("days")
                .help("Number of consecutive days")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let start_date_formatted = matches.value_of("start_date").unwrap();
    let start_date = NaiveDate::parse_from_str(start_date_formatted, "%Y-%m-%d")?;
    let days_formatted = matches.value_of("days").unwrap();
    let days: i64 = days_formatted.parse()?;

    println!("start_date = {}, days = {}", start_date, days);

    let mut films = HashSet::new();
    let client = reqwest::Client::new();

    for day_offset in 0..days {
        let date = start_date + Duration::days(day_offset);
        let url = format!(
            "https://www.filmhousecinema.com/whats-on/{}",
            date.format("%Y-%m-%d").to_string()
        );
        println!("url = {}", url);
        let body = client.get(&url).send()?.text()?;

        let document = Html::parse_document(&body);

        let event_selector = Selector::parse("div[itemtype=\"http://schema.org/Event\"]").unwrap();
        let title_selector = Selector::parse(".field--name-title").unwrap();
        let director_selector = Selector::parse(".attr-director .attr").unwrap();

        for event_element in document.select(&event_selector) {
            for title_element in event_element.select(&title_selector) {
                let title = title_element.text().collect::<Vec<_>>().join(" ");
                println!("title = {:?}", title);
                for director_element in event_element.select(&director_selector) {
                    let director = director_element.text().collect::<Vec<_>>().join(" ");
                    println!("director = {:?}", director);
                    films.insert(Film {
                        title: title.clone(),
                        director: director.clone(),
                    });
                }
            }
        }
    }
    println!("films = {:?}", films);
    let films_file = File::create("films.json")?;
    serde_json::to_writer_pretty(films_file, &films)?;

    Ok(())
}
