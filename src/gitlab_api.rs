use crate::data::{Issue, Note};
use crate::util::{config, get_client, get_client_wt};
use serde::Deserialize;

pub async fn request_multiple<T>(
    endpoint: &str,
    parameters: Option<&[(&str, &str)]>,
) -> anyhow::Result<Vec<T>>
where
    T: for<'r> Deserialize<'r>,
{
    let request_path = format!("https://{}/api/v4/{}", config().url(), endpoint);
    let page_size = 100usize; // max supported page size by api
    let mut page = 0usize;
    let mut out = Vec::<T>::new();
    loop {
        let mut req = get_client().get(format!("{request_path}?per_page={page_size}&page={page}"));
        if let Some(params) = parameters {
            req = req.query(params);
        }
        let resp = req.send().await?;
        if !resp.status().is_success() {
            let status = &resp.status();
            let body = &resp.text().await.unwrap_or_default();
            panic!(
                "failed with code {} and message {} at endpoint {}",
                status.as_str(),
                body.as_str(),
                request_path
            );
        }
        let next_page = resp
            .headers()
            .get("X-Next-Page")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| {
                if s.is_empty() {
                    None
                } else {
                    s.parse::<usize>().ok()
                }
            });
        let mut page_results: Vec<T> = resp.json().await?;
        let count = page_results.len();

        out.append(&mut page_results);
        if count < page_size {
            break;
        } else if let Some(n) = next_page {
            page = n
        } else {
            page += 1
        }
    }
    Ok(out)
}

pub async fn request_one<T>(request_path: String, token: &str) -> anyhow::Result<T>
where
    T: for<'r> Deserialize<'r>,
{
    let client = get_client_wt(token);
    let resp = client.get(request_path).send().await?;
    Ok(resp.json().await?)
}

pub async fn get_issues() -> anyhow::Result<Vec<Issue>> {
    request_multiple::<Issue>(
        &(config().endpoint() + "/issues/"),
        Some(config().parameters().as_slice()),
    )
    .await
}

pub async fn get_notes_for_issue(issue: &Issue) -> anyhow::Result<(Issue, Vec<Note>)> {
    let issues = request_multiple::<Note>(
        format!("/projects/{}/issues/{}/notes", issue.project_id, issue.iid).as_str(),
        None,
    )
    .await?;
    Ok((
        issue.clone(),
        issues
            .into_iter()
            .filter(|note| {
                (note.body.contains("added") || note.body.contains("deleted"))
                    && note.body.contains("time spent")
            })
            .collect(),
    ))
}
