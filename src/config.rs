use crate::data::{Group, Project};
use crate::gitlab_api::request_one;
use anyhow::Context;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use regex::Regex;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(Deserialize, Debug)]
pub struct ConfFile {
    url: String,
    token: String,
    milestones: Vec<String>,
    parameters: Vec<(String, String)>,
    output: String,
}

#[derive(Debug)]
pub struct Config {
    pub base_url: String,
    pub token: String,
    pub path: String,
    pub parameters: Vec<(String, String)>,
    pub output: String,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "gitlab tt arguments")]
pub struct Args {
    #[structopt(short, long)]
    config: Option<String>,
}

impl Config {
    pub async fn get() -> anyhow::Result<Config> {
        let path = Args::from_args()
            .config
            .unwrap_or("config.toml".to_string());
        let config = toml::from_str::<ConfFile>(&std::fs::read_to_string(path)?)?;
        let captures =
            Regex::new(r"(?:https?://|git@)([\w.-]+\.\w+)[/:]([\w\-./]+?)(?:\.git)?/?$")?
                .captures(&config.url)
                .context("Invalid URL")?;
        if captures.len() < 3 {
            anyhow::bail!("Missing Captures");
        }
        let params: Vec<(String, String)> = [
            config.parameters,
            config
                .milestones
                .iter()
                .map(|x| ("milestone".to_string(), x.to_owned()))
                .collect(),
        ]
        .concat();
        let base_url = captures[1].to_string();
        let path = utf8_percent_encode(&captures[2], NON_ALPHANUMERIC).to_string();
        let location = if let Ok(project) = request_one::<Project>(
            format!("https://{base_url}/api/v4/projects/{}", path),
            &config.token,
        )
        .await
        {
            format!("projects/{}", project.id)
        } else if let Ok(group) = request_one::<Group>(
            format!("https://{base_url}/api/v4/groups/{}", path),
            &config.token,
        )
        .await
        {
            format!("groups/{}", group.id)
        } else {
            anyhow::bail!("No project or group found")
        };
        Ok(Config {
            base_url,
            token: config.token,
            path: location,
            parameters: params,
            output: config.output,
        })
    }

    pub fn parameters(&self) -> Vec<(&str, &str)> {
        self.parameters
            .iter()
            .map(|(n, v)| (n.as_str(), v.as_str()))
            .collect::<Vec<(&str, &str)>>()
    }

    pub fn token(&self) -> String {
        self.token.to_owned()
    }

    pub fn url(&self) -> String {
        self.base_url.to_owned()
    }

    pub fn endpoint(&self) -> String {
        self.path.to_owned()
    }
}
