use crate::CONFIG;
use crate::config::Config;
use crate::data::{Author, Issue, Note, TimeEntry};
use reqwest::{
    self, Client,
    header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT},
};
use std::collections::HashMap;

pub fn config() -> &'static Config {
    CONFIG.get().expect("couldn't access config")
}

pub fn get_client() -> Client {
    get_client_wt(&config().token())
}

pub fn get_client_wt(token: &str) -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("timekeeping-system-test"),
    );
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );
    Client::builder().default_headers(headers).build().unwrap()
}

pub fn match_time_str(times: Vec<&str>) -> isize {
    let mut s = 0;
    for time in times {
        let unit = time.chars().last().unwrap();
        let val = time[..time.len() - 1].parse::<isize>().unwrap();
        s += match unit {
            'm' => val,
            'h' => val * 60,
            'd' => val * 480,
            _ => 0,
        };
    }
    s
}

pub fn process_note(note: Note) -> Option<(Author, isize)> {
    let (time_substr, _) = note.body.split_once(" of")?;
    let mut time: Vec<&str> = time_substr.split(" ").collect();
    let prefix = match time.remove(0) {
        "added" => 1,
        "deleted" => -1,
        val => {
            println!("found unknown prefix {}", val);
            1
        }
    };
    let time = match_time_str(time) * prefix;
    Some((note.author, time))
}

pub fn process_issue(notes: Vec<Note>, ticket: &Issue) -> Vec<TimeEntry> {
    let mut table: HashMap<Author, isize> = HashMap::new();
    for note in notes {
        if let Some((author, time)) = process_note(note) {
            *table.entry(author).or_insert(0) += time;
        }
    }
    table
        .into_iter()
        .map(|(author, val)| TimeEntry {
            author,
            ticket: ticket.clone(),
            time_spent: val,
        })
        .collect()
}
