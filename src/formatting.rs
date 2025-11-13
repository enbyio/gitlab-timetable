use crate::data::{Author, TimeEntry};
use std::collections::HashMap;

struct TableEntry {
    title: String,
    time_spent: isize,
    ticket: String,
    ticket_link: String,
}

pub struct PersonEntry {
    pub user: Author,
    total_time_spent: isize,
    tickets: Vec<TableEntry>,
}

impl std::fmt::Display for PersonEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "== {}\n{} h\n", self.user.name, (self.total_time_spent as f64 / 60.0).round() as i32)?;
        writeln!(f, "|===\n|Thema |Issues |Prozentangabe\n")?;
        self.tickets.iter().for_each(|e| {
            let percentage = e.time_spent * 100 / self.total_time_spent;
            _ = writeln!(f, "{e}| {}\n", percentage.max(1));
        });
        writeln!(f, "|===\n")?;
        Ok(())
    }
}

impl std::fmt::Display for TableEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "| {} \n| {}[{}] \n",
            self.title, self.ticket_link, self.ticket
        )
    }
}

pub fn generate_time_table(times: Vec<TimeEntry>) -> Vec<PersonEntry> {
    let mut entries: HashMap<Author, (isize, Vec<TableEntry>)> = HashMap::new();
    for time in times {
        let author = time.author.clone();
        let entry = entries
            .entry(author)
            .or_insert_with(|| (0isize, Vec::new()));
        let te = generate_entry(&time);
        entry.0 += te.time_spent;
        entry.1.push(te);
    }
    let mut people: Vec<PersonEntry> = Vec::new();
    for (k, (v, vv)) in entries {
        people.push(PersonEntry {
            user: k,
            total_time_spent: v,
            tickets: vv,
        })
    }
    people.sort_by_key(|e| e.user.name.clone());
    people
}

fn generate_entry(entry: &TimeEntry) -> TableEntry {
    let parts = entry.ticket.web_url.split('/').collect::<Vec<_>>();
    TableEntry {
        title: entry.ticket.title.clone(),
        time_spent: entry.time_spent,
        ticket: format!("{}#{}", parts[parts.len() - 4], entry.ticket.iid),
        ticket_link: entry.ticket.web_url.clone(),
    }
}
