use serde::Deserialize;

#[derive(Debug)]
pub struct TimeEntry {
    pub author: Author,
    pub ticket: Issue,
    pub time_spent: isize,
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Author {
    pub id: u64,
    pub username: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Note {
    pub body: String,
    pub author: Author,
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct Issue {
    pub iid: u64,
    pub project_id: u64,
    pub title: String,
    pub time_stats: Option<TimeStats>,
    pub milestone: Option<Milestone>,
    pub web_url: String,
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct Milestone {
    pub title: String,
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct TimeStats {
    pub total_time_spent: Option<u64>,
}

#[derive(Deserialize)]
pub struct Project {
    pub id: u64,
    /*pub name: String,
    pub web_url: String,
    pub path_with_namespace: String,*/
}

#[derive(Deserialize)]
pub struct Group {
    pub id: u64,
    //pub full_path: String,
}
