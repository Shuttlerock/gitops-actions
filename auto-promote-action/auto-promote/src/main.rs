use clap::Parser;
use std::path::{Path, PathBuf};
use std::error::Error;
use uuid::Uuid;

mod config;
mod pr;
mod git;
mod hcl;

use config::Pattern;
use glob::glob;

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short='u', long)]
    git_user: String,

    #[clap(short='e', long)]
    git_email: String,

    #[clap(short='p', long)]
    git_password: Option<String>,

    #[clap(short, long)]
    config: std::path::PathBuf,

    /// Key value pairs of the form 'key=value'
    #[clap(short, parse(try_from_str = parse_key_val), multiple_occurrences(true))]
    values: Vec<(String, String)>,
}

// Get all file paths matching the unix glob pattern.
fn glob_files(pattern: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let paths = glob(pattern)?
        .filter(|path| match path {
            Err(_) => true,
            Ok(path) => if path.is_file() { true } else { false },
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(paths)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let cfg = config::from_path(&args.config)?;

    let repository_str = "/tmp/repository";
    let repository_path = Path::new(&repository_str);

    // Process all enabled targets.
    let targets = cfg.targets
        .iter()
        .filter(|t| t.enabled);

    for target in targets {
        // Clone and checkout target repo / branch.
        let ctx = git::clone(&target.repository, repository_path, &args.git_user, &args.git_email, args.git_password.as_deref())?;
        ctx.checkout(&target.branch)?;

        // Apply all rules.
        for rule in target.rules.iter() {
            let absolute_paths = glob_files(&format!("{}/{}", repository_str, rule.file_pattern))?;

            // Strip repository path to get just relative paths.
            let relative_paths = absolute_paths
                .iter()
                .map(|path| path.strip_prefix(repository_path))
                .collect::<Result<Vec<_>, _>>()?;

            // Find the first correct value for variable in the inputs.
            let value = args.values
                .iter()
                .find(|(key, _)| *key == rule.variable)
                .map(|(_, value)| value)
                .ok_or("variable not found in inputs")?;

            match &rule.pattern {
                Pattern::Hcl { block, labels, attribute } => {
                    for path in &absolute_paths {
                        hcl::update_file(&path, &block, &labels, &attribute, &value)?;
                    }
                },
            }

            // Add and commit updated file.
            ctx.add_and_commit(&relative_paths, &format!("Bump {} to {}.", rule.variable, value))?;
        }

        // Generate a unique branch name.
        let origin_branch = format!("auto-promote-{}", Uuid::new_v4().to_string());

        // Push changes to new branch on origin.
        ctx.push_head(&origin_branch)?;

        // Create and merge PR.
        pr::merge_pull_request(&target.repository, &origin_branch, &target.branch, "Auto Promotion", "Auto Promotion", args.git_password.as_deref()).await?;

        // Remove local repository.
        ctx.cleanup()?;
    }

    Ok(())
}