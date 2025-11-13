mod config;
mod data;
mod formatting;
mod gitlab_api;
mod util;

use data::*;
use std::fs::File;
use std::io::Write;
use std::sync::OnceLock;
use std::time::Instant;

use crate::config::Config;
use crate::formatting::generate_time_table;
use crate::gitlab_api::{get_issues, get_notes_for_issue};
use crate::util::{config, process_issue};
use futures::future;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[tokio::main]
async fn main() {
    if let Err(e) = generate_table().await {
        eprintln!("Failed to run with error: {}", e);
    }
}

async fn generate_table() -> anyhow::Result<()> {
    CONFIG
        .set(Config::get().await?)
        .expect("Config is already set, aborting...");
    let start = Instant::now();
    let tickets = get_issues().await?;
    println!("Found {:?} tickets", tickets.len());
    let futures = tickets.iter().map(get_notes_for_issue);
    let entries: Vec<(Issue, Vec<Note>)> = future::try_join_all(futures).await?;
    let times: Vec<TimeEntry> = entries
        .iter()
        .flat_map(|(id, notes)| process_issue(notes.clone(), id).into_iter())
        .collect();
    let t = generate_time_table(times);
    let mut file = File::create(&config().output)?;
    writeln!(file, ":toc:")?;
    t.iter().for_each(|e| {
        println!("{e}");
        writeln!(file, "{e}").unwrap();
    });
    println!("took {} seconds", start.elapsed().as_secs());
    Ok(())
}
