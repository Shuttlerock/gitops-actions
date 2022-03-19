use octocrab;
use octocrab::params;
use std::error::Error;
use regex::Regex;

pub async fn merge_pull_request(
    url: &str,
    source_branch: &str,
    target_branch: &str,
    title: &str,
    body: &str,
    token: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    // Parse owner and repository name from url to work with github API.
    let re = Regex::new(r"^(?:(?:https?|ssh|git|ftps?)://)?(?:(?:[^/@]+)@)?(?:[^/:]+)[/:]([^/:]+)/(.+).git/?$")?;

    let group = re
        .captures_iter(url)
        .next()
        .ok_or(format!("invalid git url specified: {}", url))?;

    let owner = group
        .get(1)
        .ok_or(format!("failed to parse owner from url: {}", url))?;

    let repo = group
        .get(2)
        .ok_or(format!("failed to parse name from url: {}", url))?;

    let mut builder = octocrab::OctocrabBuilder::new();

    if let Some(token) = token {
        builder = builder.personal_token(token.to_string());
    }

    let octocrab = builder.build()?;

    // Create PR.
    let pr = octocrab
        .pulls(owner.as_str(), repo.as_str())
        .create(title, source_branch, target_branch)
        .body(body)
        .send()
        .await?;

    // Merge PR.
    octocrab
        .pulls(owner.as_str(), repo.as_str())
        .merge(pr.number)
        .method(params::pulls::MergeMethod::Squash)
        .send()
        .await?;

    Ok(())
}